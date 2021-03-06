use super::{Scheduler, ScheduledItem};
use alloc::collections::LinkedList;

/// 尽量调度相同地址空间的调度器
pub struct SameAddrSpaceScheduler<T, const N: usize> {
    tasks: LinkedList<T>,
    max_len: usize
}

impl<T, const N: usize> SameAddrSpaceScheduler<T, N> {
    #[allow(unused)]
    pub const fn new() -> Self {
        Self {
            tasks: LinkedList::new(),
            max_len: N
        }
    }
}

impl<T, const N: usize> Scheduler<T> for SameAddrSpaceScheduler<T, N>
    where T: ScheduledItem + Clone + PartialEq

{
    type Priority = ();
    /// 添加任务到调度队列  
    /// 如果队列已满，返回 Some(task)  
    fn add_task(&mut self, task: T) -> Option<T> {
        if self.tasks.len() < self.max_len {
            // 加入链表尾部
            self.tasks.push_back(task);
            None
        } else {
            Some(task)
        }
    }
    /// 取出下一个当前地址空间的任务  
    /// 如果当前地址空间的任务或者所有任务已经完成，返回 None
    fn next_task(&mut self) -> Option<T> {
        
        let mut count = 0;
        let len = self.tasks.len();
        while self.tasks.front().is_some() && count < len {
            // note(unwrap): 前面 self.tasks.front().is_some() 返回 Some
            let task = self.tasks.pop_front().unwrap();
            if task.need_switch() {
                self.tasks.push_back(task);
                count += 1;
            } else {
                return Some(task);
            }
        }
        None
    }
    /// 尝试获取下一个当前地址空间的任务的引用  
    /// 如果不存在，则返回链表头部的任务的引用  
    /// 如果所有任务已经完成，返回 None  
    fn peek_next_task(&self) -> Option<&T> {
        let mut iter = self.tasks.iter();
        loop {
            let task = iter.next();
            if task.is_some() {
                if !task.as_ref().unwrap().need_switch() {
                    return task;
                }
            } else {
                break;
            }
        }
        self.tasks.front()
    }
    fn remove_task(&mut self, task: &T) {
        // 移除相应的线程并且确认恰移除一个线程
        let mut removed = self.tasks.drain_filter(|t| t == task);
        assert!(removed.next().is_some() && removed.next().is_none());
    }
    fn set_priority(&mut self, _task: T, _prio: Self::Priority) {
        todo!()
    }
}

impl<T, const N: usize> IntoIterator for SameAddrSpaceScheduler<T, N> {
    type Item = T;
    type IntoIter = alloc::collections::linked_list::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.tasks.into_iter()
    }

}