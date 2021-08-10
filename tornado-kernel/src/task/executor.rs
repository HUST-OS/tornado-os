use super::{KernelTask, Process};
use crate::hart::KernelHartInfo;
use crate::syscall::get_swap_cx;
use crate::task::{KernelTaskRepr, TaskResult, TaskState};
use crate::trap::switch_to_user;
use alloc::{boxed::Box, sync::Arc};
use core::{
    mem,
    task::{Context, Poll},
};
use riscv::register::{sie, sstatus};
use woke::waker_ref;

/*
如果是当前上下文，就解释运行，如果不是，就切换上下文
切换上下文时，要把上下文保存好，最终还是要回到切换的地方继续运行。
*/
pub fn run_until_idle(
    peek_task: impl Fn() -> TaskResult,
    delete_task: impl Fn(usize) -> bool,
    set_task_state: impl Fn(usize, TaskState),
) {
    loop {
        unsafe {
            sstatus::set_sie();
        }
        ext_intr_off();
        let task = peek_task();
        ext_intr_on();
        println!(">>> kernel executor: next task = {:x?}", task);
        match task {
            TaskResult::Task(task_repr) => {
                // 在相同的（内核）地址空间里面
                ext_intr_off();
                set_task_state(task_repr, TaskState::Sleeping);
                ext_intr_on();
                let task: Arc<KernelTaskRepr> = unsafe { Arc::from_raw(task_repr as *mut _) };
                // 注册 waker
                let waker = waker_ref(&task);
                let mut context = Context::from_waker(&*waker);
                let ret = task.task().future.lock().as_mut().poll(&mut context);
                if let Poll::Pending = ret {
                    mem::forget(task); // 不要释放task的内存，它将继续保存在内存中被使用
                } else {
                    // 否则，释放task的内存
                    ext_intr_off();
                    delete_task(task_repr);
                    ext_intr_on();
                } // 隐含一个drop(task)
            }
            TaskResult::ShouldYield(next_asid) => {
                // 不释放这个任务的内存，执行切换地址空间的系统调用
                mem::forget(task);
                let next_satp = KernelHartInfo::user_satp(next_asid).expect("get satp with asid");
                let swap_cx = unsafe { get_swap_cx(&next_satp, next_asid) };
                switch_to_user(swap_cx, next_satp.inner(), next_asid)
            }
            TaskResult::NoWakeTask => {
                // todo!()
            }
            TaskResult::Finished => break,
        }
        unsafe {
            sstatus::clear_sie();
        }
    }
}

/// 用于内核第一次升到用户态
///
/// note: 需要确保共享调度器中只有一个任务
pub fn run_one(
    add_task: impl Fn(usize) -> bool,
    peek_task: impl Fn() -> TaskResult,
    delete_task: impl Fn(usize) -> bool,
    set_task_state: impl Fn(usize, TaskState),
) {
    loop {
        ext_intr_off();
        let task = peek_task();
        ext_intr_on();
        // println!(">>> run one: next task = {:x?}", task);
        match task {
            TaskResult::Task(task_repr) => {
                ext_intr_off();
                set_task_state(task_repr, TaskState::Sleeping);
                ext_intr_on();
                let task: Arc<KernelTaskRepr> = unsafe { Arc::from_raw(task_repr as *mut _) };
                // 注册 waker
                let waker = waker_ref(&task);
                let mut context = Context::from_waker(&*waker);
                // poll 操作之前在共享调度器中删除这个任务
                ext_intr_off();
                delete_task(task_repr);
                ext_intr_on();
                let ret = task.task().future.lock().as_mut().poll(&mut context);
                if let Poll::Pending = ret {
                    mem::forget(task); // 不要释放task的内存，它将继续保存在内存中被使用
                    ext_intr_off();
                    add_task(task_repr); // 重新把这个任务放进共享调度器
                    ext_intr_on();
                } else {
                    // 否则，释放task的内存
                    unreachable!() // 该任务不可能返回 Ready(T)
                }
            }
            TaskResult::NoWakeTask => {
                // todo!()
            }
            _ => unreachable!(),
        }
    }
}

impl woke::Woke for KernelTaskRepr {
    fn wake_by_ref(task: &Arc<Self>) {
        unsafe { task.do_wake() }
    }
}

/// 打开外部中断
pub fn ext_intr_on() {
    unsafe {
        sie::set_sext();
    }
}

/// 关闭外部中断
pub fn ext_intr_off() {
    unsafe {
        sie::clear_sext();
    }
}
