use super::Scheduler;
use alloc::collections::LinkedList;

/// 先进先出任务调度器
pub struct FifoScheduler<T> {
    pool: LinkedList<T>,
}

impl<T: Clone + PartialEq> FifoScheduler<T> {
    /// 创建一个空的调度器
    pub fn new() -> Self {
        Self {
            pool: LinkedList::new(),
        }
    }
}

impl<T: Clone + PartialEq> Scheduler<T> for FifoScheduler<T> {
    type Priority = ();
    fn add_task(&mut self, task: T) {
        // 加入链表尾部
        self.pool.push_back(task);
    }
    fn next_task(&mut self) -> Option<T> {
        // 从头部取出放回尾部，同时将其返回
        if let Some(task) = self.pool.pop_front() {
            self.pool.push_back(task.clone());
            Some(task)
        } else {
            None
        }
    }
    fn peek_next_task(&self) -> Option<&T> {
        // 拿出头部的引用
        self.pool.front()
    }
    fn remove_task(&mut self, task: &T) {
        // 移除相应的线程并且确认恰移除一个线程
        let mut removed = self.pool.drain_filter(|t| t == task);
        assert!(removed.next().is_some() && removed.next().is_none());
    }
    fn set_priority(&mut self, _task: T, _prio: ()) {}
}
