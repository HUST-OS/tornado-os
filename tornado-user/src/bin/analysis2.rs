#![no_std]
#![no_main]
#![feature(asm)]
#![feature(llvm_asm)]

extern crate alloc;
#[macro_use]
extern crate tornado_user;


use tornado_user::{ADDRESS_SPACE_ID, do_yield, execute_async, spawn};
async unsafe fn analysis_task(n: usize) {
    // println!("[analysis] task {} in address space {}", n, ADDRESS_SPACE_ID);
}

// 异步main函数，由entry调用execute_async_main
#[no_mangle]
fn main() -> i32 {
    unsafe {
        for i in 0..100 {
            spawn(analysis_task(i));
            do_yield(1);
        }
    }
    execute_async();
    0
}
