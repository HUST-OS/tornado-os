// syscall-exit

mod config;
mod user_syscall;

use config::*;
pub enum SyscallResult {
    Procceed { code: usize, extra: usize },
    Retry,
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
    let next_asid = param[0]; // a0
    
    SyscallResult::ok(next_asid)
}