#![no_std]
#![feature(llvm_asm)]
#![feature(asm)]
#![feature(panic_info_message)]
#![feature(linkage)]
#![feature(alloc_error_handler)]
#![feature(maybe_uninit_uninit_array)]

extern crate alloc;

#[macro_use]
pub mod console;
pub mod io;
pub mod syscall;
pub mod task;
pub use console::{stdin, Stdin};

use buddy_system_allocator::LockedHeap;
use core::future::Future;

const USER_HEAP_SIZE: usize = 128 * 1024; // 1M

static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

static mut SHARED_PAYLOAD_BASE: usize = 0;
static mut ADDRESS_SPACE_ID: usize = 0;

#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

#[cfg_attr(not(test), panic_handler)]
pub fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    let err = panic_info.message().unwrap().as_str();
    if let Some(location) = panic_info.location() {
        syscall::sys_panic(
            Some(location.file()),
            location.line(),
            location.column(),
            err,
        );
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
        fn sbss();
        fn ebss();
    }
    unsafe {
        r0::zero_bss(
            &mut sbss as *mut _ as *mut u32,
            &mut ebss as *mut _ as *mut u32,
        );
        HEAP.lock()
            .init(HEAP_SPACE.as_ptr() as usize, USER_HEAP_SIZE);
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
    let shared_payload = unsafe { task::shared::SharedPayload::new(SHARED_PAYLOAD_BASE) };
    let address_space_id = unsafe { task::shared::AddressSpaceId::from_raw(ADDRESS_SPACE_ID) };
    static mut EXIT_CODE: i32 = 0;
    let main_task = task::new_user(
        async move {
            unsafe { EXIT_CODE = main.await };
        },
        shared_payload.shared_scheduler,
        shared_payload.shared_set_task_state,
    );
    unsafe {
        shared_payload.add_task(hart_id, address_space_id, main_task.task_repr());
    }
    task::shared::run_until_ready(
        || unsafe { shared_payload.peek_task(task::shared::user_should_switch) },
        |task_repr| unsafe { shared_payload.delete_task(task_repr) },
        |task_repr, new_state| unsafe { shared_payload.set_task_state(task_repr, new_state) },
    );
    unsafe { EXIT_CODE }
}

/// 生成一个新的任务
pub fn spawn(future: impl Future<Output = ()> + Send + Sync + 'static) {
    let shared_payload = unsafe { task::shared::SharedPayload::new(SHARED_PAYLOAD_BASE) };
    let asid = unsafe { task::shared::AddressSpaceId::from_raw(ADDRESS_SPACE_ID) };
    let task = task::new_user(
        future,
        shared_payload.shared_scheduler,
        shared_payload.shared_set_task_state,
    );
    unsafe {
        shared_payload.add_task(0 /* todo */, asid, task.task_repr());
    }
}

/// 运行异步任务
pub fn execute_async() {
    let shared_payload = unsafe { task::shared::SharedPayload::new(SHARED_PAYLOAD_BASE) };
    task::shared::run_until_ready(
        || unsafe { shared_payload.peek_task(task::shared::user_should_switch) },
        |task_repr| unsafe { shared_payload.delete_task(task_repr) },
        |task_repr, new_state| unsafe { shared_payload.set_task_state(task_repr, new_state) },
    );
}

use syscall::*;

pub fn exit(exit_code: i32) -> SyscallResult {
    sys_exit(exit_code)
}
pub fn do_yield(next_asid: usize) -> SyscallResult {
    sys_yield(next_asid)
}
pub fn test_write(buf: &[u8]) -> SyscallResult {
    sys_test_write(buf)
}
