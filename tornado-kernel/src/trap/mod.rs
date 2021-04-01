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
    pub kernel_satp: usize, // 0
    /// 内核栈指针
    pub kernel_stack: usize, // 8
    /// 陷入内核时的处理函数
    pub user_trap_handler: usize, // 16
    /// sepc 寄存器
    pub epc: usize, // 24
    /// 内核 tp 寄存器的值
    pub kernel_tp: usize, // 32
    /// 31 个通用寄存器，x0 被硬编码为 0 因此不用保存
    pub x: [usize; 31] // 40-280
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

// 该函数的指针在从内核态返回到用户态之前被写到 stvec 寄存器里面去
// 用户态从这里开始陷入内核态
// 但目前的页表还是用户态的页表
// 先保存 SwapContext,然后切换到内核的地址空间
#[link_section = ".swap"]
#[export_name = "_user_to_supervisor"]
pub unsafe extern "C" fn user_to_supervisor() -> ! {
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

    // 恢复内核栈指针
    "ld sp, 8(a0)",

    // todo: 如何处理 tp 寄存器
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
#[link_section = ".swap"]
#[export_name = "_supervisor_to_user"]
pub unsafe extern "C" fn supervisor_to_user() -> ! {
    asm!(
    "csrw satp, a1
    sfence.vma", // 刷新页表

    // 从 SwapContext 中恢复用户的上下文
    // 将用户的 a0 寄存器保存在 sscratch 寄存器中，
    // 这样子可以在最后一步将它和 a0（ctx） 进行交换
    "
    ld t0, 112(a0)
    csrw sscratch, t0
    ",
    // 恢复通用寄存器的上下文
    "
    ld ra, 40(a0)
    ld sp, 48(a0)
    ld gp, 56(a0)
    ld tp, 64(a0)
    ld t0, 72(a0)
    ld t1, 80(a0)
    ld t2, 88(a0)
    ld s0, 96(a0)
    ld s1, 104(a0)
    ld a1, 120(a0)
    ld a2, 128(a0)
    ld a3, 136(a0)
    ld a4, 144(a0)
    ld a5, 152(a0)
    ld a6, 160(a0)
    ld a7, 168(a0)
    ld s2, 176(a0)
    ld s3, 184(a0)
    ld s4, 192(a0)
    ld s5, 200(a0)
    ld s6, 208(a0)
    ld s7, 216(a0)
    ld s8, 224(a0)
    ld s9, 232(a0)
    ld s10, 240(a0)
    ld s11, 248(a0)
    ld t3, 256(a0)
    ld t4, 264(a0)
    ld t5, 272(a0)
    ld t6, 280(a0)
    ",
    // 恢复用户的 a0 寄存器，并且保存交换栈顶在 sscratch 寄存器中
    "csrrw a0, sscratch, a0",
    // 返回到用户态
    "sret",
    options(noreturn)
    )
}

use crate::memory::{SWAP_FRAME_VA, SWAP_CONTEXT_VA};

/// 上升到用户态
/// 目前让这个函数接收一个 SwapContext 参数和用户的页表，测试使用
#[no_mangle]
pub fn switch_to_user(context: &SwapContext, user_satp: usize) -> ! {
    use riscv::register::{sstatus::{self, SPP}, stvec::{self, TrapMode}};
    // 关中断
    unsafe { sstatus::clear_sie(); }
    extern "C" {
        fn _swap_frame();
        fn _supervisor_to_user();
    }
    // 用户态发生中断时 pc 将会被设置成此值
    let user_trap_va = SWAP_FRAME_VA as usize;
    // 该函数最后应该跳转的虚拟地址
    let jmp_va = supervisor_to_user as usize - _swap_frame as usize + SWAP_FRAME_VA;
    // println!("jmp_va = {:#x}", jmp_va);
    
    // 设置用户态陷入内核时需要跳转的地址
    unsafe { stvec::write(user_trap_va, TrapMode::Direct); }

    // 设置 sstatus.SPP 的值为 User
    unsafe { sstatus::set_spp(SPP::User); }

    // 将 SwapContext.epc 写到 sepc 寄存器
    // 这个是用户程序入口
    // println!("sepc: {:#x}", context.epc);
    riscv::register::sepc::write(context.epc);

    // todo: 如何处理 tp 寄存器
        
    // unsafe {
    //     asm!(
    //         "fence.i",
    //         "li     a0, {}",
    //         "mv     a1, {}",
    //         "jr     {}",
    //         const SWAP_CONTEXT_VA,
    //         in(reg) user_satp,
    //         in(reg) jmp_va,
    //         options(noreturn)
    //     );
    // }
    // 上面这样写生产出的汇编好像不太对，因此改为下面这样写
    unsafe {
        llvm_asm!("fence.i" :::: "volatile");
        llvm_asm!("jr $0" :: "r"(jmp_va), "{a0}"(SWAP_CONTEXT_VA), "{a1}"(user_satp) :: "volatile");
    }
    unreachable!()
}

// 打印 satp 寄存器
#[allow(unused)]
fn print_satp(satp: usize) {
    use bit_field::BitField;
    println!("root ppn: {:#x}", &satp.get_bits(0..44));
}
