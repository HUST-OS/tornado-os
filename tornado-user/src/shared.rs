use crate::do_yield;

//！ 尝试在用户态给共享调度器添加任务
use super::task::{TaskResult, UserTask};
use woke::waker_ref;
use alloc::sync::Arc;
use core::{mem, task::{Poll, Context}};
use core::ptr::NonNull;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct SharedTaskHandle {
    /// 处理核编号
    pub(crate) hart_id: usize,
    /// 地址空间编号
    pub(crate) address_space_id: AddressSpaceId,
    /// task_ptr 是 Arc<Task> 的虚拟地址
    pub(crate) task_ptr: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct AddressSpaceId(u16);

impl AddressSpaceId {
    // todo: 可见性
    pub unsafe fn from_raw(asid: usize) -> AddressSpaceId {
        AddressSpaceId(asid as u16)
    }
}

pub extern "C" fn user_should_switch(_handle: &SharedTaskHandle) -> bool {
    false
}

pub fn run_until_ready(
    peek_task: impl Fn() -> TaskResult,
    delete_task: impl Fn(usize) -> bool,
) -> Option<usize> {
    loop {
        let task = peek_task();
        match task {
            TaskResult::Task(task_repr) => {
                // 在相同的（内核）地址空间里面
                let task: Arc<UserTask> = unsafe { Arc::from_raw(task_repr as *mut _) };
                task.mark_sleep();
                // make a waker for our task
                let waker = waker_ref(&task);
                // poll our future and give it a waker
                let mut context = Context::from_waker(&*waker);

                let ret = task.future.lock().as_mut().poll(&mut context);
                if let Poll::Ready(x) = ret {
                    delete_task(task_repr);
                    return Some(x);
                } else {
                    mem::forget(task); // 不要释放task的内存，它将继续保存在内存中被使用
                }
            },
            TaskResult::ShouldYield(next_asid) => {
                // 让出操作
                do_yield(next_asid);
            },
            TaskResult::Finished => return None
        }
    }
}

/// 共享载荷
#[repr(C)]
pub struct SharedPayload {
    shared_scheduler: NonNull<()>,
    shared_add_task: unsafe extern "C" fn(NonNull<()>, usize, AddressSpaceId, usize) -> bool,
    shared_peek_task: unsafe extern "C" fn(NonNull<()>, extern "C" fn(&SharedTaskHandle) -> bool) -> TaskResult,
    shared_delete_task: unsafe extern "C" fn(NonNull<()>, usize) -> bool,
}

type SharedPayloadAsUsize = [usize; 6]; // 编译时基地址，（已清空）初始化函数，共享调度器地址，添加函数，弹出函数
type SharedPayloadRaw = (
    usize, // 编译时基地址，转换后类型占位，不使用
    usize, // 初始化函数已清空，不适用
    NonNull<()>,
    unsafe extern "C" fn(NonNull<()>, usize, AddressSpaceId, usize) -> bool,
    unsafe extern "C" fn(NonNull<()>, extern "C" fn(&SharedTaskHandle) -> bool) -> TaskResult,
    unsafe extern "C" fn(NonNull<()>, usize) -> bool,
);

impl SharedPayload {
    pub unsafe fn new(base: usize) -> Self {
        let mut payload_usize = *(base as *const SharedPayloadAsUsize);
        let compiled_offset = payload_usize[0];
        for (i, idx) in payload_usize.iter_mut().enumerate() {
            if i == 0 || i == 1 {
                continue
            }
            *idx = idx.wrapping_sub(compiled_offset).wrapping_add(base);
        }
        let raw_table: SharedPayloadRaw = mem::transmute(payload_usize);
        Self {
            shared_scheduler: raw_table.2,
            shared_add_task: raw_table.3,
            shared_peek_task: raw_table.4,
            shared_delete_task: raw_table.5,
        }
    }

    pub unsafe fn add_task(&self, hart_id: usize, address_space_id: AddressSpaceId, task_repr: usize) -> bool {
        let f = self.shared_add_task;
        f(self.shared_scheduler, hart_id, address_space_id, task_repr)
    }

    pub unsafe fn peek_task(&self, should_yield: extern "C" fn(&SharedTaskHandle) -> bool) -> TaskResult {
        let f = self.shared_peek_task;
        f(self.shared_scheduler, should_yield)
    }

    pub unsafe fn delete_task(&self, task_repr: usize) -> bool {
        let f = self.shared_delete_task;
        f(self.shared_scheduler, task_repr)
    }
}
