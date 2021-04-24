// syscall-exit

mod config;
mod user_syscall;

pub use user_syscall::user_trap_handler;

use config::*;

use crate::{hart::KernelHartInfo, memory::{AddressSpaceId, Satp}};
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

pub fn syscall(param: [usize; 2], func: usize, module: usize) -> SyscallResult {
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
fn switch_next_task(param: [usize; 2], func: usize) -> SyscallResult {
    let next_asid = unsafe { AddressSpaceId::from_raw(param[0]) }; // a0
    todo!()
}

fn do_process(param: [usize; 2], func: usize) -> SyscallResult {
    match func {
        FUNC_PROCESS_EXIT => SyscallResult::Terminate(param[0] as i32),
        FUNC_PROCESS_PANIC => panic!("User panic!"),
        _ => panic!("Unknown syscall process, func: {}, param: {:?}", func, param)
    }
}

fn do_test_interfase(param: [usize; 2], func: usize) -> SyscallResult {
    todo!()
}

fn do_task(param: [usize; 2], func: usize) -> SyscallResult {
    todo!()
}