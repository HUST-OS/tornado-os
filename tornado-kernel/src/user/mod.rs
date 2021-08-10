//! 用户态相关
//!
//! 本项目通过跳板页来进行内核态和用户态的特权级切换，还有用户/内核上下文的保存，
//! 跳板页原理可参考[xv6-book](https://pdos.csail.mit.edu/6.828/2019/xv6/book-riscv-rev0.pdf)中的`Traps and device drivers`章节
//!
//! 本模块负责以下几个部分：
//! * 从文件系统中加载用户程序到内存
//! * 将每个用户的上下文放到[`KernelHartInfo`]结构中进行管理，具体请看`src/hart.rs`
//! * 内核态切换到用户态的具体实现
//!
mod load;
mod space;
mod trap;

pub use trap::{enter_user, prepare_user};
