#![feature(naked_functions, asm, global_asm)]
#![feature(llvm_asm)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]
#![feature(generator_trait)]
#![feature(destructuring_assignment)]
#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;

#[macro_use]
mod console;
mod algorithm;
mod async_mutex;
mod event;
mod hart;
mod panic;
mod sbi;
mod syscall;
mod task;
mod trap;
mod user;
mod runtime;
mod mm; 

use alloc::vec::Vec;

#[cfg(not(test))]
global_asm!(include_str!("entry.asm"));

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

lazy_static::lazy_static! {
    pub static ref TRAMPOLINE_VA_START: spin::Mutex<Option<mm::VirtAddr>> = spin::Mutex::new(None);
}

#[no_mangle]
pub extern "C" fn rust_main(hart_id: usize) -> ! {
    // 初始化内存
    if hart_id == 0 {
        unsafe {
            r0::zero_bss(&mut _sbss, &mut _ebss);
            r0::init_data(&mut _sdata, &mut _edata, &_sidata);
        }
        mm::heap_init();
        println!("_swap_frame: {:#x}", _swap_frame as usize);
        println!("_user_to_supervisor: {:#x}", _user_to_supervisor as usize);
        println!("_supervisor_to_user: {:#x}", _supervisor_to_user as usize);
    }

    // 测试页帧分配器
    mm::test_frame_alloc();
    // 页帧分配器。对整个物理的地址空间来说，无论有多少个核，页帧分配器只有一个。
    extern "C" { fn free_memory_start(); }
    let from = mm::PhysAddr(free_memory_start as usize).page_number::<mm::Sv39>();
    let to = mm::PhysAddr(0x8800_0000).page_number::<mm::Sv39>(); // 暂时对qemu写死
    let frame_alloc = spin::Mutex::new(mm::StackFrameAllocator::new(from, to));

    // 测试大页求解算法
    mm::test_map_solve();

    // 在启动程序之前，需要加载内核当前线程的信息到tp寄存器中
    // 这里将创建一个地址空间分配器
    unsafe { hart::KernelHartInfo::load_hart(hart_id) };
    // 这之后就可以分配地址空间了，这之前只能用内核的地址空间

    println!("[kernel] current hart: {}", hart::KernelHartInfo::hart_id());

    // 加载共享调度器
    let shared_payload = unsafe { task::SharedPayload::load(0x8600_0000) };

    if hart_id == 0 {
        // 创建内核地址空间
        let (kernel_addr_space, _kernel_as_frames, trampoline_va_start) = 
            create_sv39_kernel_address_space(&frame_alloc);
        *TRAMPOLINE_VA_START.lock() = Some(trampoline_va_start);
        let kernel_asid = hart::KernelHartInfo::alloc_address_space_id()
            .expect("allocate kernel address space id");
        println!("[kernel] activate kernel address space");
        // 激活内核空间
        let kernel_satp = unsafe {
            mm::activate_paged_riscv_sv39(kernel_addr_space.root_page_number(), kernel_asid)
        };
        unsafe { 
            hart::KernelHartInfo::load_address_space_id(kernel_asid);
            hart::KernelHartInfo::add_asid_satp_map(kernel_asid, kernel_satp); // 暂时不知道什么用，后面再说
        }
        // 创建内核启动所需的进程
        insert_kernel_start_tasks(hart_id, kernel_asid, shared_payload.clone());
    }

    unsafe {dummy()};

    // 初始化运行时，每个核都初始化一次
    loop { 
        if let Some(trampoline_va_start) = *TRAMPOLINE_VA_START.lock() {
            runtime::init(trampoline_va_start);
            break
        }
    }
    // let mut rt = runtime::Runtime::new(
    //     0x1000, 
    //     user_stack_addr,
    //     mm::get_satp_sv39(user_asid, user_space.root_page_number()),
    //     trampoline_va_start,
    //     trampoline_data_addr,
    // ); 
    // use core::pin::Pin;
    // use core::ops::{Generator, GeneratorState};
    pub extern "C" fn kernel_should_switch(address_space_id: mm::AddressSpaceId) -> bool {
        // 如果当前和下一个任务间地址空间变化了，就说明应当切换上下文
        hart::KernelHartInfo::current_address_space_id() != address_space_id
    }
    loop {
        use task::{TaskResult, TaskState, KernelTaskRepr};
        use alloc::sync::Arc;
        let task_result = unsafe {
            shared_payload.peek_task(kernel_should_switch)
        };
        match task_result {
            TaskResult::Task(kernel_task_repr) => { // 轮询到的任务在相同的（内核）地址空间里面
                unsafe {
                    shared_payload.set_task_state(kernel_task_repr, TaskState::Sleeping)
                };
                let task: Arc<KernelTaskRepr> = unsafe { Arc::from_raw(kernel_task_repr as *mut _) };
                // 注册 waker
                let waker = woke::waker_ref(&task);
                let mut context = Context::from_waker(&*waker);
                let ret = task.task().future.lock().as_mut().poll(&mut context);
                if let Poll::Pending = ret {
                    core::mem::forget(task); // 不要释放task的内存，它将继续保存在内存中被使用
                } else {
                    // 否则，释放task的内存
                    unsafe { shared_payload.delete_task(kernel_task_repr) };
                } // 隐含一个drop(task)
            }
            TaskResult::Yield(next_asid) => {
                todo!()
            }
            TaskResult::Wait => {
                unsafe { riscv::asm::wfi() };
                continue
            },
            TaskResult::Finished => break,
        }
    }
    // 开始运行任务
    // executor, runtime, ...

    // task::run_until_idle(
    //     || unsafe { shared_payload.peek_task(task::kernel_should_switch) },
    //     |task_repr| unsafe { shared_payload.delete_task(task_repr) },
    //     |task_repr, new_state| unsafe { shared_payload.set_task_state(task_repr, new_state) },
    // );

    // // 进入用户态
    // user::first_enter_user(stack_handle.end.0 - 4)

    // // 关机之前，卸载当前的核。虽然关机后内存已经清空，不是必要，预留未来热加载热卸载处理核的情况
    unsafe { hart::KernelHartInfo::unload_hart() };
    // // 没有任务了，关机
    sbi::shutdown()
}

