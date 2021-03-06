#![no_std]
#![no_main]
#![feature(global_asm, llvm_asm, asm, alloc_error_handler)]
#![feature(drain_filter)]
#![feature(maybe_uninit_uninit_array)]

#[macro_use]
extern crate alloc;

#[macro_use]
mod console;
mod algorithm;
mod panic;
mod sbi;
mod interrupt;
mod memory;
mod process;
mod hart;

#[cfg(not(test))]
global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    println!("booted");

    memory::init();
    interrupt::init();

    unsafe {
        llvm_asm!("ebreak"::::"volatile");
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
    let remap = memory::MemorySet::new_kernel().unwrap();
    remap.activate();
    println!("kernel remapped");

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
    
    // let executor = task::Executor::default();

    // executor.spawn(async {
    //     println!("Hello world!")
    // });

    // executor.run_until_idle();


    // todo: 这里要有个地方往tp里写东西，目前会出错
    let process = process::Process::new_kernel().expect("create process 1");
    // let stack_handle = process.alloc_stack().expect("alloc initial stack");


    let task_1 = process::Task::new_kernel(task_1(), process.clone());
    let task_2 = process::Task::new_kernel(task_2(), process.clone());
    let task_3 = process::Task::new_kernel(async { task_3().await }, process);
    
    println!("task_1: {:?}", task_1);
    println!("task_2: {:?}", task_2);

    let shared_scheduler = process::shared_scheduler();
    println!("Shared scheduler: {:?}", shared_scheduler);
    unsafe { 
        process::shared_add_task(shared_scheduler, task_1.shared_task_handle());
        process::shared_add_task(shared_scheduler, task_2.shared_task_handle());
    }
    process::Executor::run_until_idle(
        || unsafe { process::shared_pop_task(shared_scheduler) },
        |handle| unsafe { process::shared_add_task(shared_scheduler, handle) }
    );
    unsafe {
        process::shared_add_task(shared_scheduler, task_3.shared_task_handle());
    }
    process::Executor::block_on(|| unsafe { process::shared_pop_task(shared_scheduler)});
    sbi::shutdown()
}

async fn task_1() {
    // let new_task = process::Task::new_kernel(task_3(), process);
    // let shared_scheduler = process::shared_scheduler();
    // process::shared_add_task(shared_scheduler, handle);
    println!("hello world from 1!");
}

async fn task_2() {
    println!("hello world from 2!")
}

fn task_3() -> impl core::future::Future<Output = ()> {
    println!("hello world from 3!");
    TestFuture::new_ready()
}

pub(crate) struct TestFuture {
    is_ready: bool
}

impl TestFuture {
    pub fn _new_pending() -> Self {
        TestFuture {
            is_ready: false
        }
    }

    pub fn new_ready() -> Self {
        TestFuture {
            is_ready: true
        }
    }

    pub fn _set_state(&mut self, is_ready: bool) {
        self.is_ready = is_ready;
    }
}

impl core::future::Future for TestFuture {
    type Output = ();
    
    fn poll(self: core::pin::Pin<&mut Self>, _cx: &mut core::task::Context<'_>) -> core::task::Poll<Self::Output> {
        if self.is_ready {
            core::task::Poll::Ready(())
        } else {
            core::task::Poll::Pending
        }
    }
}