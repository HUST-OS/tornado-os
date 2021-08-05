//! 从用户过来的系统调用在这里处理
use super::{syscall, SyscallResult};
use crate::hart::KernelHartInfo;
use crate::memory::{VirtualAddress, VirtualPageNumber, KERNEL_MAP_OFFSET};
use crate::trap;
use crate::task;
use crate::{
    memory::{self, Satp},
    trap::SwapContext,
};
use riscv::register::scause::{self, Interrupt, Trap};
use riscv::register::{sepc, stval};

/// 测试用的中断处理函数，用户态发生中断会陷入到这里
pub extern "C" fn user_trap_handler() {
    // 从 [`KernelHartInfo`] 中获取用户地址空间的 [`Satp`] 结构
    let user_satp = KernelHartInfo::prev_satp().expect("get prev user satp");
    // 从 [`KernelHartInfo`] 中获取用户地址空间编号
    let asid = KernelHartInfo::get_prev_asid();
    let swap_cx = unsafe { get_swap_cx(&user_satp, asid) };
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
            match syscall(param, user_satp.inner(), a6, a7) {
                SyscallResult::Procceed { code, extra } => {
                    swap_cx.x[9] = code;
                    swap_cx.x[10] = extra;
                    swap_cx.epc = swap_cx.epc.wrapping_add(4);
                    trap::switch_to_user(swap_cx, user_satp.inner(), asid)
                }
                SyscallResult::Retry => {
                    // 不跳过指令，继续运行
                    trap::switch_to_user(swap_cx, user_satp.inner(), asid)
                }
                SyscallResult::NextASID { asid, satp } => {
                    // 跳过 `do_yield` 指令
                    swap_cx.epc = swap_cx.epc.wrapping_add(4);
                    // 需要转到目标地址空间去运行
                    println!("[syscall] yield: {}", asid);
                    let next_swap_contex = unsafe { get_swap_cx(&satp, asid) };
                    trap::switch_to_user(next_swap_contex, satp.inner(), asid)
                }
                SyscallResult::KernelTask => {
                    // 跳过 `do_yield` 指令
                    swap_cx.epc = swap_cx.epc.wrapping_add(4);
                    println!("[syscall] yield kernel");
                    let shared_payload = unsafe { task::SharedPayload::load(crate::SHAREDPAYLOAD_BASE) };
                    task::run_until_idle(
                        || unsafe { shared_payload.peek_task(task::kernel_should_switch) },
                        |task_repr| unsafe { shared_payload.delete_task(task_repr) },
                        |task_repr, new_state| unsafe { shared_payload.set_task_state(task_repr, new_state) },
                    );
                    crate::end()
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
pub unsafe fn get_swap_cx<'cx>(satp: &'cx Satp, asid: usize) -> &'cx mut SwapContext {
    let swap_cx_va = VirtualAddress(memory::swap_contex_va(asid));
    let swap_cx_vpn = VirtualPageNumber::floor(swap_cx_va);
    let swap_cx_ppn = satp.translate(swap_cx_vpn).unwrap();
    // 将物理页号转换成裸指针
    (swap_cx_ppn
        .start_address()
        .0
        .wrapping_add(KERNEL_MAP_OFFSET) as *mut SwapContext)
        .as_mut()
        .unwrap()
}
