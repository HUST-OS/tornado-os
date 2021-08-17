//! 对比rCore-Tutorial-v3任务切换性能测试
#![no_std]
#![no_main]
#![feature(asm)]
#![feature(llvm_asm)]

extern crate alloc;
#[macro_use]
extern crate tornado_user;

use tornado_user::{do_yield, execute_async, read_timer, spawn};

async fn a(_x: usize) {}

// 异步main函数，由entry调用execute_async_main
#[no_mangle]
fn main() -> i32 {
    for i in 0..200 {
        spawn(a(i));
    }
    let start = read_timer();
    execute_async();
    let end = read_timer();
    println!("[analysis] tornado-os time: {}", end - start);
    do_yield(2);
    0
}
