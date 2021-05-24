//! 循环先进先出队列调度器实现
//! 

use super::Scheduler;
use core::mem::MaybeUninit;
use core::ptr;

/// 先进先出轮转任务调度器
pub struct RingFifoScheduler<T, const N: usize> {
    ring: RingQueue<T, N>,
    current: Option<T>,
}

impl<T, const N: usize> RingFifoScheduler<T, N> {
    /// 创建一个空的调度器
    pub const fn new() -> Self {
        Self {
            ring: RingQueue::new(),
            current: None,
        }
    }
}

impl<T: Clone + PartialEq, const N: usize> Scheduler<T> for RingFifoScheduler<T, N> {
    type Priority = ();
    fn add_task(&mut self, task: T) -> Option<T> {
        // 加入环形队列
        let ans = self.ring.push_back(task);
        // 直接返回self.ring.push_back也可以
        if let Some(t) = ans { // 如果满了，退出
            return Some(t)
        }
        None
    }
    fn next_task(&mut self) -> Option<T> {
        // 从头部取出
        let ans = self.ring.pop_front();
        self.current = ans.clone();
        ans
    }
    fn peek_next_task(&self) -> Option<&T> {
        // 拿出头部的引用
        self.ring.front()
    }
    fn peek_next_task_mut(&mut self)  -> Option<&mut T> {
        self.ring.front_mut()
    }
    fn current_task(&self) -> Option<T> {
        self.current.clone()
    }
    fn remove_task(&mut self, task: &T) {
        // 移除相应的线程并且确认恰移除一个线程
        drop(task);
        todo!("还没做")
    }
    fn set_priority(&mut self, _task: T, _prio: ()) {}
    fn queue_len(&self) -> Option<usize> {
        Some(self.ring.len())
    }
}

pub struct RingQueue<T, const N: usize> {
    elem: [MaybeUninit<T>; N],
    front: usize,
    tail: usize
}

impl<T, const N: usize> RingQueue<T, N> {
    pub const fn new() -> Self {
        Self {
            elem: MaybeUninit::uninit_array(),
            front: 0,
            tail: 0,
        }
    }
    pub const fn len(&self) -> usize {
        self.tail.wrapping_sub(self.front) % N
    }
    pub const fn is_empty(&self) -> bool {
        self.tail == self.front
    }
    #[inline] fn is_full(&self) -> bool {
        self.len() == N - 1
    }
    // if push failed, value T is returned
    pub fn push_back(&mut self, value: T) -> Option<T> {
        if self.is_full() {
            return Some(value);
        }
        unsafe { *self.elem[self.tail].as_mut_ptr() = value };
        self.tail = self.tail.wrapping_add(1);
        // '>' -> '>='
        if self.tail >= N || self.tail == 0 {
            self.tail = self.tail.wrapping_sub(N);
        }
        None // success
    }
    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let value = unsafe { ptr::read(self.elem[self.front].as_ptr()) };
        self.front = self.front.wrapping_add(1); // assured non empty
        // '>' -> '>='
        if self.front >= N || self.front == 0 {
            self.front = self.front.wrapping_sub(N);
        }
        Some(value)
    }
    pub fn front(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(unsafe { &*self.elem[self.front].as_ptr() })
        }
    }
    pub fn front_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            None
        } else {
            Some(unsafe { &mut *self.elem[self.front].as_mut_ptr() })
        }
    }
}
