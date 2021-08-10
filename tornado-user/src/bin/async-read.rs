#![no_std]
#![no_main]
#![feature(asm)]
#![feature(llvm_asm)]

extern crate alloc;
#[macro_use]
extern crate tornado_user;

use tornado_user::{do_yield, execute_async, io::read_block, spawn};
async fn async_main() {
    println!("[user] start async read block");
    let mut buf = [0; 512];
    read_block(0, &mut buf).await;
    println!("[user] async read block ret: {:x?}", buf);
}

// 异步main函数，由entry调用execute_async_main
#[no_mangle]
fn main() -> i32 {
    spawn(async_main());
    do_yield(4);
    println!("[user] yield back 3");
    execute_async();
    0
}
