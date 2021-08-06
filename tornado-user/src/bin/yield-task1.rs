#![no_std]
#![no_main]
#![feature(asm)]
#![feature(llvm_asm)]

extern crate alloc;
#[macro_use]
extern crate tornado_user;

use tornado_user::{execute_async, do_yield, spawn};
async fn async_main() {
    println!("yield test task 1");
}

// 异步main函数，由entry调用execute_async_main
#[no_mangle]
fn main() -> i32 {
    spawn(async_main());
    do_yield(3);
    println!("yield back 2");
    execute_async();
    0
}
