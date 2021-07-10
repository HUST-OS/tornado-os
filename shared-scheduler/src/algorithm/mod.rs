//! 调度算法这这里实现

mod ring_fifo;
pub use ring_fifo::RingFifoScheduler;

/// 调度器实例需要实现的 Trait
pub trait Scheduler<T: Clone + PartialEq> {
    /// 优先级的类型
    type Priority;
    /// 向调度器中添加一个任务，成功返回 None，不成功返回 Some(T)
    fn add_task(&mut self, task: T) -> Option<T>;
    /// 获取下一个任务的不可变引用，但不弹出任务
    fn peek_next_task(&self) -> Option<&T>;
    /// 获取下一个任务的可变引用，但不弹出任务
    fn peek_next_task_mut(&mut self) -> Option<&mut T>;
    /// 弹出下一个时间段应当执行的任务
    fn next_task(&mut self) -> Option<T>;
    /// 获取正在运行的任务，中断发生时，将保存这个任务的上下文
    fn current_task(&self) -> Option<T>;
    /// 移除一个任务
    fn remove_task(&mut self, task: &T);
    /// 设置任务的优先级
    fn set_priority(&mut self, task: T, priority: Self::Priority);
    /// 返回调度队列的总长度
    /// 如果有队列，则返回 Some，否则返回 None
    ///
    /// 目前这里是实现上的需要，后面需要移除这个方法
    /// 因为具体调度器实现里面可能没有队列，这个不是调度器的共性
    fn queue_len(&self) -> Option<usize> {
        None
    }
}
