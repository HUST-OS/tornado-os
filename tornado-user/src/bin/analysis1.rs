#![no_std]
#![no_main]
#![feature(asm)]
#![feature(llvm_asm)]

extern crate alloc;
#[macro_use]
extern crate tornado_user;


use tornado_user::{execute_async_main, do_yield};
async fn async_main(n: usize) -> i32 {
    for _ in 0..n {
        do_yield(2);
    }
    0
}

// 异步main函数，由entry调用execute_async_main
#[no_mangle]
fn main() -> i32 {
    execute_async_main(async_main(10))
}
