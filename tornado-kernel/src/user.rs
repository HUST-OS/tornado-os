use riscv::register::scause::{self, Trap, Interrupt};
use riscv::register::sepc;
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
    }
    // println!("_test_user_trap: {:#x}, _test_user_entry: {:#x}", _test_user_trap as usize, _test_user_entry as usize);
   
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
    // println!("swap_cx_ppn: {:x?}", swap_cx_ppn);
    // 将物理页号转换为裸指针
    let swap_cx = unsafe { (swap_cx_ppn.start_address().0.wrapping_add(memory::KERNEL_MAP_OFFSET) as *mut trap::SwapContext).as_mut().unwrap() };
    // println!("swap_cx_va: {:#x}, swap_cx_pa: {:#x}", memory::SWAP_CONTEXT_VA, swap_cx_ppn.start_address().0.wrapping_add(memory::KERNEL_MAP_OFFSET));
    // 用户态程序入口，这里将映射到的物理页号打印出来
    let user_entry_va = memory::VirtualAddress(0);
    let user_entry_vpn = memory::VirtualPageNumber::floor(user_entry_va);
    let user_entry_ppn = user_memory.mapping
        .translate(user_entry_vpn)
        .unwrap()
        .page_number();
    println!("user_entry_ppn: {:x?}", user_entry_ppn);
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
    // _test_user_entry 由虚拟地址 0 映射到真实物理地址
    *swap_cx = trap::SwapContext::new_to_user(
        kernel_satp, 0, 0, kernel_stack_top, user_stack_top, _test_user_trap as usize);
    // println!("swap_cx.epc: {:#x}", swap_cx.epc);
    // println!("swap_cx.trap_handler: {:#x}", swap_cx.user_trap_handler);
    trap::switch_to_user(swap_cx, user_satp)
}

// 测试用的中断处理函数，用户态发生中断会陷入到这里
#[export_name = "_test_user_trap"]
pub extern "C" fn test_user_trap() {
    match scause::read().cause() {
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            println!("s mode timer!");
            // 目前遇到时钟中断先让系统退出，等把内核进程队列完善好了再来处理
            crate::sbi::shutdown();
        },
        _ => todo!("scause: {:?}, sepc: {:#x}", scause::read().cause(), sepc::read())
    }
}

// 测试用的用户程序入口
#[export_name = "_test_user_entry"]
#[link_section = ".user_text"]
pub extern "C" fn test_user_entry() {
    let mut data = [0usize; 500];
    data.iter_mut().for_each(|i| *i += 1);
    loop {}
}

// 用户态数据，这里是为了占位
#[export_name = "_user_data"]
#[link_section = ".user_data"]
static _USER_STACK: [usize; PAGE_SIZE / 8] = [1; PAGE_SIZE / 8];