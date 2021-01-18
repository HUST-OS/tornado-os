#![no_std]
#![no_main]
#![feature(global_asm, llvm_asm)]

#[macro_use]
mod console;
mod panic;
mod sbi;

#[cfg(not(test))]
global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    println!("OK");
    sbi::shutdown()
}