fn insert_kernel_start_tasks(hart_id: usize, kernel_asid: mm::AddressSpaceId, shared_payload: task::SharedPayload) {
    let process = task::Process::new(
        vec![],
    );
    // let hart_id = crate::hart::KernelHartInfo::hart_id();
    // let address_space_id = process.address_space_id();
    // let stack_handle = process.alloc_stack().expect("alloc initial stack");
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
    // let task_3 = task::new_kernel(
    //     FibonacciFuture::new(8),
    //     process.clone(),
    //     shared_payload.shared_scheduler,
    //     shared_payload.shared_set_task_state,
    // );
    // use alloc::sync::Arc;
    // let mutex = Arc::new(async_mutex::AsyncMutex::new(0));
    // let event = Arc::new(event::Event::new());
    // let task_4 = task::new_kernel(
    //     async_mutex::async_mutex_test0(Arc::clone(&mutex), Arc::clone(&event)),
    //     process.clone(),
    //     shared_payload.shared_scheduler,
    //     shared_payload.shared_set_task_state,
    // );
    // let task_5 = task::new_kernel(
    //     async_mutex::async_mutex_test1(Arc::clone(&mutex), Arc::clone(&event)),
    //     process.clone(),
    //     shared_payload.shared_scheduler,
    //     shared_payload.shared_set_task_state,
    // );
    println!("task_1: {:?}", task_1);
    println!("task_2: {:?}", task_2);
    // println!("task_3: {:?}", task_3);
    // println!("task_4: {:?}", task_4);
    // println!("task_5: {:?}", task_5);

    unsafe {
        shared_payload.add_task(hart_id, kernel_asid, task_1.task_repr());
        shared_payload.add_task(hart_id, kernel_asid, task_2.task_repr());
    //     shared_payload.add_task(hart_id, address_space_id, task_3.task_repr());
    //     shared_payload.add_task(hart_id, address_space_id, task_4.task_repr());
    //     shared_payload.add_task(hart_id, address_space_id, task_5.task_repr());
    }
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

fn create_sv39_kernel_address_space<A: mm::FrameAllocator + Clone>(frame_alloc: A) -> (mm::PagedAddrSpace<mm::Sv39, A>, Vec<mm::FrameBox<A>>, mm::VirtAddr) {
    let mut kernel_addr_space = mm::PagedAddrSpace::try_new_in(mm::Sv39, frame_alloc)
        .expect("allocate page to create kernel paged address space");
    // 暂时暴力映射所有需要的内核空间
    kernel_addr_space.allocate_map(
        mm::VirtAddr(0x8000_0000).page_number::<mm::Sv39>(), 
        mm::PhysAddr(0x8000_0000).page_number::<mm::Sv39>(), 
        (0x8800_0000 - 0x8000_0000) >> 12, 
        mm::Sv39Flags::R | mm::Sv39Flags::W | mm::Sv39Flags::X
    ).expect("allocate remaining space");
    // 跳板代码页
    let (vpn, ppn, n) = get_trampoline_text_paging_config::<mm::Sv39>();
    let trampoline_va_start = vpn.addr_begin::<mm::Sv39>();
    kernel_addr_space.allocate_map(
        vpn, ppn, n,
        mm::Sv39Flags::R | mm::Sv39Flags::W | mm::Sv39Flags::X
    ).expect("allocate trampoline code mapped space");
    // 跳板数据页
    let /*mut */frames = Vec::new();
    // let data_len = core::mem::size_of::<executor::ResumeContext>();
    // let frame_size = 1_usize << <mm::Sv39 as mm::PageMode>::FRAME_SIZE_BITS;
    // assert!(data_len > 0, "resume context should take place in memory");
    // let data_frame_count = (data_len - 1) / frame_size + 1; // roundup(data_len / frame_size)
    // for i in 0..data_frame_count {
    //     let frame_box = mm::FrameBox::try_new_in(&frame_alloc).expect("allocate user stack frame");
    //     kernel_addr_space.allocate_map(
    //         // 去掉代码页的数量n
    //         mm::VirtAddr(usize::MAX - n * 0x1000 - data_frame_count * 0x1000 + i * 0x1000 + 1).page_number::<mm::Sv39>(), 
    //         frame_box.phys_page_num(), 
    //         1,
    //         mm::Sv39Flags::R | mm::Sv39Flags::W
    //     ).expect("allocate trampoline data mapped space");
    //     frames.push((i, frame_box))
    // }
    // let trampoline_data_addr = mm::VirtAddr(usize::MAX - n * 0x1000 - data_frame_count * 0x1000 + 1);
    (kernel_addr_space, frames, trampoline_va_start)
}

fn get_trampoline_text_paging_config<M: mm::PageMode>() -> (mm::VirtPageNum, mm::PhysPageNum, usize) {
    let (trampoline_pa_start, trampoline_pa_end) = {
        extern "C" { fn strampoline(); fn etrampoline(); }
        (strampoline as usize, etrampoline as usize)
    };
    assert_ne!(trampoline_pa_start, trampoline_pa_end, "trampoline code not declared");
    let trampoline_len = trampoline_pa_end - trampoline_pa_start;
    let trampoline_va_start = usize::MAX - trampoline_len + 1;
    let vpn = mm::VirtAddr(trampoline_va_start).page_number::<M>();
    let ppn = mm::PhysAddr(trampoline_pa_start).page_number::<M>();
    let n = trampoline_len >> M::FRAME_SIZE_BITS;
    (vpn, ppn, n)
}

#[naked]
#[link_section = ".trampoline"]
pub unsafe extern "C" fn dummy() {
    asm!("ret", options(noreturn))
} // 暂时保证跳板页中有数据，这段数据没用，为了保证上面代码检查通过
