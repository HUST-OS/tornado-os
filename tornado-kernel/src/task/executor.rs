use crate::task::{KernelTaskRepr, TaskResult, TaskState};
use alloc::sync::Arc;
use core::{
    mem,
    task::{Context, Poll},
};
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
        // unsafe { riscv::register::sie::clear_sext(); }
        let task = peek_task();
        // unsafe { riscv::register::sie::set_sext(); }
        println!(">>> kernel executor: next task = {:x?}", task);
        match task {
            TaskResult::Task(task_repr) => { // 在相同的（内核）地址空间里面
                // unsafe { riscv::register::sie::clear_sext(); }
                set_task_state(task_repr, TaskState::Sleeping);
                // unsafe { riscv::register::sie::set_sext(); }
                let task: Arc<KernelTaskRepr> = unsafe { Arc::from_raw(task_repr as *mut _) };
                // 注册 waker
                let waker = waker_ref(&task);
                let mut context = Context::from_waker(&*waker);
                let ret = task.task().future.lock().as_mut().poll(&mut context);
                if let Poll::Pending = ret {
                    mem::forget(task); // 不要释放task的内存，它将继续保存在内存中被使用
                } else { // 否则，释放task的内存
                    // unsafe { riscv::register::sie::clear_sext(); }
                    delete_task(task_repr);
                    // unsafe { riscv::register::sie::set_sext(); }
                } // 隐含一个drop(task)
            }
            TaskResult::ShouldYield(next_asid) => {
                todo!("切换到 next_asid (= {}) 对应的地址空间", next_asid)
            }
            TaskResult::NoWakeTask => {
                // todo!()
            },
            TaskResult::Finished => break
        }
    }
}

impl woke::Woke for KernelTaskRepr {
    fn wake_by_ref(task: &Arc<Self>) {
        unsafe { task.do_wake() }
    }
}
