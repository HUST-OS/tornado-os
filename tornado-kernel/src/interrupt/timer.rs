#![allow(unused)] // todo：协作式调度写完后移除
use crate::sbi::set_timer;
use riscv::register::{time, sie};

/// 初始化时钟中断
pub fn init() {
    // todo: 先写协作式调度

    // unsafe {
    //     sie::set_stimer(); // 允许时钟中断
    // }
    // set_next_timeout(); // 设置下一次时钟中断
}

static INTERVAL: usize = 100000;

fn set_next_timeout() {
    set_timer(time::read() + INTERVAL);
}

pub static mut TICKS: usize = 0;

pub fn tick() {
    set_next_timeout();
    unsafe {
        TICKS = TICKS.wrapping_add(1);
        if TICKS % 100 == 0 {
            println!("{} tick", TICKS);
        }
    }
}
