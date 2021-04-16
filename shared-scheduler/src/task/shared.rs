//! 共享运行时的设计思路
//! 
use crate::algorithm::{Scheduler, RingFifoScheduler};
use crate::mm::AddressSpaceId;
use core::{ptr::NonNull, usize};
use super::TaskResult;
use super::lock;
use spin::Mutex;

#[link_section = ".shared_text"]
#[export_name  = "_shared_raw_table"]
// 给出函数表偏移量，返回函数地址
// 0 - hello_world()
// 1 - shared_scheduler()
// 2 - shared_add_task()
// 3 - shared_pop_task()
pub extern "C" fn raw_table(offset: usize) -> usize {
    // println!("[shared-rt] enter shared raw table with offset: {:#x}", offset);
    match offset {
        0 => hello_world as usize,
        1 => shared_scheduler as usize,
        2 => shared_add_task as usize,
        3 => shared_pop_task as usize,
        _ => unimplemented!()
    }
}

#[link_section = ".shared_text"]
#[no_mangle]
pub fn hello_world() {
    println!("[shared-rt] hello world from shared-rt");
} 

/// 共享调度器的类型
type SharedScheduler = Mutex<RingFifoScheduler<SharedTaskHandle, 100>>;

/// 全局的共享调度器
/// 放到 .shared_data 段，内核或用户从这个地址里取得共享调度器
#[link_section = ".shared_data"]
pub static SHARED_SCHEDULER: SharedScheduler = Mutex::new(RingFifoScheduler::new());

/// 得到当前正在运行的任务，以备保存上下文
/// 
/// 只供内核中断使用，不需要和用户层共享代码
pub fn current_task() -> Option<SharedTaskHandle> {
    SHARED_SCHEDULER.lock().current_task()
}

/// 得到共享的调度器指针
/// 
/// 可以在共享的添加任务，弹出下一个任务中使用
#[no_mangle]
#[link_section = ".shared_text"]
pub fn shared_scheduler() -> NonNull<()> {
    NonNull::new(&SHARED_SCHEDULER as *const _ as *mut ())
        .expect("create non null pointer")
}

/// 共享任务的句柄
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct SharedTaskHandle {
    /// 运行此任务的硬件线程编号
    pub(crate) hart_id: usize,
    /// 地址空间的编号
    pub(crate) address_space_id: AddressSpaceId,
    /// 对每个虚拟空间来说，task_ptr是Arc<Task>相应的虚拟地址
    /// 比如内核中是内核虚拟地址，用户中是用户的虚拟地址
    pub(crate) task_ptr: usize,
}

// impl SharedTaskHandle {
//     pub fn new(hart_id: usize, asid: usize, task_ptr: usize) -> Self {
//         Self {
//             hart_id,
//             address_space_id: unsafe { AddressSpaceId::from_raw(asid) },
//             task_ptr
//         }
//     }
// }

/// 给共享调度器添加任务
/// 
/// 在内核态和用户态都可以调用
#[no_mangle]
#[link_section = ".shared_text"]
pub unsafe fn shared_add_task(
    shared_scheduler: NonNull<()>,
    handle: SharedTaskHandle
) -> Option<SharedTaskHandle> {
    let s: NonNull<SharedScheduler> = shared_scheduler.cast();
    let mut scheduler = s.as_ref().lock();
    scheduler.add_task(handle)
}

/// 从共享调度器中弹出一个任务
/// 
/// 在内核态和用户态都可以调用
#[no_mangle]
#[link_section = ".shared_text"]
pub unsafe fn shared_pop_task(
    shared_scheduler: NonNull<()>,
    should_switch: fn(&SharedTaskHandle) -> bool
) -> TaskResult {
    // 得到共享调度器的引用
    let mut s: NonNull<SharedScheduler> = shared_scheduler.cast();
    let mut scheduler = s.as_mut().lock();
    if let Some(task) = scheduler.peek_next_task() {
        if should_switch(task) {
            // 如果需要跳转到其他地址空间，则不弹出任务，给出信号
            return TaskResult::ShouldYield
        }
        // 从共享调度器弹出任务交给调用者
        let next_task = scheduler.next_task().unwrap();
        drop(scheduler); // 释放锁
        return TaskResult::Task(next_task)
    } else {
        // 没有任务了，返回已完成
        return TaskResult::Finished;
    }
}