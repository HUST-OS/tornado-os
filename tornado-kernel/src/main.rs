#![no_std]
#![no_main]
#![feature(global_asm, llvm_asm, alloc_error_handler)]
extern crate alloc;

#[macro_use]
mod console;
mod panic;
mod sbi;
mod interrupt;
mod memory;

#[cfg(not(test))]
global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    println!("booted");

    memory::init();
    interrupt::init();

    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    };

    // 动态内存分配测试
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    let v = Box::new(5);
    assert_eq!(*v, 5);
    core::mem::drop(v);

    let mut vec = Vec::new();
    for i in 0..10000 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10000);
    for (i, value) in vec.into_iter().enumerate() {
        assert_eq!(value, i);
    }
    println!("heap test passed");
    loop {}
    // sbi::shutdown()
}
