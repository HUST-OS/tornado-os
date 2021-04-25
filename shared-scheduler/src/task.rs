//! 共享任务调度器
use crate::algorithm::{Scheduler, RingFifoScheduler};
use crate::mm::AddressSpaceId;
use core::ptr::NonNull;
use spin::Mutex;

/// 共享调度器返回的结果
// 不应该移除，这对FFI是安全的，我们只考虑Rust用户，其它语言自己想办法
#[derive(Debug)]
#[repr(C)]
pub enum TaskResult {
    /// 应当立即执行特定任务，里面是表示形式
    // 如果不释放任务，再次执行，还是会得到相同的任务，必须释放任务
    Task(TaskRepr),
    /// 其他地址空间的任务要运行，应当让出时间片
    /// 并返回下一个地址空间的编号
    ShouldYield(usize),
    /// 队列已空，所有任务已经结束
    Finished
}

/// 任务的表示形式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TaskRepr(usize);

/// 共享调度器的类型
pub type SharedScheduler = Mutex<RingFifoScheduler<TaskMeta, 100>>;

/// 全局的共享调度器
/// 放到数据段，内核或用户从这个地址里取得共享调度器
pub static SHARED_SCHEDULER: SharedScheduler = Mutex::new(RingFifoScheduler::new());

/// 共享任务的元数据
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct TaskMeta {
    /// 运行此任务的硬件线程编号
    pub(crate) hart_id: usize,
    /// 地址空间的编号
    pub(crate) address_space_id: AddressSpaceId,
    // 元数据指针，由所在的地址空间解释
    task_repr: TaskRepr,
    // 任务当前的状态
    pub(crate) state: TaskState,
}

// todo: 在调度器中设计，如果任务正在休眠，就跳过

/// 任务当前的状态
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TaskState {
    Ready = 0,
    Sleeping = 1,
}

/// 给共享调度器添加任务
/// 
/// 在内核态和用户态都可以调用
pub unsafe extern "C" fn shared_add_task(
    shared_scheduler: NonNull<()>,
    hart_id: usize,
    address_space_id: AddressSpaceId,
    task_repr: TaskRepr,
) -> bool { // 本次比赛的设计比较简单，true表示成功，false表示失败
    // println!("[Shared add task] {:p} {} {:?} {:x?}", shared_scheduler, hart_id, address_space_id, task_repr);
    let s: NonNull<SharedScheduler> = shared_scheduler.cast();
    let handle = prepare_handle(hart_id, address_space_id, task_repr);
    let mut scheduler = s.as_ref().lock();
    scheduler.add_task(handle).is_none()
}

#[inline] unsafe fn prepare_handle(
    hart_id: usize,
    address_space_id: AddressSpaceId,
    task_repr: TaskRepr,
) -> TaskMeta {
    TaskMeta {
        hart_id,
        address_space_id,
        task_repr,
        state: TaskState::Ready, // 默认为就绪状态
    }
}

/// 从共享调度器中找到下一个任务
/// 
/// 在内核态和用户态都可以调用
pub unsafe extern "C" fn shared_peek_task(
    shared_scheduler: NonNull<()>,
    should_switch: extern "C" fn(AddressSpaceId) -> bool
) -> TaskResult {
    // 得到共享调度器的引用
    // println!("[Shared peek task] {:p} {:x}", shared_scheduler, should_switch as usize);
    let mut s: NonNull<SharedScheduler> = shared_scheduler.cast();
    let scheduler = s.as_mut().lock();
    if let Some(task) = scheduler.peek_next_task() {
        // println!("Pop task {:x?}!", task);
        if should_switch(task.address_space_id) {
            // 如果需要跳转到其他地址空间，则不弹出任务，返回需要跳转到的地址空间编号
            return TaskResult::ShouldYield(task.address_space_id.into_inner())
        }
        // 直接把任务交给调用者
        let task_repr = task.task_repr;
        drop(scheduler); // 释放锁
        return TaskResult::Task(task_repr)
        // 调用者拿到任务后，执行此任务，然后必须销毁任务，否则任务会被重新拿出来再执行一次
        // CCC: 这样的设计好像会增加一些时间复杂度，后面需要再考虑一下这部分的设计
    } else {
        // 没有任务了，返回已完成
        return TaskResult::Finished;
    }
}

/// 删除一个共享调度器中的任务
pub unsafe extern "C" fn shared_delete_task(
    shared_scheduler: NonNull<()>,
    _hart_id: usize,
    _address_space_id: AddressSpaceId,
    task_repr: TaskRepr,
) -> bool {
    let mut s: NonNull<SharedScheduler> = shared_scheduler.cast();
    let mut scheduler = s.as_mut().lock();
    let next_handle = scheduler.next_task();
    if let Some(handle) = next_handle {
        if handle.task_repr == task_repr {
            return true
        } else {
            return false // panic!("delete a previous task is not currently supported")
        }
    }
    false
}

/// 设置任务的状态
pub unsafe extern "C" fn shared_set_task_state(
    shared_scheduler: NonNull<()>,
    hart_id: usize,
    address_space_id: AddressSpaceId,
    task_repr: TaskRepr,
    new_state: TaskState,
) {
    let mut s: NonNull<SharedScheduler> = shared_scheduler.cast();
    let mut scheduler = s.as_mut().lock();
    if let Some(task) = scheduler.find_first_task_mut(|t| task_eq(t, hart_id, address_space_id, task_repr)) {
        task.state = new_state;
    }
}

fn task_eq(
    a: &TaskMeta, 
    hart_id: usize,
    address_space_id: AddressSpaceId,
    task_repr: TaskRepr,
) -> bool {
    a.hart_id == hart_id && a.address_space_id == address_space_id && a.task_repr == task_repr
}
