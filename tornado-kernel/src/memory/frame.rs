//! 帧分配器
mod allocator;
mod tracker;

pub use tracker::FrameTracker;

use super::PhysicalPageNumber;

/// 分配一个物理页  
/// 如果已经分配完毕，返回 `None`
pub fn frame_alloc() -> Option<FrameTracker> {
    allocator::FRAME_ALLOCATOR.lock().alloc()
}
