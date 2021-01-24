use crate::memory::{PhysicalAddress, PhysicalPageNumber, config::PAGE_SIZE};
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

/// 释放的时候，将释放所持有的帧内存
impl Drop for FrameTracker {
    fn drop(&mut self) {
        FRAME_ALLOCATOR.lock().dealloc(self);
    }
}


/// `FrameTracker` 可以 deref 得到对应的 `[u8; PAGE_SIZE]`
impl core::ops::Deref for FrameTracker {
    type Target = [u8; PAGE_SIZE];
    fn deref(&self) -> &Self::Target {
        unsafe { self.page_number().start_address().deref_linear_static() }
    }
}

/// `FrameTracker` 可以 deref 得到对应的 `[u8; PAGE_SIZE]`
impl core::ops::DerefMut for FrameTracker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.page_number().start_address().deref_linear_static() }
    }
}
