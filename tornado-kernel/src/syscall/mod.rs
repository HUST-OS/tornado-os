// syscall-exit

const MODULE_PROCESS: usize = 0x114514;

pub enum SyscallResult {
    Procceed { code: usize, extra: usize },
    Retry,
}

impl SyscallResult {
    fn ok(extra: usize) -> Self {
        SyscallResult::Procceed { code: 0, extra }
    }
}

pub fn syscall(param: [usize; 2], function: usize, module: usize) -> SyscallResult {
    match module {
        MODULE_PROCESS => SyscallResult::ok(0x1919810),
        // 完整的设计里，应该退出进程，然后完成剩下的事情
        _ => panic!("Unknown module {:x}", module),
    }
}
