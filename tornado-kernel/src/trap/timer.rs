use crate::sbi::set_timer;
use riscv::register::{sie, time};

/// 初始化时钟中断
pub fn init() {
    unsafe {
        sie::set_stimer(); // 允许时钟中断
        // sie::clear_stimer(); // 关闭时钟中断
    }
    set_next_timeout(); // 设置下一次时钟中断
}

static INTERVAL: usize = 10000;

/// 设置下一次时钟中断
fn set_next_timeout() {
    set_timer(time::read() + INTERVAL);
}

pub static mut TICKS: usize = 0;

pub fn tick() {
    set_next_timeout();
    unsafe {
        TICKS = TICKS.wrapping_add(1);
        if TICKS % 1 == 0 {
            println!("[timer] {} tick", TICKS);
        }
    }
}
