use riscv::register::scause::{self, Trap, Interrupt};
use riscv::register::{sepc, stval};
/// 临时的用户态程序和数据

use crate::memory::{self, PAGE_SIZE};
use crate::trap;
use crate::task;

// 尝试进入用户态
pub fn try_enter_user(kernel_stack_top: usize) -> ! {
    extern {
        // 用户陷入内核时候的中断处理函数
        fn _test_user_trap();
        // 用户程序入口点
        fn _test_user_entry();
        fn _sshared_data();
        fn _eshared_data();
        fn _sshared_text();
        fn _eshared_text();
    }
   
    // 创建一个用户态映射
    // 用户态程序目前写死在 0x87000000 处
    let user_memory = memory::MemorySet::new_bin().unwrap();
    
    // 存放用户特权级切换上下文的虚拟地址
    let swap_cx_va = memory::VirtualAddress(memory::SWAP_CONTEXT_VA);
    // 存放用户特权级切换上下文的虚拟页号
    let swap_cx_vpn =  memory::VirtualPageNumber::floor(swap_cx_va);
    // 获取存放用户特权级切换上下文的物理页号
    let swap_cx_ppn = user_memory.mapping
        .translate(swap_cx_vpn)
        .unwrap()
        .page_number();
    // 将物理页号转换为裸指针
    let swap_cx = unsafe { (swap_cx_ppn.start_address().0.wrapping_add(memory::KERNEL_MAP_OFFSET) as *mut trap::SwapContext).as_mut().unwrap() };
    // 获取用户的satp寄存器
    let user_satp = user_memory.mapping.get_satp(user_memory.address_space_id);
    let process = task::Process::new_user(user_memory).unwrap();
    // 用户态栈
    let user_stack_handle = process.alloc_stack().expect("alloc user stack");
    // 这里减 4 是因为映射的时候虚拟地址的右半边是不包含的
    let user_stack_top = user_stack_handle.end.0 - 4;
    println!("kernel stack top: {:#x}, user stack top: {:#x}", kernel_stack_top, user_stack_top);
    // 获取内核的satp寄存器
    let kernel_satp = riscv::register::satp::read().bits();

    // 往 SwapContext 写东西
    *swap_cx = trap::SwapContext::new_to_user(
        kernel_satp, 0, 0, kernel_stack_top, user_stack_top, _test_user_trap as usize
    );
    
    // 在这里把共享运行时中 raw_table 的地址通过 gp 寄存器传给用户
    swap_cx.set_gp(0x8021_b000);
    trap::switch_to_user(swap_cx, user_satp)
}

// 测试用的中断处理函数，用户态发生中断会陷入到这里
#[export_name = "_test_user_trap"]
pub extern "C" fn test_user_trap() {
    match scause::read().cause() {
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            println!("s mode timer!");
            // 目前遇到时钟中断先让系统退出，等把内核完善好了再来处理
            crate::sbi::shutdown();
        },
        Trap::Exception(scause::Exception::Breakpoint) => {
            println!("user mode panic!");
            crate::sbi::shutdown();
        },
        Trap::Exception(scause::Exception::UserEnvCall) => {
            println!("ecall from user.");
            crate::sbi::shutdown();
        }
        _ => todo!("scause: {:?}, sepc: {:#x}, stval: {:#x}", scause::read().cause(), sepc::read(), stval::read())
    }
}

