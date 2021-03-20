use crate::task::{TaskResult, SharedTaskHandle, KernelTask};
use woke::waker_ref;
use alloc::sync::Arc;
use core::{mem, task::{Poll, Context}};

/*
如果是当前上下文，就解释运行，如果不是，就切换上下文
切换上下文时，要把上下文保存好，最终还是要回到切换的地方继续运行。
*/

pub fn run_until_idle<F, G>(pop_task: F, push_task: G)
where
    F: Fn() -> TaskResult,
    G: Fn(SharedTaskHandle) -> Option<SharedTaskHandle> 
{
    loop {
        let task = pop_task();
        if let TaskResult::Task(handle) = task {
            let task: Arc<KernelTask> = unsafe { Arc::from_raw(handle.task_ptr as *mut _) };
            if task.is_sleeping() {
                mem::forget(task); // 不要释放内存
                push_task(handle);
                continue
            }
            mem::forget(task); // 不要释放内存
        }
        println!(">>> next task = {:x?}", task);
        match task {
            TaskResult::Task(handle) => {
                // 在相同的（内核）地址空间里面
                let task: Arc<KernelTask> = unsafe { Arc::from_raw(handle.task_ptr as *mut _) };
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
                    push_task(handle); 
                } // 否则，释放task的内存。这里相当于drop(task)
            },
            TaskResult::ShouldYield => {
                //todo
                crate::trap::switch_to_user()
            },
            TaskResult::Finished => break
        }
    }
}
