use super::Allocator;
use alloc::{vec, vec::Vec};
use core::ops::Range;

/// 栈分配器
pub struct StackedAllocator {
    list: Vec<Range<usize>>,
}

impl StackedAllocator {
    pub fn new(capacity: usize) -> Self {
        Self {
            list: vec![0..capacity],
        }
    }
}

impl Allocator for StackedAllocator {
    fn alloc(&mut self) -> Option<usize> {
        if let Some(range) = self.list.pop() {
            if range.end - range.start > 1 {
                self.list.push(range.start + 1..range.end);
            }
            Some(range.start)
        } else {
            None
        }
    }

    fn dealloc(&mut self, index: usize) {
        self.list.push(index..index + 1);
    }
}
