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

pub extern "C" fn kernel_should_switch(handle: &SharedTaskHandle) -> bool {
    // 如果当前和下一个任务间地址空间变化了，就说明应当切换上下文
    KernelHartInfo::current_address_space_id() != handle.address_space_id
}

/// 共享载荷
#[repr(C)]
pub struct SharedPayload {
    shared_scheduler: NonNull<()>,
    shared_add_task: unsafe extern "C" fn(
        shared_scheduler: NonNull<()>, handle: SharedTaskHandle
    ) -> FfiOption<SharedTaskHandle>,
    shared_pop_task: unsafe extern "C" fn(
        shared_scheduler: NonNull<()>, should_switch: extern "C" fn(&SharedTaskHandle) -> bool
    ) -> TaskResult
}

type SharedPayloadAsUsize = [usize; 5]; // 编译时基地址，初始化函数，共享调度器地址，添加函数，弹出函数
type InitFunction = unsafe extern "C" fn() -> PageList;
type SharedPayloadRaw = (
    usize, // 编译时基地址，转换后类型占位，不使用
    usize, // 初始化函数，执行完之后，内核将函数指针置空
    NonNull<()>,
    unsafe extern "C" fn(NonNull<()>, SharedTaskHandle) -> FfiOption<SharedTaskHandle>,
    unsafe extern "C" fn(NonNull<()>, extern "C" fn(&SharedTaskHandle) -> bool) -> TaskResult,
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
            shared_pop_task: raw_table.4
        }
    }

    /// 往共享调度器中添加任务
    pub unsafe fn add_task(&self, handle: SharedTaskHandle) -> Option<SharedTaskHandle> {
        let f = self.shared_add_task;
        f(self.shared_scheduler, handle).into()
    }

    /// 从共享调度器中弹出任务
    pub unsafe fn pop_task(&self, should_yield: extern "C" fn(&SharedTaskHandle) -> bool) -> TaskResult {
        let f = self.shared_pop_task;
        f(self.shared_scheduler, should_yield)
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

impl<T> From<FfiOption<T>> for Option<T> {
    fn from(src: FfiOption<T>) -> Option<T> {
        if let FfiOption::Some(t) = src {
            Some(t)
        } else {
            None
        }
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
