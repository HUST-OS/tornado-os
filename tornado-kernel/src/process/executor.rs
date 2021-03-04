use hashbrown::HashSet;
use crate::process::{Lock, TaskResult, Task, SharedTaskHandle};
use woke::{waker_ref, Woke};
use alloc::sync::Arc;
use core::task::{Poll, Context};

/// 运行任务的处理器
#[derive(Default)]
pub struct Executor {
    // /// 当前正在执行的任务
    // current_task: Option<SharedTaskHandle>,
    // /// 休眠的任务
    // sleeping_tasks: HashSet<SharedTaskHandle>,
} 

// lazy_static::lazy_static! {
//     /// 全局的处理器
//     pub static ref EXECUTOR: Lock<Executor> = Lock::new(Executor {
//         current_task: None,
//         sleeping_tasks: HashSet::new()
//     });
// }

impl Executor {
    pub fn run_until_idle<F, G>(pop_task: F, push_task: G)
    where
        F: Fn() -> TaskResult,
        G: Fn(SharedTaskHandle) -> Option<SharedTaskHandle> 
    {
        loop {
            match pop_task() {
                TaskResult::Task(handle) => {
                    // 在相同的地址空间里面
                    let task: Arc<Task> = unsafe { Arc::from_raw(handle.task_ptr as *mut _) };
                    task.mark_sleep();

                    // make a waker for our task
                    let waker = waker_ref(&task);

                    // poll our future and give it a waker
                    let mut context = Context::from_waker(&*waker);
                    let ret = task.future.lock().as_mut().poll(&mut context);
                    if let Poll::Pending = ret {
                        push_task(handle);
                    }
                },
                TaskResult::ShouldYield => {}
                TaskResult::Finished => break
            }
        }
    }
}
