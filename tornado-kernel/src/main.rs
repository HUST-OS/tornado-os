#![no_std]
#![no_main]
#![feature(global_asm, llvm_asm, asm, alloc_error_handler)]
#![feature(drain_filter)]
#![feature(maybe_uninit_uninit_array)]
#![feature(naked_functions)]
#[macro_use]
extern crate alloc;

#[macro_use]
mod console;
mod algorithm;
mod hart;
mod memory;
mod panic;
mod sbi;
mod syscall;
mod task;
mod trap;
mod user;
mod virtio;
mod plic;
mod sdcard;
mod fs;

#[cfg(not(test))]
global_asm!(include_str!("entry.asm"));

#[cfg(feature = "qemu")]
const SHAREDPAYLOAD_BASE: usize = 0x8600_0000;

#[cfg(feature = "k210")]
const SHAREDPAYLOAD_BASE: usize = 0x8020_0000;

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

    println!("hart {} booted", hart_id);

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

    let mut vec = Vec::new();
    for i in 0..10000 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10000);
    for (i, value) in vec.into_iter().enumerate() {
        assert_eq!(value, i);
    }

    println!("heap test passed");
    println!("Max asid = {:?}", memory::max_asid());

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
            "Test #{}: {:?} and {:?}",
            i,
            frame_0.start_address(),
            frame_1.start_address()
        );
    }

    println!("_swap_frame: {:#x}", _swap_frame as usize);
    println!("_user_to_supervisor: {:#x}", _user_to_supervisor as usize);
    println!("_supervisor_to_user: {:#x}", _supervisor_to_user as usize);

    // 在启动程序之前，需要加载内核当前线程的信息到tp寄存器中
    unsafe { hart::KernelHartInfo::load_hart(hart_id) };
    // 这之后就可以分配地址空间了，这之前只能用内核的地址空间

    println!("Current hart: {}", hart::KernelHartInfo::hart_id());

    // todo: 这里要有个地方往tp里写东西，否则目前会出错
    let kernel_memory = memory::MemorySet::new_kernel().expect("create kernel memory set");
    kernel_memory.activate();
    
    #[cfg(feature = "qemu")]
    unsafe { plic::xv6_plic_init(); }

    let shared_payload = unsafe { task::SharedPayload::load(SHAREDPAYLOAD_BASE) };

    let process = task::Process::new(kernel_memory).expect("create process 1");
    let hart_id = crate::hart::KernelHartInfo::hart_id();
    let address_space_id = process.address_space_id();
    let stack_handle = process.alloc_stack().expect("alloc initial stack");
    
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

    println!("task_1: {:?}", task_1);
    println!("task_2: {:?}", task_2);
    println!("task_3: {:?}", task_3);
    #[cfg(feature = "k210")]
    println!("task_4: {:?}", task_4);

    unsafe {
        shared_payload.add_task(hart_id, address_space_id, task_1.task_repr());
        shared_payload.add_task(hart_id, address_space_id, task_2.task_repr());
        shared_payload.add_task(hart_id, address_space_id, task_3.task_repr());
        #[cfg(feature = "k210")]
        shared_payload.add_task(hart_id, address_space_id, task_4.task_repr());
    }

    #[cfg(feature = "qemu")]
    {
        let mut t = VIRTIO_TASK.lock();
        let task4_repr = unsafe { task_4.task_repr() };
        t.push(task4_repr);
        // 释放锁
        drop(t);
        unsafe {
            shared_payload.add_task(hart_id, address_space_id, task4_repr);
        }
    }

    task::run_until_idle(
        || unsafe { shared_payload.peek_task(task::kernel_should_switch) },
        |task_repr| unsafe { shared_payload.delete_task(task_repr) },
        |task_repr, new_state| unsafe { shared_payload.set_task_state(task_repr, new_state) },
    );

    end(stack_handle.end.0 - 4)
}

#[cfg(feature = "qemu")]
fn end(stack_end: usize) -> ! {
    user::first_enter_user(stack_end)
}
#[cfg(feature = "k210")]
fn end(_stack_end: usize) -> ! {
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

use alloc::vec::Vec;
use spin::Mutex;
use lazy_static::lazy_static;
lazy_static!(
    pub static ref VIRTIO_TASK: Mutex<Vec<usize>> = Mutex::new(Vec::new());
);