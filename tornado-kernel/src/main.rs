#![no_std]
#![no_main]
#![feature(global_asm, llvm_asm, alloc_error_handler)]
#![feature(drain_filter)]

use memory::STACK_SIZE;

#[macro_use]
extern crate alloc;

#[macro_use]
mod console;
mod algorithm;
mod panic;
mod sbi;
mod interrupt;
mod memory;
mod task;
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

    let process = process::Process::new_kernel().expect("create process");
    let stack = process.alloc_stack().expect("alloc initial stack");
    let task = process::Task::new_kernel(async {
        println!("hello world!")
    }, process, stack);

    sbi::shutdown()
}
