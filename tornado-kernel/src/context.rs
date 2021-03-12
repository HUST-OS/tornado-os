use core::usize;

/// 用户态设计思路
/// 
/// 用户和内核共用一个共享调度器，内核应该告诉用户共享内存的地址在哪。
/// 从内核态切换到用户态需要在虚拟地址空间有一个共享位置，保存上下文和切换的初始化函数。
/// 这个切换时保存的上下文的内容包括但不仅限于 `TrapFrame`，还包括 `satp`，`tp`，`sp`，等等。
/// 目前这个上下文的结构暂时命名为 `Context`，以区别于 `TrapFrame`，后面可能修改。
/// 
/// 上面思路的一个实现方法是，在链接脚本里面添加一个段（类似于 shared_data），S 和 U 态切换的上下文和初始化函数保存在这个段里面。
/// 然后用户态地址空间和内核态地址空间都映射到这个位置，每次 S 态和 U 态切换的时候，都会从这里读取上下文并且运行这里的初始化函数。
/// 
/// 貌似共享内存的位置也可以通过这个方法在内核态和用户态之间共享。
/// 
/// 这样会有安全性问题，目前先把雏形搓出来，安全问题后面再考虑。

use trap::TrapFrame;

use crate::trap;

/// 内核态和用户态切换时需要保存的上下文
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Context<S, T> {
    // 页表基址
    satp: usize,
    // tp寄存器，内核中 tp 寄存器指向 `KernelHartInfo`
    tp: usize,
    // 栈顶
    sp: usize,
    trapframe: TrapFrame,
    // 共享调度器指针
    shared_scheduler: *mut S,
    // 共享调度函数表
    // 包括添加新任务，弹出下一个任务
    shared_raw_table: *mut T,
}

