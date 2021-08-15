#![no_std]
#![no_main]
#![feature(asm)]
#![feature(llvm_asm)]

extern crate alloc;
#[macro_use]
extern crate tornado_user;


use tornado_user::{execute_async, spawn, reset_timer, read_timer};

async fn a(_x: usize) {
    // println!("[analysis] task: {}", x);
}

// 异步main函数，由entry调用execute_async_main
#[no_mangle]
fn main() -> i32 {
    for i in 0..100 {
        spawn(a(i));
    }
    println!("[analysis] timer: {}", read_timer());
    execute_async();
    0
}
