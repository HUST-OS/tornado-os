/// 用户态设计思路
/// 
/// 用户和内核共用一个共享调度器，内核应该告诉用户共享内存的地址在哪。
/// 从内核态切换到用户态需要在虚拟地址空间有一个共享位置，保存上下文和切换的初始化函数。
/// 这个切换时保存的上下文的内容包括但不仅限于 `TrapFrame`，还包括内核页表，共享内存指针等等。
/// 目前这个上下文的结构暂时命名为 `SwapContext`，以区别于 `TrapFrame`，后面可能修改。
/// 
/// 上面思路的一个实现方法是，在链接脚本里面添加一个段（类似于 shared_data），S 和 U 态切换的上下文和初始化函数保存在这个段里面。
/// 然后用户态地址空间和内核态地址空间都映射到这个位置，每次 S 态和 U 态切换的时候，都会从这里读取上下文并且运行这里的初始化函数。
/// 
/// 在 RISC-V 中， a0 和 a1 这两个寄存器用于函数返回值，尝试通过这两个寄存器告诉用户态共享调度器和共享调度函数表的位置
/// 
/// 这样会有安全性问题，目前先把雏形搓出来，安全问题后面再考虑。

use crate::trap::TrapFrame;
use crate::task::{SHARED_SCHEDULER, SHARED_RAW_TABLE};

/// 内核态和用户态切换时需要保存的上下文
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SwapContext<S, T> {
    // 根页表的物理页号
    pagetable: usize, // 0
    // x[32]: 8 - 256
    trapframe: TrapFrame,
    // 地址空间编号
    // todo: u16
    asid: usize, // 264
    // 共享调度器指针
    shared_scheduler: *mut S, // 272
    // 共享调度函数表
    // 包括添加新任务，弹出下一个任务
    shared_raw_table: *mut T, // 280
}

impl<S, T> SwapContext<S, T> {
    // 新建一个用户态的 `SwapContext`，用于切换到用户态
    #[allow(unused)]
    pub fn new_user(
        pagetable: usize,
        asid: usize,
        pc: usize, // 将会被设置到 TrapFrame.sepc, sret 的时候会读取这个值
        tp: usize, // 用户态的 tp 寄存器，tp 指向的结构体由用户定义
        stack_top: usize // 用户栈顶
    ) -> Self {
        let trapframe = TrapFrame::new_task_context(true, pc, tp, stack_top);
        Self {
            pagetable,
            trapframe,
            asid,
            shared_scheduler: &SHARED_SCHEDULER as *const _ as *mut S,  // 指向共享调度器的指针
            shared_raw_table: &SHARED_RAW_TABLE as *const _ as *mut T   // 指向共享调度函数表的指针
        }
    }
    // 设置上下文的根页表
    #[allow(unused)]
    pub fn set_pagetable(&mut self, root_ppn: usize) {
        self.pagetable = root_ppn;
    }
    // 设置 TrapFrame
    #[allow(unused)]
    pub fn set_trapframe(&mut self, trapframe: TrapFrame) {
        self.trapframe = trapframe;
    }

}

#[no_mangle]
/// 进入用户态
pub fn enter_user() -> ! {
    extern "C" {
        fn user2supervisor();
        fn supervisor2user();
    }
    
    todo!()
}

global_asm!(
    "
    .section .swap_text
    .globl user2supervisor
    .globl supervisor2user
    .align 4
# 用户态切换到内核态首先跳转到这里
user2supervisor:

# 内核态切换到用户态最后通过这里
# 该函数有两个参数：
# a0：用户态 SwapContext 的裸指针
# 这个裸指针指向的 SwapContext 中的 trapframe 成员里面会保存返回用户态时候的 a0 和 a1 寄存器
# 在 RISC-V 标准里面 a0 和 a1 是函数返回值
# 因此尝试将共享调度器和共享调度函数表的地址通过这两个寄存器返回给用户态
# a1：新的 satp 寄存器的值，用于切换地址空间
supervisor2user:
    csrw satp, a1
    sfence.vma  # 刷新页表
    # 将 shared_scheduler 的值暂时保存在 sscratch 寄存器中
    ld t0, 272(a0)
    csrw sscratch, t0
    
    # 从 SwapContext 中恢复用户的上下文
    ld x1, 16(a0)
    ld x2, 24(a0)
    ld x3, 32(a0)
    ld x4, 40(a0)
    ld x5, 48(a0)
    ld x6, 56(a0)
    ld x7, 64(a0)
    ld x8, 72(a0)
    ld x9, 80(a0)
    ld x11, 280(a0) # shared_raw_table: *mut T
    ld x12, 96(a0)
    ld x13, 104(a0)
    ld x14, 112(a0)
    ld x15, 120(a0)
    ld x16, 128(a0)
    ld x17, 136(a0)
    ld x18, 144(a0)
    ld x19, 152(a0)
    ld x20, 160(a0)
    ld x21, 168(a0)
    ld x22, 176(a0)
    ld x23, 184(a0)
    ld x24, 192(a0)
    ld x25, 200(a0)
    ld x26, 208(a0)
    ld x27, 216(a0)
    ld x28, 224(a0)
    ld x29, 232(a0)
    ld x30, 240(a0)
    ld x31, 248(a0)

    # 交换 sccratch 和 a0 的值，这个值变成了共享调度器的地址

    # 返回到用户态
    sret
    "
);