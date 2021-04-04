#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(panic_info_message)]

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    loop {}
}

#[cfg_attr(not(test), panic_handler)]
fn panic_handler(_panic_info: &core::panic::PanicInfo) -> ! {
    unsafe { llvm_asm!("ecall"); }
    unreachable!()
}