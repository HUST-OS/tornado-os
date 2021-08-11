use crate::{
    algorithm::{Allocator, StackedAllocator},
    memory::{
        config::{FREE_MEMORY_START, MEMORY_END_ADDRESS},
        frame::FrameTracker,
        PhysicalPageNumber,
    },
};
use core::ops::Range;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    /// 全局帧分配器
    pub static ref FRAME_ALLOCATOR: Mutex<FrameAllocator<StackedAllocator>> = {
        let pa_start = FREE_MEMORY_START.physical_address_linear();
        let ppn_start = PhysicalPageNumber::ceil(pa_start);
        let ppn_end = PhysicalPageNumber::floor(MEMORY_END_ADDRESS);
        return Mutex::new(FrameAllocator::new(
            ppn_start..ppn_end,
            StackedAllocator::new(ppn_end - ppn_start),
        ))
    };
}

/// 帧分配器
pub struct FrameAllocator<A> {
    allocator: A,
    start_ppn: PhysicalPageNumber,
}

impl<A: Allocator> FrameAllocator<A> {
    /// 创建一个帧分配器
    pub fn new(range: Range<PhysicalPageNumber>, allocator: A) -> Self {
        FrameAllocator {
            start_ppn: range.start,
            allocator,
        }
    }
    /// 申请一个帧
    pub fn alloc(&mut self) -> Option<FrameTracker> {
        self.allocator
            .alloc()
            .map(|idx| FrameTracker(self.start_ppn + idx))
    }
}

impl<A: Allocator> FrameAllocator<A> {
    // only be called in FrameTracker::drop
    pub(in crate::memory) fn dealloc(&mut self, frame: &FrameTracker) {
        self.allocator.dealloc(frame.page_number() - self.start_ppn)
    }
}
