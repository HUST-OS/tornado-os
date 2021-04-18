/// 共享调度器的设计思路
///
/// 在这个内核中，调度器和执行器是分离的。调度器只根据元数据调度，得到下一个任务是什么。
/// 至于这个任务该如何运行，调度器不知道，需要交给执行器来解释元数据的意义，拿到异步结构之后运行。
/// 这里的异步结构由每个地址空间规定，在内核中，它包含着一个Future，用于运行任务。
/// 
/// 地址空间对应一套虚实地址的映射关系，它相当于传统意义上“进程”的一部分。
/// 对地址空间来说，它规定了虚拟地址的意义，因此如果一个任务的元数据包含它的指针，只有这个地址空间能解释它。
/// 比如共享的任务元数据包含一个指针，但只有创建这个元数据的地址空间，能理解这个元数据的意义。
/// 
/// 所以在每个地址空间，都存在一个执行器。调度器运行调度算法，如果调度算法得到的任务属于当前地址空间，
/// 那么当前空间的执行器能理解元数据的意义，就能从指针中获取到异步结构，便可运行对应的任务。
/// 
/// 如果执行器从共享调度器中拿到的任务不属于当前地址空间，则需要进行地址空间的切换。  
/// 
/// 每个地址空间的执行器能各自解释任务，那么异步结构的设计就因人而异，不一定局限于内核的设计。
/// 用户运行时应当给出自己的设计，然后提供统一的指针，写到元数据中，供调度器调度。
/// 调度器不能理解统一指针的意义，但是能理解任务所在的地址空间编号、硬件线程和优先级。通过这三个参数，
/// 共享调度器就能完成调度任务了。
///
/// **正因为地址空间是解释和运行异步任务的前提，我们认为它是异步内核不可或缺的概念。**
/// 地址空间和进程也可以有进一步的联系，在传统的内核中，他们是一对一的从属关系。
/// 许多的指令集架构存在也是名为“地址空间”的优化方法，来提高页表缓存的访问效率，我们可以用它们实现软件上的地址空间。
/// 如果具体的处理核上没有实现这种硬件优化，我们只用软件给出“地址空间”的概念，而不在硬件上利用它们。

#[allow(unused_imports)]
use crate::algorithm::{Scheduler, RingFifoScheduler, SameAddrSpaceScheduler};
use crate::memory::AddressSpaceId;
use crate::hart::KernelHartInfo;
use core::ptr::NonNull;
use core::mem;
use super::TaskResult;
use super::lock;


/// 共享的包含Future在用户空间的地址
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

impl SharedTaskHandle {
    pub fn _new(hart_id: usize, asid: usize, task_ptr: usize) -> Self {
        Self {
            hart_id,
            address_space_id: unsafe { AddressSpaceId::from_raw(asid) },
            task_ptr
        }
    }
    pub fn should_switch(handle: &SharedTaskHandle) -> bool {
        // 如果当前和下一个任务间地址空间变化了，就说明应当切换上下文
        // KernelHartInfo::current_address_space_id() != handle.address_space_id
        false
    }

}

impl crate::algorithm::WithAddressSpace for SharedTaskHandle {
    fn should_switch(&self) -> bool {
        self.should_switch()
    }
}

pub struct SharedRawTable {
    pub shared_scheduler: unsafe fn() -> NonNull<()>,
    pub shared_add_task: unsafe fn(
        shared_scheduler: NonNull<()>, handle: SharedTaskHandle
    ) -> Option<SharedTaskHandle>,
    pub shared_pop_task: unsafe fn(
        shared_scheduler: NonNull<()>, should_switch: fn(&SharedTaskHandle) -> bool
    ) -> TaskResult
}


impl SharedRawTable {
    pub unsafe fn new(base: usize) -> Self {
        let raw_table_ptr = base
            as *const [extern "C" fn(); 3]
            as *const extern "C" fn();
        let shared_scheduler_ptr = raw_table_ptr as usize as *const extern "C" fn();
        let shared_add_task = (raw_table_ptr as usize + mem::size_of::<extern "C" fn()>())
            as *const extern "C" fn();
        let shared_pop_task = (raw_table_ptr as usize + mem::size_of::<extern "C" fn()>() * 2)
            as *const extern "C" fn();
        let shared_scheduler: fn() -> NonNull<()> = mem::transmute(*shared_scheduler_ptr);
        let shared_add_task: unsafe fn(
            shared_scheduler: NonNull<()>, handle: SharedTaskHandle
        ) -> Option<SharedTaskHandle> = mem::transmute(*shared_add_task);
        let shared_pop_task: unsafe fn(
            shared_scheduler: NonNull<()>,
            should_yield: fn(&SharedTaskHandle) -> bool
        ) -> TaskResult = mem::transmute(*shared_pop_task);
        Self {
            shared_scheduler,
            shared_add_task,
            shared_pop_task
        }
    }

    pub unsafe fn scheduler(&self) -> NonNull<()> {
        let f = self.shared_scheduler;
        f()
    }

    pub unsafe fn add_task(&self, scheduler: NonNull<()>, handle: SharedTaskHandle) -> Option<SharedTaskHandle> {
        let f = self.shared_add_task;
        f(scheduler, handle)
    }

    pub unsafe fn pop_task(&self, scheduler: NonNull<()>, should_yield: fn(&SharedTaskHandle) -> bool) -> TaskResult {
        let f = self.shared_pop_task;
        f(scheduler, should_yield)
    }
}



