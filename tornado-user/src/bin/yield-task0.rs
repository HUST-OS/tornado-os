#![no_std]
#![no_main]
#![feature(asm)]
#![feature(llvm_asm)]

extern crate alloc;
#[macro_use]
extern crate tornado_user;

use tornado_user::{do_yield, execute_async};
async fn async_main() {
    println!("[user] yield test task 0");
}

// 异步main函数，由entry调用execute_async_main
#[no_mangle]
fn main() -> i32 {
    // 往共享调度器中添加任务
    tornado_user::spawn(async_main());
    // 切换到地址空间2
    do_yield(2);
    println!("[user] yield back 1");
    execute_async();
    0
}
