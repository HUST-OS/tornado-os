//! 协程内核异步任务

pub use self::shared::SharedTaskHandle;

mod shared;

/// 共享调度器返回的结果
/// 这个后面会考虑去掉，内核或用户与共享运行时通信的媒介只有 Rust 的基本类型数据
#[derive(Debug)]
#[repr(C)]
pub enum TaskResult {
    /// 应当立即执行特定任务
    Task(SharedTaskHandle),
    /// 其他地址空间的任务要运行，应当让出时间片
    ShouldYield(usize),
    /// 队列已空，所有任务已经结束
    Finished
}