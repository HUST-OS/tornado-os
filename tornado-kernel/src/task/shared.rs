#[allow(unused_imports)]
use crate::algorithm::{Scheduler, ScheduledItem, RingFifoScheduler, SameAddrSpaceScheduler};
use crate::memory::AddressSpaceId;
use crate::hart::KernelHartInfo;
use core::ptr::NonNull;
use super::TaskResult;

/// 共享调度器的类型
// type SharedScheduler = spin::Mutex<RingFifoScheduler<SharedTaskHandle, 500>>;
type SharedScheduler = spin::Mutex<SameAddrSpaceScheduler<SharedTaskHandle, 500>>;

/// 所有任务的调度器
///
/// 注意：所有.shared_data段内的数据不应该分配堆空间
#[link_section = ".shared_data"]
pub static SHARED_SCHEDULER: SharedScheduler = spin::Mutex::new(SameAddrSpaceScheduler::new());

/// 得到共享的调度器指针
///
/// 可以在共享的添加任务、弹出下一个任务中使用
pub fn shared_scheduler() -> NonNull<()> {
    NonNull::new(&SHARED_SCHEDULER as *const _ as *mut ()).expect("create non null pointer")
}

/// 共享的包含Future在用户空间的地址
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl SharedTaskHandle {
    fn should_switch(&self) -> bool {
        // 如果当前和下一个任务间地址空间变化了，就说明应当切换上下文
        KernelHartInfo::current_address_space_id() != self.address_space_id
    }
}

impl ScheduledItem for SharedTaskHandle {
    fn need_switch(&self) -> bool {
        self.should_switch()
    }
}

#[allow(unused)] // todo: 用上 -- luojia65
pub static SHARED_RAW_TABLE: (unsafe fn(NonNull<()>, SharedTaskHandle) -> Option<SharedTaskHandle>, unsafe fn(NonNull<()>) -> TaskResult)
    = (shared_add_task, shared_pop_task);

/// 共享的添加新任务
///
/// 在内核态和用户态都可以调用，访问的是shared_scheduler对应的同一块内存
#[link_section = ".shared_text"]
pub unsafe fn shared_add_task(shared_scheduler: NonNull<()>, handle: SharedTaskHandle) -> Option<SharedTaskHandle> {
    let s: NonNull<SharedScheduler> = shared_scheduler.cast();
    // println!("Add task: scheduler = {:?}, handle = {:x?}", s, handle);
    let mut scheduler = s.as_ref().lock();
    scheduler.add_task(handle)
}

/// 共享的弹出下一个任务
///
/// 在内核态和用户态都可以调用，访问的是shared_scheduler对应的同一块内存
#[link_section = ".shared_text"]
pub unsafe fn shared_pop_task(shared_scheduler: NonNull<()>) -> TaskResult {
    // 得到共享调度器的引用
    let mut s: NonNull<SharedScheduler> = shared_scheduler.cast();
    let mut scheduler = s.as_mut().lock();
    if let Some(task) = scheduler.peek_next_task() {
        // 还有任务，尝试运行这个任务
        if task.should_switch() { // 如果需要跳转到其它的地址空间，就不弹出任务，提示需要切换地址空间
            return TaskResult::ShouldYield
        }
        // 是本地址空间的任务，从调度器拿出这个任务
        // note(unwrap): 前面peek已经返回Some了
        let next_task = scheduler.next_task().unwrap();
        drop(scheduler); // 释放锁
        // 返回这个任务，以便当前地址空间的执行器运行
        TaskResult::Task(next_task)
    } else {
        // 没有任务了，返回已完成
        TaskResult::Finished
    }
}
