use crate::{hart::KernelHartInfo, memory::AddressSpaceId, task::TaskResult};
use core::{mem, ptr::NonNull};

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

/// 共享调度器
#[repr(C)]
pub struct SharedPayload {
    pub(crate) shared_scheduler: NonNull<()>,
    shared_add_task: unsafe extern "C" fn(NonNull<()>, usize, AddressSpaceId, usize) -> bool,
    shared_peek_task:
        unsafe extern "C" fn(NonNull<()>, extern "C" fn(AddressSpaceId) -> bool) -> TaskResult,
    shared_delete_task: unsafe extern "C" fn(NonNull<()>, usize) -> bool,
    pub(crate) shared_set_task_state: unsafe extern "C" fn(NonNull<()>, usize, TaskState),
}

unsafe impl Send for SharedPayload {}
unsafe impl Sync for SharedPayload {}

type SharedPayloadAsUsize = [usize; 7]; // 编译时基地址，初始化函数，共享调度器地址，添加函数，弹出函数
type InitFunction = unsafe extern "C" fn() -> PageList;
type SharedPayloadRaw = (
    usize, // 编译时基地址，转换后类型占位，不使用
    usize, // 初始化函数，执行完之后，内核将函数指针置空
    NonNull<()>,
    unsafe extern "C" fn(NonNull<()>, usize, AddressSpaceId, usize) -> bool, // 添加任务
    unsafe extern "C" fn(NonNull<()>, extern "C" fn(AddressSpaceId) -> bool) -> TaskResult, // 弹出任务
    unsafe extern "C" fn(NonNull<()>, usize) -> bool, // 删除任务
    unsafe extern "C" fn(NonNull<()>, usize, TaskState), // 改变任务的状态
);

impl SharedPayload {
    /// 根据基地址加载共享调度器
    ///
    /// # Example:
    ///
    /// ```
    /// # const BASE: usize = 0x8600_000;
    /// let shared_load = unsafe { SharedPayload::load(BASE); }
    /// ```
    pub unsafe fn load(base: usize) -> Self {
        let mut payload_usize = *(base as *const SharedPayloadAsUsize);
        // println!(
        //     "[kernel:shared] Raw table base: {:p}",
        //     base as *const SharedPayloadAsUsize
        // );
        // println!("[kernel:shared] Content: {:x?}", payload_usize);
        let compiled_offset = payload_usize[0];
        for (i, idx) in payload_usize.iter_mut().enumerate() {
            if i == 0 {
                continue;
            }
            *idx = idx.wrapping_sub(compiled_offset).wrapping_add(base);
            if *idx == 0 {
                panic!("shared scheduler used effective address of zero")
            }
        }
        // println!("[kernel:shared] After patched: {:x?}", payload_usize);
        let payload_init: InitFunction = mem::transmute(payload_usize[1]);
        let page_list = payload_init(); // 初始化载荷，包括零初始化段的清零等等
        payload_usize[1] = 0; // 置空初始化函数
                              // println!("[kernel:shared] Init, page list: {:x?}", page_list); // 应当在分页系统中使用上，本次比赛设计暂时不深入
        let raw_table: SharedPayloadRaw = mem::transmute(payload_usize);
        Self {
            shared_scheduler: raw_table.2,
            shared_add_task: raw_table.3,
            shared_peek_task: raw_table.4,
            shared_delete_task: raw_table.5,
            shared_set_task_state: raw_table.6,
        }
    }

    /// 往共享调度器中添加任务
    ///
    /// # Example:
    ///
    /// ```
    /// # const BASE: usize = 0x8600_000;
    /// unsafe {
    ///     let shared_load = SharedPayload::new(BASE);
    ///     let asid = AddressSpaceId::from_raw(0);
    ///     shared_load.add_task(0, asid, task.task_repr());
    /// }
    /// ```
    pub unsafe fn add_task(
        &self,
        hart_id: usize,
        address_space_id: AddressSpaceId,
        task_repr: usize,
    ) -> bool {
        let f = self.shared_add_task;
        // hart_id, address_space_id, task_repr);
        f(self.shared_scheduler, hart_id, address_space_id, task_repr)
    }

    /// 从共享调度器中得到下一个任务
    ///
    /// # Example:
    ///
    /// ```
    /// todo!()
    /// ```
    pub unsafe fn peek_task(
        &self,
        should_yield: extern "C" fn(AddressSpaceId) -> bool,
    ) -> TaskResult {
        let f = self.shared_peek_task;
        f(self.shared_scheduler, should_yield)
    }

    /// 从共享调度器中删除任务
    ///
    /// ```
    /// unsafe{
    ///     assert!(shared_load.delete_task(task.task_repr()));        
    /// }
    /// ```
    pub unsafe fn delete_task(&self, task_repr: usize) -> bool {
        let f = self.shared_delete_task;
        f(self.shared_scheduler, task_repr)
    }

    /// 设置一个任务的状态
    ///
    /// # Example:
    ///
    /// ```
    /// todo!()
    /// ```
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
    data: [usize; 2],   // 数据段
    text: [usize; 2],   // 代码段
}
