mod allocator;
mod tracker;

pub use tracker::FrameTracker;

pub fn frame_alloc() -> Option<FrameTracker> {
    allocator::FRAME_ALLOCATOR.lock().alloc()
}
