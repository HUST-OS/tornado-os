use core::panic::PanicInfo;

#[cfg_attr(not(test), panic_handler)]
#[allow(unused)]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    crate::sbi::shutdown()
}
