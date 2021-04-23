use alloc::collections::LinkedList;

pub struct FifoScheduler<T> {
    pool: LinkedList<T>, 
}

impl<T> FifoScheduler<T> {
    pub const fn new() -> Self {
        Self {
            pool: LinkedList::new(),
        }
    }
}

impl<T: PartialEq> Scheduler<T> for FifoScheduler<T> {
    fn insert(&mut self, task: T) {
        self.pool.push_back(task)
    }
    fn peek(&self)  -> Option<&T> {
        self.pool.front()
    }
    fn peek_mut(&mut self)  -> Option<&mut T> {
        self.pool.front_mut()
    }
    fn pop(&mut self) -> Option<T> {
        self.pool.pop_front()
    }
    fn remove(&mut self, task: &T) -> Option<T> {
        self.pool.drain_filter(|t| t == task).next()
    }
}

/// 调度器实例需要实现的 Trait
/// 
pub trait Scheduler<T> {
    /// 向调度器中添加一个任务
    fn insert(&mut self, task: T);
    /// 获取下一个任务的引用，但不弹出任务
    fn peek(&self)  -> Option<&T>;
    /// 获取下一个任务的可变引用，但不弹出任务
    fn peek_mut(&mut self)  -> Option<&mut T>;
    /// 弹出下一个任务
    fn pop(&mut self)  -> Option<T>;
    /// 移除一个任务 
    fn remove(&mut self, task: &T) -> Option<T>;
}
