use crate::task::{TaskResult, KernelTask};
use woke::waker_ref;
use alloc::sync::Arc;
use core::{mem, task::{Poll, Context}};

/*
如果是当前上下文，就解释运行，如果不是，就切换上下文
切换上下文时，要把上下文保存好，最终还是要回到切换的地方继续运行。
*/

pub fn run_until_idle(
    peek_task: impl Fn() -> TaskResult,
    delete_task: impl Fn(usize) -> bool,
) {
    loop {
        let task = peek_task();
        println!(">>> kernel executor: next task = {:x?}", task);
        match task {
            TaskResult::Task(task_repr) => {
                // 在相同的（内核）地址空间里面
                let task: Arc<KernelTask> = unsafe { Arc::from_raw(task_repr as *mut _) };
                task.mark_sleep();
                // make a waker for our task
                let waker = waker_ref(&task);
                // poll our future and give it a waker
                let mut context = Context::from_waker(&*waker);
                // println!("Poll begin");
                let ret = task.future.lock().as_mut().poll(&mut context);
                // println!("Ret = {:?}", ret);
                if let Poll::Pending = ret {
                    mem::forget(task); // 不要释放task的内存，它将继续保存在内存中被使用
                } else { // 否则，释放task的内存
                    delete_task(task_repr);
                    // drop(task)
                }
            },
            TaskResult::ShouldYield(next_asid) => {
                todo!("切换到 next_asid (= {}) 对应的地址空间", next_asid)
            },
            TaskResult::Finished => break
        }
    }
}
