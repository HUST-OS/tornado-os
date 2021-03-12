use super::{Scheduler, ScheduledItem};
use super::ring_fifo_scheduler::RingQueue;

/// 尽量调度相同地址空间的调度器
pub struct SameAddrSpaceScheduler<T, const N: usize> {
    tasks: RingQueue<T, N>,
    current: Option<T>,
}

impl<T, const N: usize> SameAddrSpaceScheduler<T, N> {
    #[allow(unused)]
    pub const fn new() -> Self {
        Self {
            tasks: RingQueue::new(),
            current: None,
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
        // 加入环形队列
        self.tasks.push_back(task)
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
                self.current = Some(task.clone()); // 保存到当前任务中
                return Some(task);
            }
        }
        self.current = None; // 没有相同地址空间的任务，设置当前任务为 None
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
    fn current_task(&self) -> Option<T> {
        self.current.clone()
    }
    fn remove_task(&mut self, task: &T) {
        // 移除相应的线程并且确认恰移除一个线程
        drop(task);
        todo!()
    }
    fn set_priority(&mut self, _task: T, _prio: Self::Priority) {
        todo!()
    }
}