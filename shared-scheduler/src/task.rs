//! 共享调度器的设计思路
//!
//! 在这个内核中，调度器和执行器是分离的。调度器只根据元数据调度，得到下一个任务是什么。
//! 至于这个任务该如何运行，调度器不知道，需要交给执行器来解释元数据的意义，拿到异步结构之后运行。
//!
//! 这里的异步结构由每个地址空间规定，通常来说，它包含着一个[`Future`]，用于运行任务。
//!
//! 地址空间对应一套虚实地址的映射关系，它相当于传统意义上“进程”的一部分。
//! 对地址空间来说，它规定了虚拟地址的意义，因此如果一个任务的元数据包含它的指针，只有这个地址空间能解释它。
//! 比如共享的任务元数据包含一个指针，但只有创建这个元数据的地址空间，能理解这个元数据的意义。
//!
//! 在每个地址空间，都存在一个执行器。调度器运行调度算法，如果调度算法得到的任务属于当前地址空间，
//! 那么当前空间的执行器能理解元数据的意义，就能从指针中获取到异步结构，便可运行对应的任务。
//!
//! 如果执行器从共享调度器中拿到的任务不属于当前地址空间，则不能理解这个任务的元数据的意义，这时候通常需要进行地址空间的切换。
//!
//! 每个地址空间的执行器能各自解释任务，那么异步结构的设计就因人而异，不一定局限于内核的设计。
//! 用户运行时应当给出自己的设计，然后提供统一的指针，写到元数据中，供调度器调度。
//! 调度器不能理解统一指针的意义，但是能理解任务所在的地址空间编号、硬件线程和优先级。通过这三个参数，
//! 共享调度器就能完成调度任务了。
//!
//! **正因为地址空间是解释和运行异步任务的前提，我们认为它是异步内核不可或缺的概念。**
//!
//! 许多的指令集架构存在也是名为“地址空间”的优化方法，来提高页表缓存的访问效率，我们可以用它们实现软件上的地址空间。
//! 如果具体的处理核上没有实现这种硬件优化，我们只用软件给出“地址空间”的概念，而不在硬件上利用它们。
use crate::algorithm::{RingFifoScheduler, Scheduler};
use crate::mm::AddressSpaceId;
use core::ptr::NonNull;
use spin::Mutex;

/// 共享调度器返回的结果
///
/// note: 不应该移除，这对FFI是安全的，我们目前只考虑Rust语言的支持
#[derive(Debug)]
#[repr(C)]
pub enum TaskResult {
    /// 应当立即执行特定任务，里面是表示形式
    ///
    /// note: 执行器从调度器获得这个值的时候需要调用`shared_delete_task`方法释放任务,
    /// 如果不释放任务，再次执行，还是会得到相同的任务
    Task(TaskRepr),
    /// 其他地址空间的任务要运行，应当提示执行器主动让出，并返回下一个地址空间的编号
    ///
    /// 这时候在用户态应该执行`yield`系统调用
    /// `yield`系统调用将保存当前用户上下文，陷入内核并切换到下一个地址空间去运行
    ShouldYield(usize),
    /// 调度器里面没有醒着的任务，但存在睡眠任务
    ///
    /// note: 当调度器返回该值的次数达到阈值的时候，应当执行`kernel_check`系统调用，
    /// 该系统调用会检查内核，适当将一些任务唤醒。
    NoWakeTask,
    /// 队列已空，所有任务已经结束
    Finished,
}

/// 任务的表示形式，通常为任务结构体的指针
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TaskRepr(usize);

/// 共享调度器的类型
///
/// 第二个泛型参数是常量泛型，设置调度器容量大小
pub type SharedScheduler = Mutex<RingFifoScheduler<TaskMeta, 400>>;

/// 全局的共享调度器
///
/// 放到数据段，内核或用户从这个地址里取得共享调度器
pub static SHARED_SCHEDULER: SharedScheduler = Mutex::new(RingFifoScheduler::new());

