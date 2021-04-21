//! 共享任务调度器
use crate::algorithm::{Scheduler, RingFifoScheduler};
use crate::mm::AddressSpaceId;
use core::{ptr::NonNull, usize};
use spin::Mutex;

/// 共享调度器返回的结果
// 不应该移除，这对FFI是安全的，我们只考虑Rust用户，其它语言自己想办法
#[derive(Debug)]
#[repr(C)]
pub enum TaskResult {
    /// 应当立即执行特定任务
    Task(SharedTaskHandle),
    /// 其他地址空间的任务要运行，应当让出时间片
    /// 并返回下一个地址空间的编号
    ShouldYield(usize),
    /// 队列已空，所有任务已经结束
    Finished
}

/// 共享调度器的类型
pub type SharedScheduler = Mutex<RingFifoScheduler<SharedTaskHandle, 100>>;

/// 全局的共享调度器
/// 放到数据段，内核或用户从这个地址里取得共享调度器
pub static SHARED_SCHEDULER: SharedScheduler = Mutex::new(RingFifoScheduler::new());

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

/// 给共享调度器添加任务
/// 
/// 在内核态和用户态都可以调用
pub unsafe extern "C" fn shared_add_task(
    shared_scheduler: NonNull<()>,
    handle: SharedTaskHandle
) -> FfiOption<SharedTaskHandle> { // 如果未来有FFI-safe core::option::Option，换掉这个返回值
    let s: NonNull<SharedScheduler> = shared_scheduler.cast();
    let mut scheduler = s.as_ref().lock();
    scheduler.add_task(handle).into()
}

/// 从共享调度器中弹出一个任务
/// 
/// 在内核态和用户态都可以调用
pub unsafe extern "C" fn shared_pop_task(
    shared_scheduler: NonNull<()>,
    should_switch: extern "C" fn(&SharedTaskHandle) -> bool
) -> TaskResult {
    // 得到共享调度器的引用
    let mut s: NonNull<SharedScheduler> = shared_scheduler.cast();
    let mut scheduler = s.as_mut().lock();
    if let Some(task) = scheduler.peek_next_task() {
        if should_switch(task) {
            // 如果需要跳转到其他地址空间，则不弹出任务，返回需要跳转到的地址空间编号
            return TaskResult::ShouldYield(task.address_space_id.into_inner())
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

// 跨FFI边界安全的Option枚举结构
#[repr(C)]
pub enum FfiOption<T> {
    None,
    Some(T),
}

impl<T> From<Option<T>> for FfiOption<T> {
    fn from(src: Option<T>) -> FfiOption<T> {
        if let Some(t) = src {
            FfiOption::Some(t)
        } else {
            FfiOption::None
        }
    }
}
