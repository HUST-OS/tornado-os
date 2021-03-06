use hashbrown::HashSet;
use crate::process::{Lock, TaskResult, Task, SharedTaskHandle};
use woke::{waker_ref, Woke};
use alloc::sync::Arc;
use core::{future, task::{Poll, Context}};

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
            let task = pop_task();
            println!("next task = {:x?}", task);
            match task {
                TaskResult::Task(handle) => {
                    // 在相同的地址空间里面
                    let task: Arc<Task> = unsafe { Arc::from_raw(handle.task_ptr as *mut _) };
                    task.mark_sleep();

                    // make a waker for our task
                    let waker = waker_ref(&task);

                    // poll our future and give it a waker
                    let mut context = Context::from_waker(&*waker);
                    let ret = task.future.lock().as_mut().poll(&mut context);
                    println!("Ret = {:?}", ret);
                    if let Poll::Pending = ret {
                        push_task(handle);
                    }
                },
                TaskResult::ShouldYield => todo!("转让到目的的地址空间中"),
                TaskResult::Finished => break
            }
        }
    }

    pub fn block_on<F: Fn() -> TaskResult>(pop_task: F) {
        let task = pop_task();
        match task {
            TaskResult::Task(handle) => {
                // 在相同的地址空间里面，不用切换上下文
                let task: Arc<Task> = unsafe { Arc::from_raw(handle.task_ptr as *mut _) };
                // 下面这个步骤应该可以不做
                task.mark_sleep();

                // create a waker for the task
                let waker = waker_ref(&task);

                // poll the future and give it a waker
                let mut context = Context::from_waker(&*waker);
                loop {
                    let mut future = task.future.lock();
                    if let Poll::Ready(_) = future.as_mut().poll(&mut context) {
                        break;
                    }
                }
            },
            TaskResult::ShouldYield => {
                // 地址空间改变，应该切换上下文
                todo!("切换到目的地址空间")
            },
            TaskResult::Finished => {} // do nothing
        }
    }
}
