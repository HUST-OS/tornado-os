/// 用户态设计思路
/// 
/// 用户和内核共用一个共享调度器，内核应该告诉用户共享内存的地址在哪。
/// 从内核态切换到用户态需要在虚拟地址空间有一个共享位置，保存上下文和切换的初始化函数。
/// 这个切换时保存的上下文的内容包括但不仅限于 `TrapFrame`，还包括内核页表，共享内核栈指针等等。
/// 目前这个上下文的结构暂时命名为 `SwapContext`，以区别于 `TrapFrame`，后面可能修改。
/// 
/// 上面思路的一个实现方法是，在内存中选择一页内存，S 和 U 态切换的上下文和初始化函数保存在这页内存里面。
/// 然后用户态地址空间和内核态地址空间都映射到这个位置，每次 S 态和 U 态切换的时候，都会从这里读取上下文并且运行这里的初始化函数。
/// 
/// 这样会有安全性问题，目前先把雏形搓出来，安全问题后面再考虑。

// use crate::trap::TrapFrame;
// use crate::task::{SHARED_SCHEDULER, SHARED_RAW_TABLE};

// todo: 只需要恢复TrapFrame、设置root_ppn？
// 地址空间编号、共享调度器由系统调用返回

/// 内核态和用户态切换时需要保存的上下文
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SwapContext {
    /// 内核的根页表的satp寄存器值，包括根页号、地址空间编号和页表模式
    kernel_satp: usize, // 0
    /// 内核栈指针
    kernel_stack: usize, // 8
    /// 陷入内核时候需要跳转到的函数指针
    user2kernel_trap: usize, // 16
    /// sepc 寄存器
    epc: usize, // 24
    /// 内核 tp 寄存器的值
    kernel_tp: usize, // 32
    /// 31 个通用寄存器，x0 被硬编码为 0 因此不用保存
    x: [usize; 31] // 40-280
}

impl SwapContext {
    // 新建一个用户态的 `SwapContext`，用于切换到用户态
    pub fn new_user(
        root_ppn: usize,
        pc: usize, // 将会被设置到 TrapFrame.sepc, sret 的时候会读取这个值
        tp: usize, // 用户态的 tp 寄存器，tp 指向的结构体由用户定义
        stack_top: usize // 用户栈顶
    ) -> Self {
        todo!()
    }
}

// 该函数的指针在从内核态返回到用户态之前被写到 stvec 寄存器里面去
// 用户态从这里开始陷入内核态
// 但目前的页表还是用户态的页表
// 先保存 SwapContext,然后切换到内核的地址空间
#[naked]
#[link_section = ".swap"]
unsafe extern "C" fn user2supervisor() -> ! {
    asm!(
    // 交换 a0 和 sscratch（原先保存着交换栈的栈顶指针）
    "csrrw a0, sscratch, a0",
    //开始保存 SwapContext
    "
    sd ra, 40(a0)
    sd sp, 48(a0)
    sd gp, 56(a0)
    sd tp, 64(a0)
    sd t0, 72(a0)
    sd t1, 80(a0)
    sd t2, 88(a0)
    sd s0, 96(a0)
    sd s1, 104(a0)
    sd a1, 120(a0)
    sd a2, 128(a0)
    sd a3, 136(a0)
    sd a4, 144(a0)
    sd a5, 152(a0)
    sd a6, 160(a0)
    sd a7, 168(a0)
    sd s2, 176(a0)
    sd s3, 184(a0)
    sd s4, 192(a0)
    sd s5, 200(a0)
    sd s6, 208(a0)
    sd s7, 216(a0)
    sd s8, 224(a0)
    sd s9, 232(a0)
    sd s10, 240(a0)
    sd s11, 248(a0)
    sd t3, 256(a0)
    sd t4, 264(a0)
    sd t5, 272(a0)
    sd t6, 280(a0)
    ",
    //保存用户的 a0 寄存器
    "csrr t0, sscratch
    sd t0, 112(a0)",

    // 保存用户栈顶指针
    "ld sp, 8(a0)",

    // 恢复内核态 tp 寄存器
    "ld tp, 32(a0)",

    // 将用户中断处理函数指针放到 t0 寄存器
    "ld t0, 16(a0)",

    // 恢复内核页表
    "ld t1, 0(a0)
    csrw satp, t1",

    "sfence.vma",

    // 跳转到中断处理函数
    "jr t0"
    , options(noreturn));
}

// 内核态切换到用户态最后通过这里
// 该函数有两个参数：
// a0：用户态 SwapContext 的裸指针
// a1：新的 satp 寄存器的值，用于切换地址空间

// trap_frame作为ctx?
#[link_section = ".swap"]
unsafe extern "C" fn supervisor2user(ctx: usize, satp: usize) -> ! {
    asm!("
    csrw satp, {satp}
    sfence.vma", // 刷新页表

    // 从 SwapContext 中恢复用户的上下文
    // 将用户的 a0 寄存器保存在 sscratch 寄存器中，
    // 这样子可以在最后一步将它和 a0（ctx） 进行交换
    "
    ld t0, 112({ctx})
    csrw sscratch, t0
    ",
    // 恢复通用寄存器的上下文
    "
    ld ra, 40({ctx})
    ld sp, 48({ctx})
    ld gp, 56({ctx})
    ld tp, 64({ctx})
    ld t0, 72({ctx})
    ld t1, 80({ctx})
    ld t2, 88({ctx})
    ld s0, 96({ctx})
    ld s1, 104({ctx})
    ld a1, 120({ctx})
    ld a2, 128({ctx})
    ld a3, 136({ctx})
    ld a4, 144({ctx})
    ld a5, 152({ctx})
    ld a6, 160({ctx})
    ld a7, 168({ctx})
    ld s2, 176({ctx})
    ld s3, 184({ctx})
    ld s4, 192({ctx})
    ld s5, 200({ctx})
    ld s6, 208({ctx})
    ld s7, 216({ctx})
    ld s8, 224({ctx})
    ld s9, 232({ctx})
    ld s10, 240({ctx})
    ld s11, 248({ctx})
    ld t3, 256({ctx})
    ld t4, 264({ctx})
    ld t5, 272({ctx})
    ld t6, 280({ctx})
    ",
    // 恢复用户的 a0 寄存器，并且保存交换栈顶在 sscratch 寄存器中
    "csrrw a0, sscratch, a0", 
    // 返回到用户态
    "sret",
    ctx = in(reg) ctx,
    satp = in(reg) satp,
    options(noreturn)
    )
}