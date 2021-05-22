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

use crate::memory::AddressSpaceId;
use crate::hart::KernelHartInfo;
use core::ptr::NonNull;
use core::mem;
use super::TaskResult;

/// 任务当前的状态
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TaskState {
    Ready = 0,
    Sleeping = 1,
}

pub extern "C" fn kernel_should_switch(address_space_id: AddressSpaceId) -> bool {
    // 如果当前和下一个任务间地址空间变化了，就说明应当切换上下文
    KernelHartInfo::current_address_space_id() != address_space_id
}

/// 共享载荷
#[repr(C)]
pub struct SharedPayload {
    pub(crate) shared_scheduler: NonNull<()>,
    shared_add_task: unsafe extern "C" fn(NonNull<()>, usize, AddressSpaceId, usize) -> bool,
    shared_peek_task: unsafe extern "C" fn(NonNull<()>, extern "C" fn(AddressSpaceId) -> bool) -> TaskResult,
    shared_peek_wake_task: unsafe extern "C" fn(NonNull<()>, extern "C" fn(AddressSpaceId) -> bool) -> TaskResult,
    shared_delete_task: unsafe extern "C" fn(NonNull<()>, usize) -> bool,
    pub(crate) shared_set_task_state: unsafe extern "C" fn(NonNull<()>, usize, TaskState),
}

unsafe impl Send for SharedPayload {}
unsafe impl Sync for SharedPayload {}

type SharedPayloadAsUsize = [usize; 8]; // 编译时基地址，初始化函数，共享调度器地址，添加函数，弹出函数
type InitFunction = unsafe extern "C" fn() -> PageList;
type SharedPayloadRaw = (
    usize, // 编译时基地址，转换后类型占位，不使用
    usize, // 初始化函数，执行完之后，内核将函数指针置空
    NonNull<()>,
    unsafe extern "C" fn(NonNull<()>, usize, AddressSpaceId, usize) -> bool, // 添加任务
    unsafe extern "C" fn(NonNull<()>, extern "C" fn(AddressSpaceId) -> bool) -> TaskResult, // 弹出任务
    unsafe extern "C" fn(NonNull<()>, extern "C" fn(AddressSpaceId) -> bool) -> TaskResult, // 弹出非睡眠任务
    unsafe extern "C" fn(NonNull<()>, usize) -> bool, // 删除任务
    unsafe extern "C" fn(NonNull<()>, usize, TaskState), // 改变任务的状态 
);

impl SharedPayload {
    pub unsafe fn load(base: usize) -> Self {
        let mut payload_usize = *(base as *const SharedPayloadAsUsize);
        println!("[kernel:shared] Raw table base: {:p}", base as *const SharedPayloadAsUsize);
        println!("[kernel:shared] Content: {:x?}", payload_usize);
        let compiled_offset = payload_usize[0];
        for (i, idx) in payload_usize.iter_mut().enumerate() {
            if i == 0 {
                continue
            }
            *idx = idx.wrapping_sub(compiled_offset).wrapping_add(base);
            if *idx == 0 {
                panic!("shared scheduler used effective address of zero")
            }
        }
        println!("[kernel:shared] After patched: {:x?}", payload_usize);
        let payload_init: InitFunction = mem::transmute(payload_usize[1]);
        let page_list = payload_init(); // 初始化载荷，包括零初始化段的清零等等
        payload_usize[1] = 0; // 置空初始化函数
        println!("[kernel:shared] Init, page list: {:x?}", page_list); // 应当在分页系统中使用上，本次比赛设计暂时不深入
        let raw_table: SharedPayloadRaw = mem::transmute(payload_usize);
        Self {
            shared_scheduler: raw_table.2,
            shared_add_task: raw_table.3,
            shared_peek_task: raw_table.4,
            shared_peek_wake_task: raw_table.5,
            shared_delete_task: raw_table.6,
            shared_set_task_state: raw_table.7,
        }
    }

    /// 往共享调度器中添加任务
    pub unsafe fn add_task(&self, hart_id: usize, address_space_id: AddressSpaceId, task_repr: usize) -> bool {
        let f = self.shared_add_task;
        // println!("Add = {:x}, p1 = {:p}, p2 = {:x}, p3 = {:?}, p4 = {:x}", f as usize, self.shared_scheduler, 
        // hart_id, address_space_id, task_repr);
        f(self.shared_scheduler, hart_id, address_space_id, task_repr)
    }

    /// 从共享调度器中得到下一个任务
    pub unsafe fn peek_task(&self, should_yield: extern "C" fn(AddressSpaceId) -> bool) -> TaskResult {
        let f = self.shared_peek_task;
        // println!("Peek = {:x}, p1 = {:p}, p2 = {:x}", f as usize, self.shared_scheduler, should_yield as usize);
        f(self.shared_scheduler, should_yield)
    }

    /// 从共享调度器中得到下一个非睡眠任务
    pub unsafe fn peek_wake_task(&self, should_yield: extern "C" fn(AddressSpaceId) -> bool) -> TaskResult {
        let f = self.shared_peek_wake_task;
        // println!("Peek wake = {:x}, p1 = {:p}, p2 = {:x}", f as usize, self.shared_scheduler, should_yield as usize);
        f(self.shared_scheduler, should_yield)
    }

    /// 从共享调度器中删除任务
    pub unsafe fn delete_task(&self, task_repr: usize) -> bool {
        let f = self.shared_delete_task;
        f(self.shared_scheduler, task_repr)
    }

    /// 设置一个任务的状态
    pub unsafe fn set_task_state(&self, task_repr: usize, new_state: TaskState) {
        let f = self.shared_set_task_state;
        f(self.shared_scheduler, task_repr, new_state)
    }
}

/// 共享载荷各个段的范围，方便内存管理的权限设置
#[derive(Debug)]
#[repr(C)]
struct PageList {
    rodata: [usize; 2], // 只读数据段
    data: [usize; 2], // 数据段
    text: [usize; 2], // 代码段
}
