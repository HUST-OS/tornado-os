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
// use crate::task::{SHARED_SCHEDULER, SHARED_RAW_TABLE};

// todo: 只需要恢复TrapFrame、设置root_ppn？
// 地址空间编号、共享调度器由系统调用返回

/// 内核态和用户态切换时需要保存的上下文
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SwapContext {
    trap_frame: TrapFrame,
    // 根页表的物理页号
    root_ppn: usize,
}

impl SwapContext {
    // 新建一个用户态的 `SwapContext`，用于切换到用户态
    pub fn new_user(
        root_ppn: usize,
        pc: usize, // 将会被设置到 TrapFrame.sepc, sret 的时候会读取这个值
        tp: usize, // 用户态的 tp 寄存器，tp 指向的结构体由用户定义
        stack_top: usize // 用户栈顶
    ) -> Self {
        let trap_frame = TrapFrame::new_task_context(true, pc, tp, stack_top);
        Self {
            root_ppn,
            trap_frame,
            // todo: 下面几两项是否可以通过系统调用完成？
            // asid,
            // shared_scheduler: &SHARED_SCHEDULER as *const _ as *mut (),  // 指向共享调度器的指针
            // shared_raw_table: &SHARED_RAW_TABLE as *const _ as *mut ()   // 指向共享调度函数表的指针
        }
    }
}

// 内核态切换到用户态最后通过这里
// 该函数有两个参数：
// a0：用户态 SwapContext 的裸指针
// 这个裸指针指向的 SwapContext 中的 trapframe 成员里面会保存返回用户态时候的 a0 和 a1 寄存器
// 在 RISC-V 标准里面 a0 和 a1 是函数返回值
// 因此尝试将共享调度器和共享调度函数表的地址通过这两个寄存器返回给用户态
// a1：新的 satp 寄存器的值，用于切换地址空间

// trap_frame作为ctx?
unsafe extern "C" fn enter_user(ctx: usize, satp: usize) -> ! {
    asm!("
    csrw satp, {satp}
    sfence.vma", // 刷新页表
    // // 将 shared_scheduler 的值暂时保存在 sscratch 寄存器中
    // "ld t0, 272({ctx})
    // csrw sscratch, t0", // todo: 可以用系统调用返回共享调度器吗？
    // 从 SwapContext 中恢复用户的上下文
    "
    ld x1, 8({ctx})
    ld x2, 16({ctx})
    ld x3, 24({ctx})
    ld x4, 32({ctx})
    ld x5, 40({ctx})
    ld x6, 48({ctx})
    ld x7, 56({ctx})
    ld x8, 64({ctx})
    ld x9, 72({ctx})
    ld x10, 80({ctx})
    ld x11, 88({ctx})
    ld x12, 96({ctx})
    ld x13, 104({ctx})
    ld x14, 112({ctx})
    ld x15, 120({ctx})
    ld x16, 128({ctx})
    ld x17, 136({ctx})
    ld x18, 144({ctx})
    ld x19, 152({ctx})
    ld x20, 160({ctx})
    ld x21, 168({ctx})
    ld x22, 176({ctx})
    ld x23, 184({ctx})
    ld x24, 192({ctx})
    ld x25, 200({ctx})
    ld x26, 208({ctx})
    ld x27, 216({ctx})
    ld x28, 224({ctx})
    ld x29, 232({ctx})
    ld x30, 240({ctx})
    ld x31, 248({ctx})",
    //  交换 sccratch 和 a0 的值，这个值变成了共享调度器的地址
    "", // todo
    // 返回到用户态
    "sret",
    ctx = in(reg) ctx,
    satp = in(reg) satp,
    options(noreturn)
    )
}

// global_asm!(
//     "
//     .pushsection .swap_text
//     .globl user2supervisor
//     .globl supervisor2user
//     .align 4
// # 用户态切换到内核态首先跳转到这里
// user2supervisor:

// # 内核态切换到用户态最后通过这里
// # 该函数有两个参数：
// # a0：用户态 SwapContext 的裸指针
// # 这个裸指针指向的 SwapContext 中的 trapframe 成员里面会保存返回用户态时候的 a0 和 a1 寄存器
// # 在 RISC-V 标准里面 a0 和 a1 是函数返回值
// # 因此尝试将共享调度器和共享调度函数表的地址通过这两个寄存器返回给用户态
// # a1：新的 satp 寄存器的值，用于切换地址空间
// supervisor2user:
//     csrw satp, a1
//     sfence.vma  # 刷新页表
//     # 将 shared_scheduler 的值暂时保存在 sscratch 寄存器中
//     ld t0, 272(a0)
//     csrw sscratch, t0
    
//     # 从 SwapContext 中恢复用户的上下文
//     ld x1, 16(a0)
//     ld x2, 24(a0)
//     ld x3, 32(a0)
//     ld x4, 40(a0)
//     ld x5, 48(a0)
//     ld x6, 56(a0)
//     ld x7, 64(a0)
//     ld x8, 72(a0)
//     ld x9, 80(a0)
//     ld x11, 280(a0) # shared_raw_table: *mut T
//     ld x12, 96(a0)
//     ld x13, 104(a0)
//     ld x14, 112(a0)
//     ld x15, 120(a0)
//     ld x16, 128(a0)
//     ld x17, 136(a0)
//     ld x18, 144(a0)
//     ld x19, 152(a0)
//     ld x20, 160(a0)
//     ld x21, 168(a0)
//     ld x22, 176(a0)
//     ld x23, 184(a0)
//     ld x24, 192(a0)
//     ld x25, 200(a0)
//     ld x26, 208(a0)
//     ld x27, 216(a0)
//     ld x28, 224(a0)
//     ld x29, 232(a0)
//     ld x30, 240(a0)
//     ld x31, 248(a0)

//     # 交换 sccratch 和 a0 的值，这个值变成了共享调度器的地址

//     # 返回到用户态
//     sret
//     "
// );