//! 为协程内核设计的共享调度器运行时
//! 
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
use algorithm::{RingFifoScheduler, Scheduler};

const USER_HEAP_SIZE: usize = 32768;

static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

#[cfg_attr(not(test), panic_handler)]
pub fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    println!("[shared-rt] panic: {:?}", panic_info);
    sbi::shutdown()    
}

#[cfg_attr(not(test), alloc_error_handler)]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    println!("[shared-rt] alloc panic: {:?}", layout);
    sbi::shutdown()
}

#[link_section = ".text.entry"]
#[export_name = "_start"]
#[naked]
unsafe extern "C" fn start() -> ! {
    asm!(
            "
    la sp, boot_stack_top
    call _shared_main

_start_abort:
    wfi
    j _start_abort
    .section .bss.stack
    .global boot_stack
boot_stack:
    .space 4096 * 4
    .global boot_stack_top
boot_stack_top:
    
    .section .data
    ", options(noreturn))
}

#[export_name = "_shared_main"]
extern "C" fn shared_main() -> ! {
    extern "C" {
        static mut _sbss: u32;
        static mut _ebss: u32;

        static mut _sdata: u32;
        static mut _edata: u32;

        static _sidata: u32;

        fn _shared_raw_table();
        fn hello_world();
    }
    unsafe { 
        r0::zero_bss(&mut _sbss, &mut _ebss);
        r0::init_data(&mut _sdata, &mut _edata, &_sidata);
    }
    unsafe {
        HEAP.lock().init(HEAP_SPACE.as_ptr() as usize, USER_HEAP_SIZE);
    }
    println!("[shared-rt] enter shared-rt!");
    println!("[shared-rt] _shared_raw_table: {:#x}", _shared_raw_table as usize);
    // 堆分配测试
    let mut v = alloc::vec::Vec::new();
    for i in 0..5 {
        v.push(i);
    }
    v.iter_mut().for_each(|x| *x += 1);
    assert_eq!(v, alloc::vec![1, 2, 3, 4, 5]);
    
    // 调度算法测试
    let mut ring_fifo: RingFifoScheduler<usize, 50> = RingFifoScheduler::new();
    ring_fifo.add_task(0);
    ring_fifo.add_task(1);
    assert_eq!(ring_fifo.next_task(), Some(0));
    assert_eq!(ring_fifo.next_task(), Some(1));

    // 通过裸指针调用函数测试
    let f_ptr = _shared_raw_table as usize as *const ();
    let f_code: fn(a0: usize) -> usize = unsafe { core::mem::transmute(f_ptr) };
    assert_eq!(f_code(0), hello_world as usize);

    println!("[shared-rt] entering kernel...");
    unsafe { enter_kernel() }
}

// 跳转到内核
#[naked]
#[link_section = ".text"]
unsafe extern "C" fn enter_kernel() -> ! {
    asm!("
1:  auipc ra, %pcrel_hi(1f)
    ld ra, %pcrel_lo(1b)(ra)
    jr ra
.align  3
1:  .dword 0x80400000
    ", options(noreturn))
}
