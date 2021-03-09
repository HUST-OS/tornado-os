use hashbrown::HashSet;
use crate::process::{Lock, TaskResult, SharedTaskHandle};
use crate::process::KernelTask;
use woke::{waker_ref, Woke};
use alloc::sync::Arc;
use core::{future, mem, task::{Poll, Context}};

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
            // println!("next task = {:x?}", task);
            if let TaskResult::Task(handle) = task {
                let task: Arc<KernelTask> = unsafe { Arc::from_raw(handle.task_ptr as *mut _) };
                if task.is_sleeping() {
                    mem::forget(task); // 不要释放内存
                    push_task(handle);
                    continue
                }
                mem::forget(task); // 不要释放内存
            }
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
                TaskResult::ShouldYield => todo!("转让到目的的地址空间中"),
                TaskResult::Finished => break
            }
        }
    }
}
