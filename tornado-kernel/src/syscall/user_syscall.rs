//! 从用户过来的系统调用在这里处理
use super::{syscall, SyscallResult};
use crate::trap;
use crate::{
    memory::{self, Satp},
    trap::SwapContext,
};
use riscv::register::scause::{self, Interrupt, Trap};
use riscv::register::{sepc, stval};

/// 测试用的中断处理函数，用户态发生中断会陷入到这里
pub extern "C" fn user_trap_handler() {
    // 用户地址空间的 satp 寄存器通过 t2 传给内核
    let user_satp: usize;
    unsafe {
        asm!("mv {}, t2", out(reg) user_satp, options(nomem, nostack));
    }
    let user_satp_2 = Satp::new(user_satp);
    let swap_cx = unsafe { get_swap_cx(&user_satp_2) };
    // 从 SwapContext 中读东西
    let mut param = [0usize; 6];
    for (idx, x) in swap_cx.x[9..15].iter().enumerate() {
        param[idx] = *x;
    }
    let a6 = swap_cx.x[15];
    let a7 = swap_cx.x[16];
    match scause::read().cause() {
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            println!("s mode timer!");
            // 目前遇到时钟中断先让系统退出，等把内核完善好了再来处理
            crate::sbi::shutdown();
        }
        Trap::Exception(scause::Exception::Breakpoint) => {
            println!("user mode panic!");
            crate::sbi::shutdown();
        }
        Trap::Exception(scause::Exception::UserEnvCall) => {
            match syscall(param, user_satp, a6, a7) {
                SyscallResult::Procceed { code, extra } => {
                    swap_cx.x[9] = code;
                    swap_cx.x[10] = extra;
                    swap_cx.epc = swap_cx.epc.wrapping_add(4);
                    // todo: 这里需要得到用户的地址空间编号，目前先写死为 1
                    trap::switch_to_user(swap_cx, user_satp, 1)
                }
                SyscallResult::Retry => {
                    // 不跳过指令，继续运行
                    // todo: 这里需要得到用户的地址空间编号，目前先写死为 1
                    trap::switch_to_user(swap_cx, user_satp, 1)
                }
                SyscallResult::NextASID { satp } => {
                    // 需要转到目标地址空间去运行
                    todo!()
                }
                SyscallResult::Terminate(exit_code) => {
                    println!("User exit!");
                    crate::sbi::shutdown();
                }
            }
        }
        _ => todo!(
            "scause: {:?}, sepc: {:#x}, stval: {:#x}, {:x?}",
            scause::read().cause(),
            sepc::read(),
            stval::read(),
            swap_cx
        ),
    }
}

// 给定 satp 寄存器，获取 [`SwapContext`] 的裸指针
// todo: 需要根据地址空间编号来得到 [`SwapContext`]
unsafe fn get_swap_cx<'cx>(satp: &'cx Satp) -> &'cx mut SwapContext {
    let swap_cx_va = memory::VirtualAddress(memory::swap_contex_va(1)); // 这里先暂时写死为 0
    let swap_cx_vpn = memory::VirtualPageNumber::floor(swap_cx_va);
    let swap_cx_ppn = satp.translate(swap_cx_vpn).unwrap();
    // 将物理页号转换成裸指针
    (swap_cx_ppn
        .start_address()
        .0
        .wrapping_add(memory::KERNEL_MAP_OFFSET) as *mut trap::SwapContext)
        .as_mut()
        .unwrap()
}
