//! 共享任务调度器
use crate::algorithm::{RingFifoScheduler, Scheduler};
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
    /// 调度器里面没有醒着的任务，但存在睡眠任务
    NoWakeTask,
    /// 队列已空，所有任务已经结束
    Finished,
}

/// 任务的表示形式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TaskRepr(usize);

/// 共享调度器的类型
pub type SharedScheduler = Mutex<RingFifoScheduler<TaskMeta, 400>>;

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
) -> bool {
    // 本次比赛的设计比较简单，true表示成功，false表示失败
    // println!("[Shared] {:p} {} {:?} {:x?}", shared_scheduler, hart_id, address_space_id, task_repr);
    let s: NonNull<SharedScheduler> = shared_scheduler.cast();
    let handle = prepare_handle(hart_id, address_space_id, task_repr);
    let mut scheduler = s.as_ref().lock();
    scheduler.add_task(handle).is_none()
}

#[inline]
unsafe fn prepare_handle(
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
/// 如果任务处于睡眠状态则重新放入调度队列尾部
///
/// 内核态和用户态都可以调用
pub unsafe extern "C" fn shared_peek_task(
    shared_scheduler: NonNull<()>,
    should_switch: extern "C" fn(AddressSpaceId) -> bool,
) -> TaskResult {
    print!(""); // 很奇怪的 bug，需要在这里输出点东西运行才会正常
    // 得到共享调度器的引用
    let mut s: NonNull<SharedScheduler> = shared_scheduler.cast();
    let mut scheduler = s.as_mut().lock();
    let mut ret_task;
    let mut count = 0; // 计数器，防止无限循环
    loop {
        ret_task = scheduler.peek_next_task();
        match ret_task {
            Some(task) => {
                if task.state == TaskState::Sleeping {
                    if count >= scheduler.queue_len().unwrap() {
                        // 已经全部遍历过一遍，没有找到醒着的任务
                        // 返回 TaskResult::NoWakeTask, 提示执行器调度器里面还有睡眠任务
                        // 如果等待时间过长，则下一次时间中断的时候切换地址空间
                        return TaskResult::NoWakeTask;
                    }
                    // 睡眠状态，将当前任务放到调度队列尾部
                    let sleep_task = scheduler.next_task().unwrap();
                    let add_ret = scheduler.add_task(sleep_task);
                    assert!(add_ret.is_none());
                    count = count.wrapping_add(1);
                    // 进行下一个循环
                } else {
                    if should_switch(task.address_space_id) {
                        // 如果需要跳转到其他地址空间，则不弹出任务，返回需要跳转到的地址空间编号
                        return TaskResult::ShouldYield(task.address_space_id.into_inner());
                    } else {
                        // 直接把任务交给调用者
                        let task_repr = task.task_repr;
                        drop(scheduler); // 释放锁
                        return TaskResult::Task(task_repr);
                    }
                }
            }
            // 没有任务了，返回已完成
            None => return TaskResult::Finished,
        }
    }
}

/// 删除一个共享调度器中的任务
pub unsafe extern "C" fn shared_delete_task(
    shared_scheduler: NonNull<()>,
    task_repr: TaskRepr,
) -> bool {
    // println!("[shared] delete task");
    let mut s: NonNull<SharedScheduler> = shared_scheduler.cast();
    let mut scheduler = s.as_mut().lock();
    let len = scheduler.queue_len().unwrap();
    let mut count = 0;
    loop {
        if count >= len {
            return false;
        }
        let next_handle = scheduler.peek_next_task();
        match next_handle {
            Some(task) => {
                if task.task_repr == task_repr {
                    // 找到了需要删除的任务
                    let _drop_task = scheduler.next_task().unwrap();
                    // 之前已经把 count 个任务从头部拿出来放到尾部了，现在要恢复它们
                    let current_len = scheduler.queue_len().unwrap();
                    for _ in 0..(current_len - count) {
                        let next_task = scheduler.next_task().unwrap();
                        scheduler.add_task(next_task);
                    }
                    return true;
                } else {
                    // 把任务从头部拿出来再放队列尾部
                    let next_task = scheduler.next_task().unwrap();
                    scheduler.add_task(next_task);
                    count += 1;
                    // 进入下一次循环
                }
            }
            None => return false,
        }
    }
}

/// 设置任务的状态
pub unsafe extern "C" fn shared_set_task_state(
    shared_scheduler: NonNull<()>,
    task_repr: TaskRepr,
    new_state: TaskState,
) {
    // println!("[shared] set task state");
    let mut s: NonNull<SharedScheduler> = shared_scheduler.cast();
    let mut scheduler = s.as_mut().lock();
    let len = scheduler.queue_len().unwrap();
    let mut count = 0;
    loop {
        if count >= len {
            // panic!("task not found!")
            break;
        }
        let next_handle = scheduler.peek_next_task();
        match next_handle {
            Some(task) => {
                if task.task_repr == task_repr {
                    // 找到了需要设置状态的任务
                    let change_task = scheduler.peek_next_task_mut().unwrap();
                    change_task.state = new_state;
                    // 之前已经把 count 个任务从头部拿出来放到尾部了，现在要恢复它们
                    for _ in 0..(len - count) {
                        let next_task = scheduler.next_task().unwrap();
                        scheduler.add_task(next_task);
                    }
                    break;
                } else {
                    // 把任务从头部拿出来再放队列尾部
                    let next_task = scheduler.next_task().unwrap();
                    scheduler.add_task(next_task);
                    count += 1;
                    // 进入下一次循环
                }
            }
            // None => panic!("task not found!"),
            None => break,
        }
    }
}
