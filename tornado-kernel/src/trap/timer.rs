use crate::sbi::set_timer;
use riscv::register::{sie, time};

const MSEC_PER_SEC: usize = 1000;

#[cfg(feature = "qemu")]
const CLOCK_FREQ: usize = 12500000;

#[cfg(feature = "k210")]
const CLOCK_FREQ: usize = 403000000 / 62;

/// 初始化时钟中断
pub fn init() {
    unsafe {
        sie::set_stimer(); // 允许时钟中断
                           // sie::clear_stimer(); // 关闭时钟中断
    }
    set_next_timeout(); // 设置下一次时钟中断
}

static INTERVAL: usize = 100000;

/// 设置下一次时钟中断
fn set_next_timeout() {
    set_timer(time::read() + INTERVAL);
}

pub static mut TICKS: usize = 0;

pub fn tick() {
    set_next_timeout();
    unsafe {
        TICKS = TICKS.wrapping_add(1);
    }
}

pub fn get_time_ms() -> usize {
    time::read() / (CLOCK_FREQ / MSEC_PER_SEC)
}