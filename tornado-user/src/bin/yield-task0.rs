#![no_std]
#![no_main]
#![feature(asm)]
#![feature(llvm_asm)]

extern crate alloc;
#[macro_use]
extern crate tornado_user;

use tornado_user::do_yield;
async fn async_main() {
    println!("yield test task 0");
}

// 异步main函数，由entry调用execute_async_main
#[no_mangle]
fn main() -> i32 {
    // 往共享调度器中添加任务
    tornado_user::spawn(async_main());
    // 切换地址空间
    do_yield(2);
    println!("yield back");
    tornado_user::execute_async();
    0
}