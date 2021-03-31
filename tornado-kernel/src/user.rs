/// 临时的用户态程序和数据

use crate::memory;
use crate::trap;
use crate::task;

// 尝试进入用户态
pub fn try_enter_user() -> ! {
    extern {
        // 用户陷入内核时候的中断处理函数
        fn _test_user_trap();
        // 用户程序入口点
        fn _test_user_entry();
    }
    // 创建一个用户态映射
    let user_memory = memory::MemorySet::new_user().unwrap();
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
    println!("swap_cx_va: {:#x}, swap_cx_pa: {:#x}", memory::SWAP_CONTEXT_VA, swap_cx_ppn.start_address().0.wrapping_add(memory::KERNEL_MAP_OFFSET));
    // 用户态程序入口，这里将映射到的物理页号打印出来方便调试
    let user_entry_va = memory::VirtualAddress(0);
    let user_entry_vpn = memory::VirtualPageNumber::floor(user_entry_va);
    let user_entry_ppn = user_memory.mapping
        .translate(user_entry_vpn)
        .unwrap()
        .page_number();
    println!("user_entry_ppn: {:x?}", user_entry_ppn);
    let read_user_va: &mut [u16; 2] = unsafe {user_entry_ppn.start_address().deref_linear_static() };
    println!("code in user text:");
    for code in read_user_va {
        println!("{:#x}", &code);
    }
    let process = task::Process::new(user_memory).unwrap();
    // 获取用户的satp寄存器
    let user_satp = process.satp();
    // 获取内核的satp寄存器
    let kernel_satp = riscv::register::satp::read().bits();
    
    println!("_test_user_trap: {:#x}, _test_user_entry: {:#x}", _test_user_trap as usize, _test_user_entry as usize);
    
    // 往 SwapContext 写东西
    // _test_user_entry 由虚拟地址 0 映射到真实物理地址
    *swap_cx = trap::SwapContext::new_to_user(
        kernel_satp, 0, 0, 0, 0, _test_user_trap as usize);
    println!("swap_cx.epc: {:#x}", swap_cx.epc);
    println!("swap_cx.trap_handler: {:#x}", swap_cx.user_trap_handler);
    trap::switch_to_user(swap_cx, user_satp)
}

// 测试用的中断处理函数，用户态发生中断会陷入到这里
#[export_name = "_test_user_trap"]
pub extern "C" fn test_user_trap() {
    println!("trap from user");
    loop {}
}

// 测试用的用户程序入口
#[export_name = "_test_user_entry"]
#[link_section = ".user_text"]
pub extern "C" fn test_user_entry() {
    // unsafe { asm!("ecall"); }
    loop {}
}