use alloc::alloc::Layout;
use super::config::KERNEL_HEAP_SIZE;
use buddy_system_allocator::LockedHeap;

static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

/// 全局的堆分配器
#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

#[allow(unused)]
#[cfg_attr(not(test), alloc_error_handler)]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("alloc error for layout {:?}", layout)
}

pub fn init() {
    unsafe {
        HEAP.lock().init(
            HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE
        )
    }
}
