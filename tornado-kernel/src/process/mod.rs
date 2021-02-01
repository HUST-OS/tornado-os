mod task;
mod process;
mod executor;
mod lock;

pub use lock::Lock;
pub use task::{Task, TaskId};
pub use process::{Process, ProcessInner};

use crate::algorithm::FifoScheduler;

/// 所有用户任务的调度器
#[link_section = ".shared_rw.data"]
pub static SHARED_SCHEDULER: spin::Mutex<FifoScheduler<Task>> = 
    spin::Mutex::new(FifoScheduler::new());

/// 共享的包含Future在用户空间的地址
pub struct SharedTaskHandle(pub usize);

#[link_section = ".shared_x.text"]
#[allow(unused)]
pub fn shared_add_task(handle: SharedTaskHandle) {}

/// 所有内核任务的调度器
pub static LOCAL_SCHEDULER: spin::Mutex<FifoScheduler<Task>> = 
    spin::Mutex::new(FifoScheduler::new());
