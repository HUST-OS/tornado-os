//! 内核异步运行时实现
//!
//! 目前包含共享调度器实例化和内核执行器两个模块。
//!
//! Rust异步运行时是不包含在标准库里面的，交给社区贡献者实现，通常包含以下几个方面：
//!
//! * Future: 标准库提供`Future` trait，封装一个`poll`方法
//! * executor: `Future`的具体运行者
//! * reactor: `Future`的唤醒者
//!
//! 目前飓风内核里面的异步运行时主要是内核执行器，其配合共享调度器进行执行任务的工作。
//!
//! 在中断处理函数或者系统调用处理函数里面存在任务唤醒机制。
mod executor;
mod shared;

pub use executor::{ext_intr_off, ext_intr_on, run_one, run_until_idle};
pub use shared::{kernel_should_switch, SharedPayload, TaskState};
