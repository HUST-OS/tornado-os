// syscall-exit

mod config;
mod user_syscall;

pub use user_syscall::user_trap_handler;

use config::*;

use crate::{
    hart::KernelHartInfo,
    memory::{
        AddressSpaceId,
        Satp,
        VirtualPageNumber,
        VirtualAddress,
        PhysicalPageNumber,
        KERNEL_MAP_OFFSET
    }
};

use bit_field::BitField;

pub enum SyscallResult {
    Procceed { code: usize, extra: usize },
    Retry,
    NextASID { satp: Satp },
    Terminate(i32),
}

impl SyscallResult {
    fn ok(extra: usize) -> Self {
        SyscallResult::Procceed { code: 0, extra }
    }
}

pub fn syscall(param: [usize; 6], func: usize, module: usize) -> SyscallResult {
    match module {
        MODULE_PROCESS => do_process(param, func),
        MODULE_TEST_INTERFACE => do_test_interfase(param, func),
        MODULE_TASK => do_task(param, func),
        _ => panic!("Unknown module {:x}", module),
    }
}

/// 用户态轮询任务的时候，发现下一个任务在不同地址空间，则产生该系统调用
/// 从共享调度器里面拿出下一个任务的引用，根据地址空间编号切换到相应的地址空间
/// 下一个任务的地址空间编号由用户通过 a0 参数传给内核
fn switch_next_task(param: [usize; 6], func: usize) -> SyscallResult {
    let next_asid = unsafe { AddressSpaceId::from_raw(param[0]) }; // a0
    todo!()
}

fn do_process(param: [usize; 6], func: usize) -> SyscallResult {
    match func {
        FUNC_PROCESS_EXIT => SyscallResult::Terminate(param[0] as i32),
        FUNC_PROCESS_PANIC => panic!("User panic!"),
        _ => panic!("Unknown syscall process, func: {}, param: {:?}", func, param)
    }
}

fn do_test_interfase(param: [usize; 6], func: usize) -> SyscallResult {
    match func {
        FUNC_TEST_WRITE => {
            let (asid, _fd, buf_ptr, buf_len) =
                (param[0], param[1], param[2], param[3]); // 地址空间参数，文件描述符，缓冲区指针，缓冲区长度
            // println!("[kernel] enter do_test_write with asid: {}, buf_ptr: {:#x}, buf_len: {}", asid, buf_ptr, buf_len);
            let user_asid = unsafe { AddressSpaceId::from_raw(asid) };
            if let Some(user_satp) = KernelHartInfo::get_satp(user_asid) {
                let offset = buf_ptr.get_bits(0..12); // Sv39 里面虚拟地址偏移量为低 12 位
                let vpn = VirtualPageNumber::floor(VirtualAddress(buf_ptr));
                let ppn = user_satp.translate(vpn).unwrap();
                unsafe {
                    let ptr = (ppn
                        .start_address()
                        .0
                        .wrapping_add(KERNEL_MAP_OFFSET)
                        .wrapping_add(offset) as *const u8)
                        .as_ref()
                        .unwrap();
                    let slice = core::slice::from_raw_parts(ptr, buf_len);
                    let str = core::str::from_utf8(slice).unwrap();
                    print!("{}", str);
                    SyscallResult::Procceed {code: 0, extra: buf_len}
                }
            } else {
                panic!("User asid not found!")
            }
        },
        _ => panic!("Unknown syscall test, func: {}, param: {:?}", func, param)
    }
}

fn do_task(param: [usize; 6], func: usize) -> SyscallResult {
    todo!()
}