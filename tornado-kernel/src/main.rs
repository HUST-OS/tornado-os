//! 飓风内核：一种基于共享调度器的异步内核设计
//!
//! 操作系统内核经历了几个主要的发展阶段，从裸机应用，批处理系统到多道任务系统，
//! 演变为至今主流的线程操作系统。这种系统基于线程的切换来调度任务。
//! 为了进一步提升性能，一些现代编程语言在应用层服用线程资源，提出了`协程`的概念，
//! 旨在节省任务调度的开销。
//!
//! 在本项目中我们提出一种新的内核开发思路：由不同资源共享调度器，在操作系统层面提供协程。
//!
//! 我们希望这种全新设计的内核在满足传统内核的易用性的同时，拥有着专用内核的高性能特点，
//! “像风一样快”，因此取名**飓风内核**--**tornado-os**。
#![no_std]
#![no_main]
#![feature(global_asm, llvm_asm, asm, alloc_error_handler)]
#![feature(drain_filter)]
#![feature(maybe_uninit_uninit_array)]
#![feature(naked_functions)]
#![feature(maybe_uninit_ref)]
#![feature(linked_list_remove)]
#[macro_use]
extern crate alloc;

#[macro_use]
mod console;
mod algorithm;
mod cache;
mod fs;
mod hart;
mod memory;
mod panic;
mod plic;
mod sbi;
mod sdcard;
mod syscall;
mod task;
mod trap;
mod user;
mod virtio;

#[cfg(not(test))]
global_asm!(include_str!("entry.asm"));

/// qemu平台下共享调度器的基地址
#[cfg(feature = "qemu")]
const SHAREDPAYLOAD_BASE: usize = 0x8600_0000;

/// k210平台下共享调度器的基地址
#[cfg(feature = "k210")]
const SHAREDPAYLOAD_BASE: usize = 0x8040_0000;

