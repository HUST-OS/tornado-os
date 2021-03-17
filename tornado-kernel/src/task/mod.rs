mod kernel_task;
mod process;
mod executor;
mod lock;
mod shared;
mod switch;
mod user_task;

pub use lock::Lock;
pub use kernel_task::{KernelTask, TaskId};
pub use process::{Process, ProcessId};
pub use executor::run_until_idle;
pub use shared::{
    SHARED_SCHEDULER, SHARED_RAW_TABLE,
    SharedTaskHandle, shared_scheduler,
    current_task, shared_add_task,
    shared_pop_task
};
pub use switch::ContextTable;

/// 共享调度器返回的结果
#[derive(Debug)]
pub enum TaskResult {
    /// 应当立即执行特定任务
    Task(SharedTaskHandle),
    /// 其它地址空间的任务要运行，应当让出时间片
    ShouldYield,
    /// 队列已空，所有任务已经结束
    Finished,
}
