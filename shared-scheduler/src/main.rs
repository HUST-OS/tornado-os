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
#[link_section = ".meta"] // 虚函数表只读
#[no_mangle]
pub static SHARED_RAW_TABLE: (
    &'static u8, // 载荷编译时的基地址
    unsafe extern "C" fn() -> PageList, // 初始化函数，执行完之后，内核将函数指针置空
    &'static SharedScheduler, // 共享调度器的地址
    unsafe extern "C" fn(NonNull<()>, SharedTaskHandle) -> FfiOption<SharedTaskHandle>,
    unsafe extern "C" fn(NonNull<()>, extern "C" fn(&SharedTaskHandle) -> bool) -> TaskResult,
) = (
    unsafe { &payload_compiled_start },
    init_payload_environment,
    &SHARED_SCHEDULER,
    shared_add_task,
    shared_pop_task,    
);

#[allow(non_upper_case_globals)]
extern "C" {
    static payload_compiled_start: u8;
    static srodata_page: u32; static erodata_page: u32;
    static sdata_page: u32; static edata_page: u32;
    static stext_page: u32; static etext_page: u32;
    static mut sbss: u32; static mut ebss: u32;
}

/// 初始化载荷环境，只能由内核运行，只能运行一次
unsafe extern "C" fn init_payload_environment() -> PageList {
    r0::zero_bss(&mut sbss, &mut ebss);
    PageList {
        rodata: [&srodata_page, &erodata_page],
        data: [&sdata_page, &edata_page],
        text: [&stext_page, &etext_page],
    }
}

/// 共享载荷各个段的范围，方便内存管理的权限设置
///
/// 有虚拟内存，用特殊的链接器脚本，以确保对齐到4K，如果没有虚拟内存，可以使用更低的对齐方法
#[repr(C)]
pub struct PageList {
    // 这里的&'static u32指向的值并不重要，它表示的地址比较重要
    rodata: [&'static u32; 2], // 只读数据段
    data: [&'static u32; 2], // 数据段
    text: [&'static u32; 2], // 代码段
}
