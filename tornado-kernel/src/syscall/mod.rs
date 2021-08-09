// syscall-exit

mod config;
mod user_syscall;

use crate::hart::KernelHartInfo;
use crate::memory::{AddressSpaceId, Satp, VirtualAddress, VirtualPageNumber};
use bit_field::BitField;
use config::*;
pub use user_syscall::get_swap_cx;
pub use user_syscall::user_trap_handler;
pub use user_syscall::WAKE_NUM;

pub enum SyscallResult {
    Procceed { code: usize, extra: usize },
    Retry,
    NextASID { asid: usize, satp: Satp },
    KernelTask,
    IOTask { block_id: usize, buf_ptr: usize, write: bool},
    Check,
    Terminate(i32),
}

impl SyscallResult {
    fn ok(extra: usize) -> Self {
        SyscallResult::Procceed { code: 0, extra }
    }
}

pub fn syscall(param: [usize; 6], user_satp: usize, func: usize, module: usize) -> SyscallResult {
    match module {
        MODULE_PROCESS => do_process(param, user_satp, func),
        MODULE_TEST_INTERFACE => do_test_interface(param, user_satp, func),
        MODULE_TASK => do_task(param, func),
        _ => panic!("Unknown module {:x}", module),
    }
}

fn do_task(param: [usize; 6], func: usize) -> SyscallResult {
    match func {
        FUNC_SWITCH_TASK => switch_next_task(param[0]),
        FUNC_IO_TASK => do_io_task(param[0], param[1], param[2]),
        FUNC_CHECK => do_check(),
        _ => unimplemented!(),
    }
}

/// 用户态轮询任务的时候，发现下一个任务在不同地址空间，则产生该系统调用
/// 从共享调度器里面拿出下一个任务的引用，根据地址空间编号切换到相应的地址空间
/// 下一个任务的地址空间编号由用户通过 a0 参数传给内核
fn switch_next_task(next_asid: usize) -> SyscallResult {
    if next_asid == 0 {
        // 内核任务
        SyscallResult::KernelTask
    } else {
        let satp = KernelHartInfo::user_satp(next_asid).expect("get satp register with asid");
        SyscallResult::NextASID {
            asid: next_asid,
            satp,
        }
    }
}

fn do_io_task(io_type: usize, block_id: usize, buf_ptr: usize) -> SyscallResult {
    match io_type {
        0 => SyscallResult::IOTask {
            block_id, buf_ptr, write: false
        },
        1 => SyscallResult::IOTask {
            block_id, buf_ptr, write: true
        },
        _ => panic!("unknown io type")
    }
}

fn do_check() -> SyscallResult {
    SyscallResult::Check
}

fn do_process(param: [usize; 6], user_satp: usize, func: usize) -> SyscallResult {
    match func {
        FUNC_PROCESS_EXIT => SyscallResult::Terminate(param[0] as i32),
        FUNC_PROCESS_PANIC => {
            //[line as usize, col as usize, f_buf, f_len, m_buf, m_len]
            let [line, col, f_buf, f_len, m_buf, m_len] = param;
            let file_name = if f_buf == 0 {
                None
            } else {
                let slice = unsafe { get_user_buf(user_satp, f_buf, f_len) };
                Some(core::str::from_utf8(slice).unwrap())
            };
            let msg = if m_buf == 0 {
                None
            } else {
                let slice = unsafe { get_user_buf(user_satp, m_buf, m_len) };
                Some(core::str::from_utf8(slice).unwrap())
            };
            let file_name = file_name.unwrap_or("<no file>");
            let msg = msg.unwrap_or("<no message>");
            println!(
                "[Kernel] User process panicked at '{}', {}:{}:{}",
                msg, file_name, line, col
            );
            SyscallResult::Terminate(-1)
        }
        _ => panic!(
            "Unknown syscall process, func: {}, param: {:?}",
            func, param
        ),
    }
}

fn do_test_interface(param: [usize; 6], user_satp: usize, func: usize) -> SyscallResult {
    match func {
        FUNC_TEST_WRITE => {
            let (_iface, buf_ptr, buf_len) = (param[0], param[1], param[2]); // 调试接口编号，缓冲区指针，缓冲区长度
            let slice = unsafe { get_user_buf(user_satp, buf_ptr, buf_len) };
            for &byte in slice {
                crate::sbi::console_putchar(byte as usize);
            }
            SyscallResult::Procceed {
                code: 0,
                extra: buf_len,
            }
        }
        FUNC_TEST_WRITE_ONE => { // 写一个字符
            let (_iface, value) = (param[0], param[1]); // 调试接口编号
            crate::sbi::console_putchar(value);
            SyscallResult::Procceed {
                code: 0,
                extra: 1, // 写了一个字符
            }
        }
        FUNC_TEST_READ_ONE => { // 读一个字符
            let _iface = param[0]; // 调试接口编号
            let input = crate::sbi::console_getchar();
            SyscallResult::Procceed {
                code: 0,
                extra: input, // 返回读出的一个字符结果
            }
        }
        FUNC_TEST_READ_LINE => {
            // 读入len个字符，如果遇到换行符，或者缓冲区满，就停止
            println!("[kernel syscall] Read line {} {:x} {}", param[0], param[1], param[2]);
            let (_iface, buf_ptr, buf_len) = (param[0], param[1], param[2]); // 调试接口编号，输出缓冲区指针，输出缓冲区长度
            let slice = unsafe { get_user_buf_mut(user_satp, buf_ptr, buf_len) };
            for i in 0..buf_len {
                let input = crate::sbi::console_getchar();
                println!("[syscall] input = {}", input);
                let byte = input as u8; // 假定SBI输入都是u8类型
                if byte == b'\n' {
                    break;
                }
                slice[i] = byte;
            }
            SyscallResult::Procceed {
                code: 0,
                extra: buf_len,
            }
        }
        _ => panic!("Unknown syscall test, func: {}, param: {:?}", func, param),
    }
}

unsafe fn get_user_buf<'a>(user_satp: usize, buf_ptr: usize, buf_len: usize) -> &'a [u8] {
    get_user_buf_mut(user_satp, buf_ptr, buf_len)
}

unsafe fn get_user_buf_mut<'a>(user_satp: usize, buf_ptr: usize, buf_len: usize) -> &'a mut [u8] {
    let user_satp = Satp(user_satp);
    let offset = buf_ptr.get_bits(0..12); // Sv39 里面虚拟地址偏移量为低 12 位
    let vpn = VirtualPageNumber::floor(VirtualAddress(buf_ptr));
    let ppn = user_satp.translate(vpn).expect("no page fault");
    let va = ppn
        .start_address()
        .virtual_address_linear()
        .0
        .wrapping_add(offset);
    let ptr = (va as *const u8).as_ref().expect("non-null pointer");
    core::slice::from_raw_parts_mut(ptr as *const _ as *mut _, buf_len)
}
