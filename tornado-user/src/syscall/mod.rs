const MODULE_PROCESS: usize = 0x114514;
const MODULE_TEST_INTERFACE: usize = 0x233666;
const MODULE_TASK: usize = 0x7777777;

const FUNC_PROCESS_EXIT: usize = 0x1919810;
const FUNC_PROCESS_PANIC: usize = 0x11451419;

const FUNC_TEST_WRITE: usize = 0x666233;
const FUNC_TEST_WRITE_ONE: usize = 0x444555;
const FUNC_TEST_READ_ONE: usize = 0x999888;
const FUNC_TEST_READ_LINE: usize = 0x11117777;

const FUNC_SWITCH_TASK: usize = 0x666666;
const FUNC_IO_TASK: usize = 0x55555;

const BLOCK_SIZE: usize = 512;
pub struct SyscallResult {
    pub code: usize,
    pub extra: usize,
}

fn syscall_0(module: usize, func: usize) -> SyscallResult {
    match () {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        () => {
            let (code, extra);
            unsafe {
                asm!(
                    "ecall",
                    in("a6") func, in("a7") module,
                    lateout("a0") code, lateout("a1") extra,
                )
            };
            SyscallResult { code, extra }
        }
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        () => {
            drop((module, func));
            unimplemented!("not RISC-V instruction set architecture")
        }
    }
}

fn syscall_1(module: usize, func: usize, arg: usize) -> SyscallResult {
    match () {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        () => {
            let (code, extra);
            unsafe {
                asm!(
                    "ecall",
                    in("a0") arg,
                    in("a6") func, in("a7") module,
                    lateout("a0") code, lateout("a1") extra,
                )
            };
            SyscallResult { code, extra }
        }
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        () => {
            drop((module, func, arg));
            unimplemented!("not RISC-V instruction set architecture")
        }
    }
}

fn syscall_2(module: usize, func: usize, args: [usize; 2]) -> SyscallResult {
    match () {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        () => {
            let (code, extra);
            unsafe {
                asm!(
                    "ecall",
                    in("a0") args[0], in("a1") args[1],
                    in("a6") func, in("a7") module,
                    lateout("a0") code, lateout("a1") extra,
                )
            };
            SyscallResult { code, extra }
        }
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        () => {
            drop((module, func, args));
            unimplemented!("not RISC-V instruction set architecture")
        }
    }
}

fn syscall_3(module: usize, func: usize, args: [usize; 3]) -> SyscallResult {
    match () {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        () => {
            let (code, extra);
            unsafe {
                asm!(
                    "ecall",
                    in("a0") args[0], in("a1") args[1], in("a2") args[2],
                    in("a6") func, in("a7") module,
                    lateout("a0") code, lateout("a1") extra,
                )
            };
            SyscallResult { code, extra }
        }
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        () => {
            drop((module, func, args));
            unimplemented!("not RISC-V instruction set architecture")
        }
    }
}

fn syscall_4(module: usize, func: usize, args: [usize; 4]) -> SyscallResult {
    match () {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        () => {
            let (code, extra);
            unsafe {
                asm!(
                    "ecall",
                    in("a0") args[0], in("a1") args[1], in("a2") args[2], in("a3") args[3],
                    in("a6") func, in("a7") module,
                    lateout("a0") code, lateout("a1") extra,
                )
            };
            SyscallResult { code, extra }
        }
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
            unsafe {
                asm!(
                    "ecall",
                    in("a0") args[0], in("a1") args[1], in("a2") args[2],
                    in("a3") args[3], in("a4") args[4], in("a5") args[5],
                    in("a6") func, in("a7") module,
                    lateout("a0") code, lateout("a1") extra,
                )
            };
            SyscallResult { code, extra }
        }
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        () => {
            drop((module, func, args));
            unimplemented!("not RISC-V instruction set architecture")
        }
    }
}

pub fn sys_exit(exit_code: i32) -> SyscallResult {
    syscall_1(MODULE_PROCESS, FUNC_PROCESS_EXIT, exit_code as usize)
}

pub fn sys_panic(
    file_name: Option<&str>,
    line: u32,
    col: u32,
    msg: Option<&str>,
) -> SyscallResult {
    let (f_buf, f_len) = file_name
        .map(|s| (s.as_ptr() as usize, s.len()))
        .unwrap_or((0, 0));
    let (m_buf, m_len) = msg
        .map(|s| (s.as_ptr() as usize, s.len()))
        .unwrap_or((0, 0));
    syscall_6(
        MODULE_PROCESS,
        FUNC_PROCESS_PANIC,
        [line as usize, col as usize, f_buf, f_len, m_buf, m_len],
    )
}

pub fn sys_yield(next_asid: usize) -> SyscallResult {
    syscall_1(MODULE_TASK, FUNC_SWITCH_TASK, next_asid)
}

pub fn sys_test_write(buf: &[u8]) -> SyscallResult {
    syscall_3(
        MODULE_TEST_INTERFACE,
        FUNC_TEST_WRITE,
        [0, buf.as_ptr() as usize, buf.len()],
    )
}

pub fn sys_test_write_one(data: usize) -> SyscallResult {
    syscall_2(
        MODULE_TEST_INTERFACE,
        FUNC_TEST_WRITE_ONE,
        [0, data],
    )
}

pub fn sys_test_read_one() -> SyscallResult {
    syscall_1(
        MODULE_TEST_INTERFACE,
        FUNC_TEST_READ_ONE,
        0,
    )
}

pub fn sys_test_read_line(buf: &mut [u8]) -> SyscallResult {
    syscall_3(
        MODULE_TEST_INTERFACE,
        FUNC_TEST_READ_LINE,
        [0, buf.as_ptr() as usize, buf.len()],
    )
}

/// 往内核注册一个
pub fn sys_enroll_read(block_id: usize, buf: &mut [u8]) -> SyscallResult {
    assert!(buf.len() == BLOCK_SIZE);
    // 第一个参数 0 表示读块设备
    syscall_3(MODULE_TASK, FUNC_IO_TASK, [0, block_id, buf.as_ptr() as usize])
}

pub fn sys_enroll_write(block_id: usize, buf: &[u8]) -> SyscallResult {
    assert!(buf.len() == BLOCK_SIZE);
    // 第一个参数 1 表示写块设备
    syscall_3(MODULE_TASK, FUNC_IO_TASK, [1, block_id, buf.as_ptr() as usize])
}