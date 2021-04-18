// syscall-exit

mod config;
mod user_syscall;

use config::*;

use crate::{hart::KernelHartInfo, memory::{AddressSpaceId, Satp}};
pub enum SyscallResult {
    Procceed { code: usize, extra: usize },
    Retry,
    NextASID { satp: Satp }
}

impl SyscallResult {
    fn ok(extra: usize) -> Self {
        SyscallResult::Procceed { code: 0, extra }
    }
}

pub fn syscall(param: [usize; 2], func: usize, module: usize) -> SyscallResult {
    match module {
        MODULE_PROCESS => SyscallResult::ok(0x1919810),
        SWITCH_TASK => {
            // 切换任务，需要切换地址空间
            switch_next_task(param, func)
        },
        USER_EXIT => {
            println!("user exit");
            crate::sbi::shutdown();
        }
        // 完整的设计里，应该退出进程，然后完成剩下的事情
        _ => panic!("Unknown module {:x}", module),
    }
}

/// 用户态轮询任务的时候，发现下一个任务在不同地址空间，则产生该系统调用
/// 从共享调度器里面拿出下一个任务的引用，根据地址空间编号切换到相应的地址空间
/// 下一个任务的地址空间编号由用户通过 a0 参数传给内核
fn switch_next_task(param: [usize; 2], func: usize) -> SyscallResult {
    let next_asid = unsafe { AddressSpaceId::from_raw(param[0]) }; // a0
    if next_asid.into_inner() == 0 {
        // 内核任务，这里为了测试，不执行，直接回到用户态
        let raw_table_ptr = 0x8600_0000 as *const ();
        let raw_table: extern "C" fn(a0: usize) -> usize = unsafe { core::mem::transmute(raw_table_ptr) };
        let shared_scheduler_ptr = raw_table(0);
        let shared_scheduler: fn()  -> core::ptr::NonNull<()> = unsafe {
            core::mem::transmute(shared_scheduler_ptr)
        };
        let shared_scheduler = shared_scheduler();
        let shared_pop_task_ptr = raw_table(2);
        let shared_pop_task: unsafe fn(
            shared_scheduler: core::ptr::NonNull<()>,
            should_switch: fn(&crate::task::SharedTaskHandle) -> bool
        ) -> crate::task::TaskResult = unsafe {
            core::mem::transmute(shared_pop_task_ptr)
        };
        unsafe { shared_pop_task(shared_scheduler, crate::task::SharedTaskHandle::should_switch); }
        return SyscallResult::Procceed{ code: 0, extra: 0};
    }
    if let Some(next_satp) = KernelHartInfo::get_satp(next_asid) {
        SyscallResult::NextASID{ satp: next_satp}
    } else {
        panic!("Next satp not found!")
    }
}