#[no_mangle]
pub extern "C" fn rust_main(hart_id: usize) -> ! {
    extern "C" {
        static mut _sbss: u32;
        static mut _ebss: u32;

        static mut _sdata: u32;
        static mut _edata: u32;

        static _sidata: u32;

        fn _swap_frame();
        fn _user_to_supervisor();
        fn _supervisor_to_user();
    }

    unsafe {
        r0::zero_bss(&mut _sbss, &mut _ebss);
        r0::init_data(&mut _sdata, &mut _edata, &_sidata);
    }

    println!("[kernel] hart {} booted", hart_id);

    memory::init();
    trap::init();

    unsafe {
        asm!("ebreak");
    };

    // 动态内存分配测试
    use alloc::boxed::Box;
    let v = Box::new(5);
    assert_eq!(*v, 5);
    core::mem::drop(v);

    let mut vec = alloc::vec::Vec::new();
    for i in 0..10000 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10000);
    for (i, value) in vec.into_iter().enumerate() {
        assert_eq!(value, i);
    }

    println!("[kernel] heap test passed");
    
    println!("[kernel] max asid = {:?}", memory::max_asid());

    // 物理页分配
    for i in 0..2 {
        let frame_0 = match memory::frame_alloc() {
            Some(frame_tracker) => frame_tracker,
            None => panic!("frame allocation failed"),
        };
        let frame_1 = match memory::frame_alloc() {
            Some(frame_tracker) => frame_tracker,
            None => panic!("frame allocation failed"),
        };
        println!(
            "[kernel] test #{}: {:?} and {:?}",
            i,
            frame_0.start_address(),
            frame_1.start_address()
        );
    }

    println!("[kernel] _swap_frame: {:#x}", _swap_frame as usize);
    println!("[kernel] _user_to_supervisor: {:#x}", _user_to_supervisor as usize);
    println!("[kernel] _supervisor_to_user: {:#x}", _supervisor_to_user as usize);

    // 在启动程序之前，需要加载内核当前线程的信息到tp寄存器中
    unsafe { hart::KernelHartInfo::load_hart(hart_id) };
    // 这之后就可以分配地址空间了，这之前只能用内核的地址空间

    println!("[kernel] current hart: {}", hart::KernelHartInfo::hart_id());

    // todo: 这里要有个地方往tp里写东西，否则目前会出错
    let kernel_memory = memory::MemorySet::new_kernel().expect("create kernel memory set");
    kernel_memory.activate();

    #[cfg(feature = "qemu")]
    unsafe {
        plic::xv6_plic_init();
    }

    let shared_payload = unsafe { task::SharedPayload::load(SHAREDPAYLOAD_BASE) };

    // 创建一个内核进程
    let process = task::Process::new(kernel_memory).expect("create process 1");
    let hart_id = crate::hart::KernelHartInfo::hart_id();
    let address_space_id = process.address_space_id();
    // 分配一个内核栈
    let stack_handle = process.alloc_stack().expect("alloc initial stack");

    // 创建一些测试任务
    let task_1 = task::new_kernel(
        task_1(),
        process.clone(),
        shared_payload.shared_scheduler,
        shared_payload.shared_set_task_state,
    );
    let task_2 = task::new_kernel(
        task_2(),
        process.clone(),
        shared_payload.shared_scheduler,
        shared_payload.shared_set_task_state,
    );
    let task_3 = task::new_kernel(
        FibonacciFuture::new(8),
        process.clone(),
        shared_payload.shared_scheduler,
        shared_payload.shared_set_task_state,
    );
    #[cfg(feature = "qemu")]
    let task_4 = task::new_kernel(
        virtio::async_virtio_blk_test(),
        process.clone(),
        shared_payload.shared_scheduler,
        shared_payload.shared_set_task_state,
    );
    #[cfg(feature = "k210")]
    let task_4 = task::new_kernel(
        sdcard::sdcard_test(),
        process.clone(),
        shared_payload.shared_scheduler,
        shared_payload.shared_set_task_state,
    );

    // 创建一个初始化文件系统的任务
    let task_5 = task::new_kernel(
        fs::fs_init(),
        process.clone(),
        shared_payload.shared_scheduler,
        shared_payload.shared_set_task_state,
    );

    unsafe {
        shared_payload.add_task(hart_id, address_space_id, task_1.task_repr());
        shared_payload.add_task(hart_id, address_space_id, task_2.task_repr());
        shared_payload.add_task(hart_id, address_space_id, task_3.task_repr());
        shared_payload.add_task(hart_id, address_space_id, task_5.task_repr());
    }

    // 运行任务
    task::run_until_idle(
        || unsafe { shared_payload.peek_task(task::kernel_should_switch) },
        |task_repr| unsafe { shared_payload.delete_task(task_repr) },
        |task_repr, new_state| unsafe { shared_payload.set_task_state(task_repr, new_state) },
    );

    // 通过一些任务从文件系统中加载用户的二进制文件和准备用户的上下文
    let task_6 = task::new_kernel(
        user::prepare_user("yield-task0.bin", stack_handle.end.0 - 4),
        process.clone(),
        shared_payload.shared_scheduler,
        shared_payload.shared_set_task_state,
    );
    let task_7 = task::new_kernel(
        user::prepare_user("yield-task1.bin", stack_handle.end.0 - 4),
        process.clone(),
        shared_payload.shared_scheduler,
        shared_payload.shared_set_task_state,
    );
    let task_8 = task::new_kernel(
        user::prepare_user("async-read.bin", stack_handle.end.0 - 4),
        process.clone(),
        shared_payload.shared_scheduler,
        shared_payload.shared_set_task_state,
    );
    let task_9 = task::new_kernel(
        user::prepare_user("channel.bin", stack_handle.end.0 - 4),
        process.clone(),
        shared_payload.shared_scheduler,
        shared_payload.shared_set_task_state,
    );

    unsafe {
        shared_payload.add_task(hart_id, address_space_id, task_6.task_repr());
        shared_payload.add_task(hart_id, address_space_id, task_7.task_repr());
        shared_payload.add_task(hart_id, address_space_id, task_8.task_repr());
        shared_payload.add_task(hart_id, address_space_id, task_9.task_repr());
    }

    // 运行执行器
    task::run_until_idle(
        || unsafe { shared_payload.peek_task(task::kernel_should_switch) },
        |task_repr| unsafe { shared_payload.delete_task(task_repr) },
        |task_repr, new_state| unsafe { shared_payload.set_task_state(task_repr, new_state) },
    );

    // 创建一个内核任务，用于测试
    let task_10 = task::new_kernel(
        yield_kernel(),
        process.clone(),
        shared_payload.shared_scheduler,
        shared_payload.shared_set_task_state,
    );

    unsafe {
        shared_payload.add_task(hart_id, address_space_id, task_10.task_repr());
    }

    // 进入地址空间编号为 1 的用户态空间
    user::enter_user(1)
    // end()
}

#[allow(unused)]
fn end() -> ! {
    // 关机之前，卸载当前的核。虽然关机后内存已经清空，不是必要，预留未来热加载热卸载处理核的情况
    unsafe { hart::KernelHartInfo::unload_hart() };
    // 没有任务了，关机
    sbi::shutdown()
}

async fn task_1() {
    println!("hello world from 1!");
}

async fn task_2() {
    println!("hello world from 2!");
}

async fn yield_kernel() {
    println!("yield kernel task!");
}

struct FibonacciFuture {
    a: usize,
    b: usize,
    i: usize,
    cnt: usize,
}

impl FibonacciFuture {
    fn new(cnt: usize) -> FibonacciFuture {
        FibonacciFuture {
            a: 0,
            b: 1,
            i: 0,
            cnt,
        }
    }
}

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

impl Future for FibonacciFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.i == self.cnt {
            println!("Fibonacci result: {}", self.a);
            Poll::Ready(())
        } else {
            let t = self.a;
            self.a += self.b;
            self.b = t;
            self.i += 1;
            println!("Fibonacci: i = {}, a = {}, b = {}", self.i, self.a, self.b);
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
