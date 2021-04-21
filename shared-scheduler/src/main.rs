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
use core::ptr::NonNull;
use crate::task::{SharedTaskHandle, FfiOption, TaskResult, SharedScheduler, SHARED_SCHEDULER, shared_add_task, shared_pop_task};

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

/// 共享载荷虚函数表
#[link_section = ".data"]
#[no_mangle]
pub static SHARED_RAW_TABLE: (
    &'static u8, // 载荷编译时的基地址
    &'static SharedScheduler, // 共享调度器的地址
    unsafe extern "C" fn(NonNull<()>, SharedTaskHandle) -> FfiOption<SharedTaskHandle>,
    unsafe extern "C" fn(NonNull<()>, extern "C" fn(&SharedTaskHandle) -> bool) -> TaskResult,
) = (
    unsafe { 
        extern "C" {
            static payload_compiled_start: u8;
        }
        &payload_compiled_start  
    },
    &SHARED_SCHEDULER,
    shared_add_task,
    shared_pop_task,    
);
