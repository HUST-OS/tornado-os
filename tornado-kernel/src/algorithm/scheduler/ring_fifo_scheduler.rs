use super::Scheduler;
use core::mem::MaybeUninit;

/// 先进先出轮转任务调度器
pub struct RingFifoScheduler<T, const N: usize> {
    pool: [MaybeUninit<T>; N],
    front: usize,
    tail: usize,
    cur: usize,
}

impl<T, const N: usize> RingFifoScheduler<T, N> {
    /// 创建一个空的调度器
    pub fn new() -> Self {
        Self {
            pool: MaybeUninit::uninit_array(),
            front: 0,
            tail: 0,
            cur: 0,
        }
    }
    const fn is_full(&self) -> bool {
        (self.tail + 1) % N == self.front
    }
    const fn is_empty(&self) -> bool {
        self.tail == self.front
    }
    const fn next_idx(idx: usize) -> usize {
        (idx + 1) % N
    }
}

impl<T: Clone + PartialEq, const N: usize> Scheduler<T> for RingFifoScheduler<T, N> {
    type Priority = ();
    fn add_task(&mut self, task: T) {
        // 加入环形队列
        if self.is_full() { // 如果满了，退出（应该要返回队列满了）
            panic!()
        }
        // 写进去
        unsafe { *self.pool[self.tail].as_mut_ptr() = task };
        // 改变下标
        self.tail = (self.tail + 1) % N;
    }
    fn next_task(&mut self) -> Option<T> {
        // 从头部取出放回尾部，同时将其返回
        if self.is_empty() {
            return None;
        }
        todo!()
    }
    fn peek_next_task(&self) -> Option<&T> {
        // 拿出头部的引用
        if self.is_empty() {
            None
        } else {
            Some(unsafe { &*self.pool[self.front].as_ptr() })
        }
    }
    fn remove_task(&mut self, task: &T) {
        // 移除相应的线程并且确认恰移除一个线程
        drop(task);
        todo!()
    }
    fn set_priority(&mut self, _task: T, _prio: ()) {}
}
