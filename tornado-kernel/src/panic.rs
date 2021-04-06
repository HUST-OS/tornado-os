use core::panic::PanicInfo;

#[cfg_attr(not(test), panic_handler)]

fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    crate::sbi::shutdown()
}