/// 共享任务的元数据
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct TaskMeta {
    /// 运行此任务的硬件线程编号
    pub(crate) hart_id: usize,
    /// 地址空间的编号
    ///
    /// 内核地址空间编号是0，用户的地址空间编号从1开始增长
    pub(crate) address_space_id: AddressSpaceId,
    /// 元数据指针，由所在的地址空间解释
    task_repr: TaskRepr,
    /// 任务当前的状态
    pub(crate) state: TaskState,
}

/// 任务当前的状态
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TaskState {
    /// 就绪状态，可以被执行器执行
    Ready = 0,
    /// 睡眠状态
    ///
    /// 一个任务通常被执行器进行`poll`操作后返回`Pending`而被设置为睡眠状态，
    /// 需要被唤醒后才能再次被执行器执行
    Sleeping = 1,
}

/// 给共享调度器添加任务
///
/// * shared_scheduler: 共享调度器的[`NonNull`]指针
/// * hard_id: 硬件线程编号
/// * asid: 任务的地址空间编号
/// * task_repr: 任务的指针
///
/// 添加任务成功返回 true,否则返回 false
pub unsafe extern "C" fn shared_add_task(
    shared_scheduler: NonNull<()>,
    hart_id: usize,
    asid: AddressSpaceId,
    task_repr: TaskRepr,
) -> bool {
    let s: NonNull<SharedScheduler> = shared_scheduler.cast();
    let handle = prepare_handle(hart_id, asid, task_repr);
    let mut scheduler = s.as_ref().lock();
    scheduler.add_task(handle).is_none()
}

#[inline]
/// 用于将一些数据打包成[`TaskMeta`]
unsafe fn prepare_handle(
    hart_id: usize,
    asid: AddressSpaceId,
    task_repr: TaskRepr,
) -> TaskMeta {
    TaskMeta {
        hart_id,
        address_space_id: asid,
        task_repr,
        state: TaskState::Ready, // 默认为就绪状态
    }
}

/// 从共享调度器中找到下一个任务
/// 
/// 如果拿出的任务处于睡眠状态则重新放入调度队列尾部
///
/// * shared_scheduler: 共享调度器的[`NonNull`]指针
/// * should_switch: 判断是否需要进行地址空间切换的函数，由使用者给出
///
/// 返回一个[`TaskResult`]，执行器需要根据返回值的类型采取相应的行为
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
                        // 返回[`TaskResult::NoWakeTask`], 提示执行器调度器里面还有睡眠任务
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
///
/// * shared_scheduler: 共享调度器的[`NonNull`]指针
/// * task_repr: 任务的指针
///
/// 删除成功返回true，找不到对应的任务返回false
pub unsafe extern "C" fn shared_delete_task(
    shared_scheduler: NonNull<()>,
    task_repr: TaskRepr,
) -> bool {
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
///
/// * shared_scheduler: 共享调度器的[`NonNull`]指针
/// * task_repr: 任务的指针
/// * new_state: 任务的新状态
///
/// 通常用来唤醒任务
pub unsafe extern "C" fn shared_set_task_state(
    shared_scheduler: NonNull<()>,
    task_repr: TaskRepr,
    new_state: TaskState,
) {
    let mut s: NonNull<SharedScheduler> = shared_scheduler.cast();
    let mut scheduler = s.as_mut().lock();
    let len = scheduler.queue_len().unwrap();
    let mut count = 0;
    loop {
        if count >= len {
            break;
        }
        let next_handle = scheduler.peek_next_task();
        match next_handle {
            Some(task) => {
                if task.task_repr == task_repr {
                    // 找到了需要设置状态的任务
                    let change_task = scheduler.peek_next_task_mut().unwrap();
                    change_task.state = new_state;
                    // 之前已经把count个任务从头部拿出来放到尾部了，现在要恢复它们
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
            None => break,
        }
    }
}
