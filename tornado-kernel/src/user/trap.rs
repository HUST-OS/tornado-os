use super::load::load_user;
use crate::memory::{
    AddressSpaceId, MemorySet, VirtualAddress, VirtualPageNumber, KERNEL_MAP_OFFSET,
    SWAP_CONTEXT_VA,
};
use crate::task;
use crate::trap;
use alloc::string::String;

/// 第一次进入用户态
pub async fn first_enter_user<S: Into<String>>(
    user: S,
    kernel_stack_top: usize,
) {
    // 创建一个用户态映射
    let user_memory = load_user(user).await;
    // 存放用户特权级切换上下文的虚拟地址
    let swap_cx_va = VirtualAddress(SWAP_CONTEXT_VA);
    // 存放用户特权级切换上下文的虚拟页号
    let swap_cx_vpn = VirtualPageNumber::floor(swap_cx_va);
    // 获取存放用户特权级切换上下文的物理页号
    let swap_cx_ppn = user_memory
        .mapping
        .translate(swap_cx_vpn)
        .unwrap()
        .page_number();
    // 将物理页号转换为裸指针
    let swap_cx = unsafe {
        (swap_cx_ppn
            .start_address()
            .0
            .wrapping_add(KERNEL_MAP_OFFSET) as *mut trap::SwapContext)
            .as_mut()
            .unwrap()
    };

    // 获取用户的 satp 寄存器
    let user_satp = user_memory.mapping.get_satp(user_memory.address_space_id);
    let process = task::Process::new_user(user_memory).unwrap();

    // 用户态栈
    let user_stack_handle = process.alloc_stack().expect("alloc user stack");
    // 这里减 4 是因为映射的时候虚拟地址的右半边是不包含的
    let user_stack_top = user_stack_handle.end.0 - 4;

    // 获取用户地址空间编号
    let user_asid = process.address_space_id().into_inner();
    // 获取内核的satp寄存器
    let kernel_satp = riscv::register::satp::read().bits();

    let tp: usize;
    unsafe {
        asm!("mv {}, tp", out(reg) tp, options(nomem, nostack));
    }
    // 往 SwapContext 写东西
    // 目前通过 tp 寄存器把地址空间编号传给用户，后面可能会修改
    *swap_cx = trap::SwapContext::new_to_user(
        kernel_satp,
        0,
        tp,
        kernel_stack_top,
        user_stack_top,
        crate::syscall::user_trap_handler as usize,
    );

    // 在这里把共享运行时中 raw_table 的地址通过 gp 寄存器传给用户
    swap_cx.set_gp(crate::SHAREDPAYLOAD_BASE);
    swap_cx.set_tp(user_asid);
    trap::switch_to_user(swap_cx, user_satp);
}
