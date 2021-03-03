use super::Scheduler;
use core::mem::MaybeUninit;
use core::ptr;

/// 先进先出轮转任务调度器
pub struct RingFifoScheduler<T, const N: usize> {
    ring: RingQueue<T, N>,
}

impl<T, const N: usize> RingFifoScheduler<T, N> {
    /// 创建一个空的调度器
    pub const fn new() -> Self {
        Self {
            ring: RingQueue::new(),
        }
    }
}

impl<T: Clone + PartialEq, const N: usize> Scheduler<T> for RingFifoScheduler<T, N> {
    type Priority = ();
    fn add_task(&mut self, task: T) {
        // 加入环形队列
        let ans = self.ring.push_back(task);
        if let Some(_t) = ans { // 如果满了，退出（应该要返回队列满了）
            panic!()
        }
    }
    fn next_task(&mut self) -> Option<T> {
        // 从头部取出放回尾部，同时将其返回
        if let Some(task) = self.ring.pop_front() {
            self.ring.push_back(task.clone());
            Some(task)
        } else {
            None
        }
    }
    fn peek_next_task(&self) -> Option<&T> {
        // 拿出头部的引用
        self.ring.front()
    }
    fn remove_task(&mut self, task: &T) {
        // 移除相应的线程并且确认恰移除一个线程
        drop(task);
        todo!("还没做")
    }
    fn set_priority(&mut self, _task: T, _prio: ()) {}
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
        if self.tail > N || self.tail == 0 {
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
        if self.front > N || self.front == 0 {
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
}
