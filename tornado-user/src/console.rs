use super::test_write;
use alloc::string::String;
use alloc::sync::Arc;
use core::fmt::{self, Write};

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        test_write(s.as_bytes());
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

lazy_static::lazy_static! {
    static ref STDIN_LOCK: Arc<spin::Mutex<()>> = Arc::new(spin::Mutex::new(()));
}

// 用0号调试接口实现的Stdin
pub struct Stdin(Arc<spin::Mutex<()>>);

pub fn stdin() -> Stdin {
    Stdin(Arc::clone(&*STDIN_LOCK))
}

impl Stdin {
    // 锁上当前的Stdin
    // pub fn lock(&self) -> StdinLock { ... }

    // 从测试接口读一行
    // pub fn read_line(&self, buf: &mut String) -> Result<usize> {
    pub fn read_line(&self, buf: &mut String) -> usize {
        const CAPACITY: usize = 1024; // 目前的内核最长读1024字符，后面都切断，未来修改
        buf.reserve(CAPACITY);
        let buf_input = unsafe { core::slice::from_raw_parts_mut(buf.as_mut_ptr(), CAPACITY) };
        let syscall_ans = crate::syscall::sys_test_read_line(buf_input);
        let bytes_read = syscall_ans.extra;
        // buf.shrink_to(bytes_read); // 与API风格有关，不用缩缓冲区
        bytes_read
    }
}
