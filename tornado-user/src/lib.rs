#![no_std]
#![feature(llvm_asm)]
#![feature(asm)]
#![feature(panic_info_message)]
#![feature(linkage)]
#![feature(alloc_error_handler)]

extern crate alloc;

pub mod excutor;
pub mod shared;
pub mod task;

use buddy_system_allocator::LockedHeap;


const USER_HEAP_SIZE: usize = 32768;

static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

#[cfg_attr(not(test), panic_handler)]
pub fn panic_handler(_panic_info: &core::panic::PanicInfo) -> ! {
    // todo: 直接传给系统调用
    unsafe { llvm_asm!("ebreak"); }
    unreachable!()
}

#[cfg_attr(not(test), alloc_error_handler)]
pub fn handle_alloc_error(_layout: core::alloc::Layout) -> ! {
    // todo: 直接传给系统调用
    unsafe { llvm_asm!("ebreak"); }
    unreachable!()
}

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    extern "C" {
        fn sbss(); fn ebss();
    } 
    unsafe { r0::zero_bss(&mut sbss as *mut _ as *mut u64, &mut ebss as *mut _ as *mut u64) };
    unsafe {
        HEAP.lock().init(HEAP_SPACE.as_ptr() as usize, USER_HEAP_SIZE);
    }
    main()
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> ! {
    panic!("Can not find main!");
}