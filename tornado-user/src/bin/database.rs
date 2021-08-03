#![no_std]
#![no_main]
#![feature(asm)]
#![feature(llvm_asm)]

extern crate alloc;
#[macro_use]
extern crate tornado_user;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

async fn async_main() -> i32 {
    let stdin = tornado_user::stdin();
    let mut buf = alloc::string::String::new();
    println!("欢迎使用数据库!");
    stdin.read_line(&mut buf);
    0
}

// 异步main函数，由entry调用execute_async_main
#[no_mangle]
fn main() -> i32 {
    tornado_user::execute_async_main(async_main())
}
