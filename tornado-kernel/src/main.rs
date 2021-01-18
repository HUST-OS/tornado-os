#![no_std]
#![no_main]
#![feature(global_asm, llvm_asm)]

mod sbi;

#[cfg(not(test))]
global_asm!(include_str!("entry.asm"));

use core::panic::PanicInfo;

#[cfg_attr(not(test), panic_handler)]
#[allow(unused)]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    sbi::console_putchar(b'O' as usize);
    sbi::console_putchar(b'K' as usize);
    sbi::console_putchar(b'\n' as usize);
    sbi::shutdown()
}
