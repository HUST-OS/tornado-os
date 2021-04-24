#![no_std]
#![feature(llvm_asm)]
#![feature(asm)]
#![feature(panic_info_message)]
#![feature(linkage)]
#![feature(alloc_error_handler)]

extern crate alloc;

pub mod shared;
pub mod task;

use buddy_system_allocator::LockedHeap;


const USER_HEAP_SIZE: usize = 32768;

pub static mut ADDRESS_SPACE_ID: usize = 0;
pub static mut SHARED_PAYLOAD_BASE: usize = 0;

static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

#[cfg_attr(not(test), panic_handler)]
pub fn panic_handler(_panic_info: &core::panic::PanicInfo) -> ! {
    sys_panic();
    unreachable!()
}

#[cfg_attr(not(test), alloc_error_handler)]
pub fn handle_alloc_error(_layout: core::alloc::Layout) -> ! {
    sys_panic();
    unreachable!()
}

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    let mut ret: usize;
    unsafe {
        // 从 gp 寄存器里面取出 shared_raw_table 的地址
        asm!("mv {}, gp", out(reg) ret, options(nomem, nostack));
        SHARED_PAYLOAD_BASE = ret;
        // 从 tp 寄存器里面取出该用户态的地址空间编号
        asm!("mv {}, tp", out(reg) ret, options(nomem, nostack));
        ADDRESS_SPACE_ID = ret;
        extern "C" {
            fn sbss(); fn ebss();
        }
        r0::zero_bss(&mut sbss as *mut _ as *mut u32, &mut ebss as *mut _ as *mut u32);
        HEAP.lock().init(HEAP_SPACE.as_ptr() as usize, USER_HEAP_SIZE);
    }
    main()
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> ! {
    panic!("Can not find main!");
}

use syscall::*;

pub fn exit(exit_code: i32) -> SyscallResult { sys_exit(exit_code) }
pub fn do_yield(next_asid: usize) -> SyscallResult { sys_yield(next_asid) }
pub fn test_write(buf: &[u8]) -> SyscallResult { unsafe { sys_test_write(ADDRESS_SPACE_ID, buf) }}
mod syscall {
    const MODULE_PROCESS: usize = 0x114514;
    const MODULE_TEST_INTERFACE: usize = 0x233666;
    const MODULE_TASK: usize = 0x7777777;
    
    const FUNC_PROCESS_EXIT: usize = 0x1919810;
    const FUNC_PROCESS_PANIC: usize = 0x11451419;

    const FUNC_TEST_WRITE: usize = 0x666233;
    pub struct SyscallResult {
        pub code: usize,
        pub extra: usize
    }
    
    fn syscall_0(module: usize, func: usize) -> SyscallResult {
        match () {
            #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
            () => {
                let (code, extra);
                unsafe { asm!(
                    "ecall", 
                    in("a6") func, in("a7") module,
                    lateout("a0") code, lateout("a1") extra,
                ) };
                SyscallResult { code, extra }
            },
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
                unsafe { asm!(
                    "ecall", 
                    in("a0") arg,
                    in("a6") func, in("a7") module,
                    lateout("a0") code, lateout("a1") extra,
                ) };
                SyscallResult { code, extra }
            },
            #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
            () => {
                drop((module, func, arg));
                unimplemented!("not RISC-V instruction set architecture")
            }
        }
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

    fn syscall_4(module: usize, func: usize, args: [usize; 4]) -> SyscallResult {
        match () {
            #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
            () => {
                let (code, extra);
                unsafe { asm!(
                    "ecall", 
                    in("a0") args[0], in("a1") args[1], in("a2") args[2], in("a3") args[3],
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

    pub fn sys_exit(exit_code: i32) -> SyscallResult {
        syscall_1(MODULE_PROCESS, FUNC_PROCESS_EXIT, exit_code as usize) // 暂时放着，写法不规范
    }

    pub fn sys_panic() -> SyscallResult {
        syscall_0(MODULE_PROCESS, FUNC_PROCESS_PANIC)
    }
    
    pub fn sys_yield(next_asid: usize) -> SyscallResult {
        todo!()
    }

    pub fn sys_test_write(asid: usize, buf: &[u8]) -> SyscallResult {
        syscall_4(MODULE_TEST_INTERFACE, FUNC_TEST_WRITE, [asid, 0, buf.as_ptr() as usize, buf.len()])
    }
}

#[macro_use]
pub mod console {
    use core::fmt::{self, Write};
    use super::test_write;

    struct Stdout;
    
    impl Write for Stdout {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            test_write(s.as_bytes());
            Ok(())
        }
    }

    pub fn print(args: fmt::Arguments) {
        Stdout.write_fmt(args).unwrap();
    }

    #[macro_export]
    macro_rules! print {
        ($fmt: literal $(, $($arg: tt)+)?) => {
            $crate::console::print(format_args!($fmt $(, $($arg)+)?));
        }
    }
    
    #[macro_export]
    macro_rules! println {
        ($fmt: literal $(, $($arg: tt)+)?) => {
            $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
        }
    }
}