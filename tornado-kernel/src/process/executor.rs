use hashbrown::HashSet;
use crate::process::{Lock, Task};
use alloc::sync::Arc;

/// 运行任务的处理器
pub struct Executor {
    /// 当前正在执行的任务
    current_task: Option<Arc<Task>>,
    /// 休眠的任务
    sleeping_tasks: HashSet<Arc<Task>>,
} 

lazy_static::lazy_static! {
    /// 全局的处理器
    pub static ref EXECUTOR: Lock<Executor> = Lock::new(Executor {
        current_task: None,
        sleeping_tasks: HashSet::new()
    });
}
