//! 内核态和用户态切换时上下文管理细节实现
//!
//! 每个用户的上下文保存在一个[`SwapContex`]的结构。
//!
//! 每个[`SwapContex`]的虚拟地址由高地址往下递减，由用户的地址空间编号唯一确定。
//!
//! 地址计算方法请参考`src/memory/config.rs`函数。

use crate::{
    hart::KernelHartInfo,
    memory::{
        swap_contex_va, AddressSpaceId, SWAP_FRAME_VA
    }
};
/// 内核态和用户态切换时需要保存的上下文
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SwapContext {
    /// 31 个通用寄存器，x0 被硬编码为 0 因此不用保存
    pub x: [usize; 31], // 0 - 30
    /// 内核的根页表的satp寄存器值，包括根页号、地址空间编号和页表模式
    pub kernel_satp: usize, // 31
    /// 内核栈指针
    pub kernel_stack: usize, // 32
    /// 陷入内核时的处理函数
    pub user_trap_handler: usize, // 33
    /// sepc 寄存器
    pub epc: usize, // 34
    /// 内核 tp 寄存器的值
    pub kernel_tp: usize, // 35
}

impl SwapContext {
    /// 新建一个用户态的 `SwapContext`，用于特权级切换
    pub fn new_to_user(
        kernel_satp: usize,
        user_entry: usize,   // 将会被写到 sepc, sret 的时候会读取这个值
        kernel_tp: usize,    // 用户态的 tp 寄存器，tp 指向的结构体由用户定义
        kernel_stack: usize, // 内核栈指针
        user_stack: usize,   // 用户栈指针
        // 将会被写到 stvec 寄存器中返回到用户态
        // 用户态发生 Trap 时将会进入的处理函数
        user_trap_handler: usize,
    ) -> Self {
        let mut swap_context = Self {
            kernel_satp,
            kernel_stack,
            user_trap_handler,
            epc: user_entry,
            kernel_tp,
            x: [0; 31],
        };
        swap_context.set_sp(user_stack);
        swap_context
    }
    /// 设置[`SwapContext`]的`sp`寄存器
    pub fn set_sp(&mut self, sp: usize) -> &mut Self {
        self.x[1] = sp;
        self
    }
    /// 设置[`SwapContext`]的`gp`寄存器
    pub fn set_gp(&mut self, gp: usize) -> &mut Self {
        self.x[2] = gp;
        self
    }
    /// 设置[`SwapContext`]的`tp`寄存器
    pub fn set_tp(&mut self, tp: usize) -> &mut Self {
        self.x[3] = tp;
        self
    }
}

/// 用户态切换到内核态，用户态从这里开始陷入。
///
/// 该函数的指针在从内核态返回到用户态之前被写到 stvec 寄存器里面去，
/// 但目前的页表还是用户态的页表。
///
/// 先保存[`SwapContext`]结构，也就是用户上下文，然后切换到内核的地址空间。
#[link_section = ".swap"]
#[export_name = "_user_to_supervisor"]
pub unsafe extern "C" fn user_to_supervisor() -> ! {
    asm!(
        // 交换 a0 和 sscratch（原先保存着交换栈的栈顶指针）
        "csrrw  a0, sscratch, a0",
        //开始保存 SwapContext
        "
    sd      ra,  0*8(a0)
    sd      sp,  1*8(a0)
    sd      gp,  2*8(a0)
    sd      tp,  3*8(a0)
    sd      t0,  4*8(a0)
    sd      t1,  5*8(a0)
    sd      t2,  6*8(a0)
    sd      s0,  7*8(a0)
    sd      s1,  8*8(a0)
    sd      a1,  10*8(a0)
    sd      a2,  11*8(a0)
    sd      a3,  12*8(a0)
    sd      a4,  13*8(a0)
    sd      a5,  14*8(a0)
    sd      a6,  15*8(a0)
    sd      a7,  16*8(a0)
    sd      s2,  17*8(a0)
    sd      s3,  18*8(a0)
    sd      s4,  19*8(a0)
    sd      s5,  20*8(a0)
    sd      s6,  21*8(a0)
    sd      s7,  22*8(a0)
    sd      s8,  23*8(a0)
    sd      s9,  24*8(a0)
    sd      s10, 25*8(a0)
    sd      s11, 26*8(a0)
    sd      t3,  27*8(a0)
    sd      t4,  28*8(a0)
    sd      t5,  29*8(a0)
    sd      t6,  30*8(a0)
    ",
        // 保存用户的 a0 寄存器
        "csrr   t0, sscratch
    sd      t0, 9*8(a0)",
        // 写 sepc 寄存器到 SwapContext 中相应位置
        "csrr   t0, sepc
    sd      t0, 34*8(a0)",
        // 恢复内核栈指针
        "ld     sp, 32*8(a0)",
        // todo: 如何处理 tp 寄存器
        "ld     tp, 35*8(a0)",
        // 将用户中断处理函数指针放到 t0 寄存器
        "ld     t0, 33*8(a0)",
        // // 将用户的 satp 寄存器放到 t2 寄存器里面去
        // "csrr   t2, satp",
        // 恢复内核页表
        "ld     t1, 31*8(a0)
    csrw    satp, t1",
        "sfence.vma",
        // 跳转到中断处理函数
        "jr     t0",
        options(noreturn)
    );
}

