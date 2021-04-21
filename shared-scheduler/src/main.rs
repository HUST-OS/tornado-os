//! 为协程内核设计的共享调度器载荷
//! 以二进制包的形式编译

#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(llvm_asm)]
#![feature(asm)]
#![feature(maybe_uninit_uninit_array, maybe_uninit_ref)]
#![feature(naked_functions)]

extern crate alloc;

mod sbi;
#[macro_use]
mod console;
mod algorithm;
mod task;
mod mm;

use buddy_system_allocator::LockedHeap;

#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

#[cfg_attr(not(test), panic_handler)]
pub fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    println!("[shared scheduler] panic: {:?}", panic_info);
    sbi::shutdown()    
}

#[cfg_attr(not(test), alloc_error_handler)]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    println!("[shared scheduler] alloc panic: {:?}", layout);
    sbi::shutdown()
}
