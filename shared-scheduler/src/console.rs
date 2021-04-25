use crate::syscall;
use core::fmt::{self, Write};

struct Stdout;

// 暂时不用关中断的锁lock::Lock，考虑多个硬件线程的情况
static STDOUT_LOCK: spin::Mutex<()> = spin::Mutex::new(());

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        STDOUT_LOCK.lock();
        syscall::sys_test_write(s.as_bytes());
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
