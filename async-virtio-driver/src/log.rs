use crate::sbi::*;
use core::fmt::{self, Write};
use rv_lock::Lock;
struct Stdout;

// 暂时不用关中断的锁lock::Lock，考虑多个硬件线程的情况
static STDOUT_LOCK: Lock<()> = Lock::new(());

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut buffer = [0u8; 4];
        STDOUT_LOCK.lock();
        for c in s.chars() {
            for code_point in c.encode_utf8(&mut buffer).as_bytes().iter() {
                console_putchar(*code_point as usize);
            }
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::log::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::log::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
