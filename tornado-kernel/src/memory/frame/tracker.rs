use crate::memory::{PhysicalAddress, PhysicalPageNumber};
use super::allocator::FRAME_ALLOCATOR;

/// 这个结构体就像一个Handle，Drop之后会释放PPN表示的物理帧
#[derive(Debug)]
pub struct FrameTracker(pub(super) PhysicalPageNumber);

impl FrameTracker {
    pub fn start_address(&self) -> PhysicalAddress {
        self.0.start_address()
    }

    pub fn page_number(&self) -> PhysicalPageNumber {
        self.0
    }
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        FRAME_ALLOCATOR.lock().dealloc(self);
    }
}
