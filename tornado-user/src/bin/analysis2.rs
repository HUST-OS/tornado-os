#![no_std]
#![no_main]
#![feature(asm)]
#![feature(llvm_asm)]

extern crate alloc;
#[macro_use]
extern crate tornado_user;


use tornado_user::{do_yield, read_timer};


// 异步main函数，由entry调用execute_async_main
#[no_mangle]
fn main() -> i32 {
    for _ in 0..100 {
        do_yield(2);
    }
    println!("[analysis] timer: {}", read_timer());
    0
}
