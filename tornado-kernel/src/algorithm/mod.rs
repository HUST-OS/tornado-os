pub use allocator::{Allocator, StackedAllocator};

mod allocator {
    mod stacked_allocator;

    pub use stacked_allocator::StackedAllocator;

    /// 帧分配器
    pub trait Allocator {
        /// 分配一个元素，无法分配则返回 `None`
        fn alloc(&mut self) -> Option<usize>;
        /// 回收一个元素
        fn dealloc(&mut self, index: usize);
    }
}
