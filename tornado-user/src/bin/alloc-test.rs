#![no_std]
#![no_main]
#![feature(asm)]
#![feature(llvm_asm)]

extern crate alloc;
#[macro_use]
extern crate tornado_user;
use alloc::vec;

// 同步函数的例子，没有调用execute_async_main
#[no_mangle]
fn main() -> i32 {
    println!("[user] enter main!");
    let mut test_v = vec![1, 2, 3, 4, 5];
    test_v.iter_mut().for_each(|x| *x += 1);
    assert_eq!(test_v, vec![2, 3, 4, 5, 6]);
    println!("[User] alloc-test: success!");
    0
}
