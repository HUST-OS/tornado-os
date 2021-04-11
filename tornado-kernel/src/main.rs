#![no_std]
#![no_main]
#![feature(global_asm, llvm_asm, asm, alloc_error_handler)]
#![feature(drain_filter)]
#![feature(maybe_uninit_uninit_array, maybe_uninit_ref)]
#![feature(naked_functions)]
#[macro_use]
extern crate alloc;

#[macro_use]
mod console;
mod algorithm;
mod panic;
mod sbi;
mod trap;
mod memory;
mod task;
mod hart;
mod user;
mod syscall;

#[cfg(not(test))]
global_asm!(include_str!("entry.asm"));

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
        fn _user_data();
    }

    unsafe { 
        r0::zero_bss(&mut _sbss, &mut _ebss);
        r0::init_data(&mut _sdata, &mut _edata, &_sidata);
    }

    println!("booted");

    memory::init();
    trap::init();

    unsafe {
        asm!("ebreak");
    };

    // 动态内存分配测试
    use alloc::boxed::Box;
    use alloc::vec::Vec;
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
    // let remap = memory::MemorySet::new_kernel().unwrap();
    // remap.activate();
    // println!("kernel remapped");

    // 物理页分配
    for i in 0..2 {
        let frame_0 = match memory::frame_alloc() {
            Some(frame_tracker) => frame_tracker,
            None => panic!("frame allocation failed")
        };
        let frame_1 = match memory::frame_alloc() {
            Some(frame_tracker) => frame_tracker,
            None => panic!("frame allocation failed")
        };
        println!("Test #{}: {:?} and {:?}", i, frame_0.start_address(), frame_1.start_address());
    }
    
    println!("_swap_frame: {:#x}", _swap_frame as usize);
    println!("_user_to_supervisor: {:#x}", _user_to_supervisor as usize);
    println!("_supervisor_to_user: {:#x}", _supervisor_to_user as usize);
    println!("_user_data: {:#x}", _user_data as usize);

    // 在启动程序之前，需要加载内核当前线程的信息到tp寄存器中
    unsafe { hart::KernelHartInfo::load_hart(hart_id) };
    // 这之后就可以分配地址空间了，这之前只能用内核的地址空间

    println!("Current hart: {}", hart::KernelHartInfo::hart_id());
    
    // todo: 这里要有个地方往tp里写东西，目前会出错
    let kernel_memory = memory::MemorySet::new_kernel().expect("create kernel memory set");
    kernel_memory.activate();
    
    // 调用共享运行时的函数
    let raw_table_ptr = 0x8021_b000 as *const ();
    let raw_table: extern "C" fn(a0: usize) -> usize = unsafe { core::mem::transmute(raw_table_ptr) };
    let shared_scheduler_ptr = raw_table(1);
    let shared_add_task_ptr = raw_table(2);
    let shared_pop_task_ptr = raw_table(3);
    let shared_scheduler: fn()  -> core::ptr::NonNull<()> = unsafe {
        core::mem::transmute(shared_scheduler_ptr)
    };
    let shared_add_task: unsafe fn(
        shared_scheduler: core::ptr::NonNull<()>, handle: task::SharedTaskHandle
    ) -> Option<task::SharedTaskHandle> = unsafe {
        core::mem::transmute(shared_add_task_ptr)
    };
    let shared_pop_task: unsafe fn(
        shared_scheduler: core::ptr::NonNull<()>,
        should_switch: fn(&task::SharedTaskHandle) -> bool
    ) -> task::TaskResult = unsafe {
        core::mem::transmute(shared_pop_task_ptr)
    };
    
    let shared_scheduler = shared_scheduler();
    println!("Shared scheduler: {:?}", shared_scheduler);

    let process = task::Process::new(kernel_memory).expect("create process 1");
    let stack_handle = process.alloc_stack().expect("alloc initial stack");
    let task_1 = task::KernelTask::new(task_1(), process.clone());
    let task_2 = task::KernelTask::new(task_2(), process.clone());
    let task_3 = task::KernelTask::new(FibonacciFuture::new(6), process.clone());
    println!("task_1: {:?}", task_1);
    println!("task_2: {:?}", task_2);
    println!("task_3: {:?}", task_3);
    unsafe {
        shared_add_task(shared_scheduler, task_1.shared_task_handle());
        shared_add_task(shared_scheduler, task_2.shared_task_handle());
        shared_add_task(shared_scheduler, task_3.shared_task_handle());
    }
    
    task::run_until_idle(
        || unsafe { shared_pop_task(shared_scheduler, task::SharedTaskHandle::should_switch) },
        |handle| unsafe { shared_add_task(shared_scheduler, handle) }
    );

    // 进入用户态
    user::try_enter_user(stack_handle.end.0 - 4)

    // 关机之前，卸载当前的核。虽然关机后内存已经清空，不是必要，预留未来热加载热卸载处理核的情况
    // unsafe { hart::KernelHartInfo::unload_hart() };
    // 没有任务了，关机
    // sbi::shutdown()
}

// fn spawn(future: impl Future<Output = ()> + 'static + Send + Sync) {
//     unsafe { 
//         // 创建一个新的任务
//         // 在用户层，这里应该使用系统调用，一次性获得一个资源分配的令牌，代替“进程”结构体，复用这个令牌获得资源
//         let process = hart::KernelHartInfo::current_process().unwrap();
//         // 新建一个任务
//         let new_task = task::KernelTask::new(future, process);
//         // 加入调度器
//         let shared_scheduler = task::shared_scheduler();
//         task::shared_add_task(shared_scheduler, new_task.shared_task_handle());
//     }
// }

async fn task_1() {
    println!("hello world from 1!");
}

async fn task_2() {
    println!("hello world from 2!");
}

async fn user_task_1() {
    println!("From user level!");
    loop {}
    // todo: 退出进程 
}

struct FibonacciFuture {
    a: usize,
    b: usize,
    i: usize,
    cnt: usize,
}

impl FibonacciFuture {
    
    fn new(cnt: usize) -> FibonacciFuture {
        FibonacciFuture { a: 0, b: 1, i: 0, cnt }
    }
}
use core::future::Future;
use core::task::{Context, Poll};
use core::pin::Pin;


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