/// 内核态切换到用户态的最后一道关卡。
///
/// 该函数有两个参数：
/// a0：用户态 SwapContext 的裸指针
/// a1：新的 satp 寄存器的值，用于切换地址空间
#[link_section = ".swap"]
#[export_name = "_supervisor_to_user"]
pub unsafe extern "C" fn supervisor_to_user() -> ! {
    asm!(
        "csrw   satp, a1
    sfence.vma", // 刷新页表
        // 从 SwapContext 中恢复用户的上下文
        // 将用户的 a0 寄存器保存在 sscratch 寄存器中，
        // 这样子可以在最后一步将它和 a0（ctx） 进行交换
        "
    ld      t0, 9*8(a0)
    csrw    sscratch, t0
    ",
        // 恢复通用寄存器的上下文
        "
    ld      ra,  0*8(a0)
    ld      sp,  1*8(a0)
    ld      gp,  2*8(a0)
    ld      tp,  3*8(a0)
    ld      t0,  4*8(a0)
    ld      t1,  5*8(a0)
    ld      t2,  6*8(a0)
    ld      s0,  7*8(a0)
    ld      s1,  8*8(a0)
    ld      a1,  10*8(a0)
    ld      a2,  11*8(a0)
    ld      a3,  12*8(a0)
    ld      a4,  13*8(a0)
    ld      a5,  14*8(a0)
    ld      a6,  15*8(a0)
    ld      a7,  16*8(a0)
    ld      s2,  17*8(a0)
    ld      s3,  18*8(a0)
    ld      s4,  19*8(a0)
    ld      s5,  20*8(a0)
    ld      s6,  21*8(a0)
    ld      s7,  22*8(a0)
    ld      s8,  23*8(a0)
    ld      s9,  24*8(a0)
    ld      s10, 25*8(a0)
    ld      s11, 26*8(a0)
    ld      t3,  27*8(a0)
    ld      t4,  28*8(a0)
    ld      t5,  29*8(a0)
    ld      t6,  30*8(a0)
    ",
        // 恢复用户的 a0 寄存器，并且保存交换栈顶在 sscratch 寄存器中
        "csrrw  a0, sscratch, a0",
        // 返回到用户态
        "sret",
        options(noreturn)
    )
}


/// 上升到用户态
///
/// 让这个函数接收一个[`SwapContext`]结构的引用和用户的页表还有地址空间编号
#[no_mangle]
pub fn switch_to_user(context: &SwapContext, user_satp: usize, user_asid: usize) -> ! {
    use riscv::register::{
        sstatus::{self, SPP},
        stvec::{self, TrapMode},
    };
    // 关中断
    unsafe {
        sstatus::clear_sie();
    }
    extern "C" {
        fn _swap_frame();
        fn _supervisor_to_user();
    }
    // 用户态发生中断时 pc 将会被设置成此值
    let user_trap_va = SWAP_FRAME_VA as usize;
    // 该函数最后应该跳转的虚拟地址
    let jmp_va = _supervisor_to_user as usize - _swap_frame as usize + SWAP_FRAME_VA;

    // 设置用户态陷入内核时需要跳转的地址
    unsafe {
        stvec::write(user_trap_va, TrapMode::Direct);
    }

    // 设置 sstatus.SPP 的值为 User
    unsafe {
        sstatus::set_spp(SPP::User);
    }

    // 将 SwapContext.epc 写到 sepc 寄存器
    // 这个是用户程序入口
    riscv::register::sepc::write(context.epc);

    // 将即将要进入的用户地址空间编号写入 [`KernelHartInfo`]
    KernelHartInfo::set_prev_asid(user_asid);

    unsafe {
        llvm_asm!("fence.i" :::: "volatile");
        llvm_asm!("jr $0" :: "r"(jmp_va), "{a0}"(swap_contex_va(user_asid)), "{a1}"(user_satp) :: "volatile");
    }
    unreachable!()
}