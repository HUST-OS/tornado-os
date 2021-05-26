#![no_std]
#![feature(llvm_asm)]
#![feature(asm)]
#![feature(panic_info_message)]
#![feature(linkage)]
#![feature(alloc_error_handler)]

extern crate alloc;

#[macro_use]
pub mod console;
pub mod shared;
pub mod task;

use buddy_system_allocator::LockedHeap;
use core::future::Future;

const USER_HEAP_SIZE: usize = 64 * 1024;

static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

static mut SHARED_PAYLOAD_BASE: usize = 0;
static mut ADDRESS_SPACE_ID: usize = 0;

#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

#[cfg_attr(not(test), panic_handler)]
pub fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    let err = panic_info.message().unwrap().as_str();
    if let Some(location) = panic_info.location() {
        syscall::sys_panic(Some(location.file()), location.line(), location.column(), err);
    } else {
        syscall::sys_panic(None, 0, 0, err);
    }
    unreachable!()
}

#[cfg_attr(not(test), alloc_error_handler)]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    println!("[User] user alloc error, layout = {:?}", layout);
    panic!("user alloc error: {:?}", layout)
}

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    let mut address_space_id: usize;
    let mut shared_payload_base: usize;
    unsafe {
        // 从 gp 寄存器里面取出 shared_raw_table 的地址
        asm!("mv {}, gp", out(reg) shared_payload_base, options(nomem, nostack));
        SHARED_PAYLOAD_BASE = shared_payload_base;
        // 从 tp 寄存器里面取出该用户态的地址空间编号
        asm!("mv {}, tp", out(reg) address_space_id, options(nomem, nostack));
        ADDRESS_SPACE_ID = address_space_id;
    }
    extern "C" {
        fn sbss(); fn ebss();
    } 
    unsafe { 
        r0::zero_bss(&mut sbss as *mut _ as *mut u32, &mut ebss as *mut _ as *mut u32);
        HEAP.lock().init(HEAP_SPACE.as_ptr() as usize, USER_HEAP_SIZE);
    }
    let exit_code = main();
    exit(exit_code);
    unreachable!()
}

#[linkage = "weak"]
#[link_section = ".text"] // 必须指定，否则llvm好像会把名字为“entry”的函数链接到最开始……
#[no_mangle]
fn main() -> i32 {
    println!("[User] No main function found; user exit");
    panic!("Can not find main!");
}

/// 运行一个异步的main函数，在用户的entry函数里调用
/// 应该作为标准库的一部分，这里使用一个库函数来模拟有标准库的情况
pub fn execute_async_main(main: impl Future<Output = i32> + Send + Sync + 'static) -> i32 {
    let hart_id = 0; // todo!
    let shared_payload = unsafe { shared::SharedPayload::new(SHARED_PAYLOAD_BASE) };
    let address_space_id = unsafe { shared::AddressSpaceId::from_raw(ADDRESS_SPACE_ID) };
    static mut EXIT_CODE: i32 = 0;
    let main_task = task::new_user(async move {
        unsafe { EXIT_CODE = main.await };
    }, shared_payload.shared_scheduler, shared_payload.shared_set_task_state);
    unsafe {
        shared_payload.add_task(hart_id, address_space_id, main_task.task_repr());
    }
    shared::run_until_ready(
        || unsafe { shared_payload.peek_task(shared::user_should_switch) },
        |task_repr| unsafe { shared_payload.delete_task(task_repr) },
        |task_repr, new_state| unsafe { shared_payload.set_task_state(task_repr, new_state)}
    );
    unsafe { EXIT_CODE }
}

/// 生成一个新的任务
pub fn spawn(future: impl Future<Output = ()> + Send + Sync + 'static) {
    let shared_payload = unsafe { shared::SharedPayload::new(SHARED_PAYLOAD_BASE) };
    let asid = unsafe { shared::AddressSpaceId::from_raw(ADDRESS_SPACE_ID) };
    let task = task::new_user(future, shared_payload.shared_scheduler, shared_payload.shared_set_task_state);
    unsafe {
        shared_payload.add_task(0/* todo */, asid, task.task_repr());
    }
}

use syscall::*;

pub fn exit(exit_code: i32) -> SyscallResult { sys_exit(exit_code) }
pub fn do_yield(next_asid: usize) -> SyscallResult { sys_yield(next_asid) }
pub fn test_write(buf: &[u8]) -> SyscallResult { sys_test_write(buf) }
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
        syscall_1(MODULE_PROCESS, FUNC_PROCESS_EXIT, exit_code as usize)
    }

    pub fn sys_panic(file_name: Option<&str>, line: u32, col: u32, msg: Option<&str>) -> SyscallResult {
        let (f_buf, f_len) = file_name.map(|s| (s.as_ptr() as usize, s.len())).unwrap_or((0, 0));
        let (m_buf, m_len) = msg.map(|s| (s.as_ptr() as usize, s.len())).unwrap_or((0, 0));
        syscall_6(
            MODULE_PROCESS, FUNC_PROCESS_PANIC, 
            [line as usize, col as usize, f_buf, f_len, m_buf, m_len]
        )
    }    
    
    pub fn sys_yield(next_asid: usize) -> SyscallResult {
        todo!()
    }

    pub fn sys_test_write(buf: &[u8]) -> SyscallResult {
        syscall_3(MODULE_TEST_INTERFACE, FUNC_TEST_WRITE, [0, buf.as_ptr() as usize, buf.len()])
    }
}
