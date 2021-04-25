const MODULE_TEST_INTERFACE: usize = 0x233666;
const MODULE_PROCESS: usize = 0x114514;

const FUNC_PROCESS_PANIC: usize = 0x11451419;

const FUNC_TEST_WRITE: usize = 0x666233;

pub fn sys_panic(file_name: Option<&str>, line: u32, col: u32, msg: Option<&str>) -> SyscallResult {
    let (f_buf, f_len) = file_name.map(|s| (s.as_ptr() as usize, s.len())).unwrap_or((0, 0));
    let (m_buf, m_len) = msg.map(|s| (s.as_ptr() as usize, s.len())).unwrap_or((0, 0));
    syscall_6(
        MODULE_PROCESS, FUNC_PROCESS_PANIC, 
        [line as usize, col as usize, f_buf, f_len, m_buf, m_len]
    )
}

pub fn sys_test_write(buf: &[u8]) -> SyscallResult {
    syscall_3(MODULE_TEST_INTERFACE, FUNC_TEST_WRITE, [0, buf.as_ptr() as usize, buf.len()])
}

pub struct SyscallResult {
    pub code: usize,
    pub extra: usize
}

fn syscall_3(module: usize, func: usize, args: [usize; 3]) -> SyscallResult {
    match () {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        () => {
            let (code, extra);
            unsafe { asm!(
                "ecall", 
                in("a0") args[0], in("a1") args[1], in("a2") args[2],
                in("a6") func, in("a7") module,
                lateout("a0") code, lateout("a1") extra,
            ) };
            SyscallResult { code, extra }
        },
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        () => {
            drop((module, func, args));
            unimplemented!("not RISC-V instruction set architecture")
        }
    }
}

fn syscall_6(module: usize, func: usize, args: [usize; 6]) -> SyscallResult {
    match () {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        () => {
            let (code, extra);
            unsafe { asm!(
                "ecall", 
                in("a0") args[0], in("a1") args[1], in("a2") args[2],
                in("a3") args[3], in("a4") args[4], in("a5") args[5],
                in("a6") func, in("a7") module,
                lateout("a0") code, lateout("a1") extra,
            ) };
            SyscallResult { code, extra }
        },
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        () => {
            drop((module, func, args));
            unimplemented!("not RISC-V instruction set architecture")
        }
    }
}
