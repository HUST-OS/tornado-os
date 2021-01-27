use hashbrown::HashSet;
use alloc::sync::Arc;
use crate::{algorithm::FifoScheduler, process::Task};

/// 运行任务的处理器
pub struct Processor<S> {
    /// 当前正在执行的任务
    current_task: Option<Arc<Task>>,
    /// 本处理器的共享任务调度器
    scheduler: S,
    /// 休眠的任务
    sleeping_tasks: HashSet<Arc<Task>>,
}

lazy_static::lazy_static! {
    pub static ref PROCESSOR: Processor<FifoScheduler<Arc<Task>>> = Processor {
        current_task: None,
        scheduler: FifoScheduler::new(),
        sleeping_tasks: HashSet::new()
    };
}
