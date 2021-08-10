//! 中断/异常管理模块
//!
//! 仅处理内核态中发生的中断/异常，用户态发生的中断/异常/系统调用处理在`src/syscall`模块。

#[cfg(target_pointer_width = "64")]
macro_rules! define_load_store {
    () => {
        ".altmacro
        .macro SAVE reg, offset
            sd  \\reg, \\offset*8(sp)
        .endm
        .macro SAVE_N n
            SAVE  x\\n, \\n
        .endm
        .macro LOAD reg, offset
            ld  \\reg, \\offset*8(sp)
        .endm
        .macro LOAD_N n
            LOAD  x\\n, \\n
        .endm"
    };
}

mod handler;
mod switch;
mod timer;

pub use handler::{trap_vector, TrapFrame};
pub use switch::*;

/// 初始化中断相关的子模块
///
/// - [`handler::init`]
/// - [`timer::init`]
pub fn init() {
    handler::init();
    timer::init();
    unsafe {
        riscv::register::sstatus::set_sie();
        riscv::register::sie::set_sext();
    }

    println!("mod interrupt initialized");
}
