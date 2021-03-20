mod handler;
mod timer;

pub use handler::TrapFrame;

/// 初始化中断相关的子模块
/// 
/// - [`handler::init`]
/// - [`timer::init`]
pub fn init() {
    handler::init();
    timer::init();
    println!("mod interrupt initialized");
}

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

// 这个函数里不包含写satp的过程，需要别的函数先写satp和刷新页表
#[naked]
#[link_section = ".text"]
unsafe extern "C" fn supervisor_restore(_target_frame: *mut TrapFrame) -> ! {
    asm!(define_load_store!(), "
        mv      sp, a0

        LOAD    t0, 32
        LOAD    t1, 33
        csrw    sstatus, t0
        csrw    sepc, t1

        LOAD    x1, 1
        .set    n, 3
        .rept   29
            LOAD_N  %n
            .set    n, n + 1
        .endr

        LOAD	sp, 2
        sret
    ", options(noreturn))
}


/// 内核态和用户态切换时需要保存的上下文
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SwapContext {
    /// 内核的根页表的satp寄存器值，包括根页号、地址空间编号和页表模式
    kernel_satp: usize, // 0
    /// 内核栈指针
    kernel_stack: usize, // 8
    /// 陷入内核时的处理函数
    user_trap_handler: usize, // 16
    /// sepc 寄存器
    epc: usize, // 24
    /// 内核 tp 寄存器的值
    kernel_tp: usize, // 32
    /// 31 个通用寄存器，x0 被硬编码为 0 因此不用保存
    x: [usize; 31] // 40-280
}

impl SwapContext {
    // 新建一个用户态的 `SwapContext`，用于切换到用户态
    pub fn new_to_user(
        kernel_satp: usize,
        user_entry: usize, // 将会被写到 sepc, sret 的时候会读取这个值
        tp: usize, // 用户态的 tp 寄存器，tp 指向的结构体由用户定义
        kernel_stack: usize, // 内核态指针
        user_stack: usize, // 用户栈指针
        // 将会被写到 stvec 寄存器中返回到用户态
        // 用户态发生 Trap 时将会进入的处理函数
        user_trap_handler: usize
    ) -> Self {
        let mut swap_context = Self {
            kernel_satp,
            kernel_stack,
            user_trap_handler,
            epc: user_entry,
            kernel_tp: tp,
            x: [0; 31]
        };
        swap_context.set_sp(user_stack);
        swap_context
    }
    pub fn set_sp(&mut self, sp: usize) -> &mut Self{
        self.x[2] = sp;
        self
    }
}

// 内核态切换到用户态最后通过这里
// 该函数有两个参数：
// a0：用户态 SwapContext 的裸指针
// a1：新的 satp 寄存器的值，用于切换地址空间
#[link_section = ".swap"]
pub unsafe extern "C" fn supervisor_to_user(ctx: *mut SwapContext, satp: usize) -> ! {
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

use crate::memory::{SWAP_FRAME_VA, SWAP_CONTEXT_VA};

/// 上升到用户态
pub fn switch_to_user() -> ! {
    use riscv::register::{sstatus::{self, SPP}, stvec::{self, TrapMode}};
    // 关中断
    unsafe { sstatus::clear_sie(); }
    extern "C" {
        fn _swap_frame();
    }
    let user_trap_va = SWAP_FRAME_VA as usize;
    let jmp_va = supervisor_to_user as usize - _swap_frame as usize + SWAP_FRAME_VA;
    println!("jmp_va = {:#x}", jmp_va);
    
    // 设置用户态陷入内核时需要跳转的地址
    unsafe { stvec::write(user_trap_va, TrapMode::Direct); }

    // 设置 sstatus.SPP 的值为 User
    unsafe { sstatus::set_spp(SPP::User); }

    // todo: 将 SwapContext.epc 写到 sepc 寄存器
    // todo: 如何处理 tp 寄存器

    // 用户 satp 寄存器
    // 需要获取当前任务的页表
    let user_satp: usize = 0; // todo!
    unsafe {
        asm!(
            "fence.i",
            "li     a0, {}",
            "mv     a1, {}",
            "jr     {}",
            const SWAP_CONTEXT_VA,
            in(reg) user_satp,
            in(reg) jmp_va,
        );
    }
    unreachable!()
}
