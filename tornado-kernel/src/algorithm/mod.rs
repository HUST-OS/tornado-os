pub use allocator::{Allocator, StackedAllocator};
pub use scheduler::{Scheduler, ScheduledItem, FifoScheduler, RingFifoScheduler, SameAddrSpaceScheduler};

mod allocator {
    mod stacked_allocator;

    pub use stacked_allocator::StackedAllocator;

    /// 帧分配器
    pub trait Allocator {
        /// 分配一个元素，无法分配则返回 `None`
        fn alloc(&mut self) -> Option<usize>;
        /// 回收一个元素
        fn dealloc(&mut self, index: usize);
    }
}

mod scheduler {
    mod fifo_scheduler;
    mod ring_fifo_scheduler;
    mod same_addr_scheduler;
    pub use fifo_scheduler::FifoScheduler;
    pub use ring_fifo_scheduler::RingFifoScheduler;
    pub use same_addr_scheduler::SameAddrSpaceScheduler;

    pub trait Scheduler<T: ScheduledItem + Clone + PartialEq> {
        /// 优先级的类型
        type Priority;
        /// 向调度器中添加一个任务；成功返回None，如果不成功，返回T
        fn add_task(&mut self, task: T) -> Option<T>;
        /// 获取下一个任务的引用，但不弹出任务
        fn peek_next_task(&self) -> Option<&T>;
        /// 获取下一个时间段应当执行的任务
        fn next_task(&mut self) -> Option<T>;
        /// 移除一个任务
        fn remove_task(&mut self, task: &T);
        /// 设置任务的优先级
        fn set_priority(&mut self, task: T, prio: Self::Priority);
    }

    /// 被调度者需要实现的 `Trait`
    pub trait ScheduledItem {
        fn need_switch(&self) -> bool;
    }
}
