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
pub struct AddressSpaceId(u16);

impl AddressSpaceId {
    pub(crate) unsafe fn from_raw(asid: usize) -> AddressSpaceId {
        AddressSpaceId(asid as u16)
    }
}

pub extern "C" fn user_should_switch(_handle: &SharedTaskHandle) -> bool {
    false
}

pub fn run_until_ready<F, G>(pop_task: F, push_task: G) -> Option<usize>
where
    F: Fn() -> TaskResult,
    G: Fn(SharedTaskHandle) -> Option<SharedTaskHandle> 
{
    loop {
        let task = pop_task();
        if let TaskResult::Task(handle) = task {
            let task: Arc<UserTask> = unsafe { Arc::from_raw(handle.task_ptr as *mut _) };
            // if task.is_sleeping() {
            //     mem::forget(task); // 不要释放内存
            //     push_task(handle);
            //     continue
            // }
            mem::forget(task); // 不要释放内存
        }
        match task {
            TaskResult::Task(handle) => {
                // 在相同的（内核）地址空间里面
                let task: Arc<UserTask> = unsafe { Arc::from_raw(handle.task_ptr as *mut _) };
                task.mark_sleep();
                // make a waker for our task
                let waker = waker_ref(&task);
                // poll our future and give it a waker
                let mut context = Context::from_waker(&*waker);

                let ret = task.future.lock().as_mut().poll(&mut context);
                if let Poll::Ready(x) = ret {
                    return Some(x);
                }
                else {
                    mem::forget(task); // 不要释放task的内存，它将继续保存在内存中被使用
                    push_task(handle); 
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
    shared_add_task: unsafe extern "C" fn(
        shared_scheduler: NonNull<()>, handle: SharedTaskHandle
    ) -> FfiOption<SharedTaskHandle>,
    shared_pop_task: unsafe extern "C" fn(
        shared_scheduler: NonNull<()>, should_switch: extern "C" fn(&SharedTaskHandle) -> bool
    ) -> TaskResult
}

type SharedPayloadAsUsize = [usize; 4]; // 编译时基地址，共享调度器地址，添加函数，弹出函数
type SharedPayloadRaw = (
    usize, // 编译时基地址，转换后类型占位，不使用
    NonNull<()>,
    unsafe extern "C" fn(NonNull<()>, SharedTaskHandle) -> FfiOption<SharedTaskHandle>,
    unsafe extern "C" fn(NonNull<()>, extern "C" fn(&SharedTaskHandle) -> bool) -> TaskResult,
);

impl SharedPayload {
    pub unsafe fn new(base: usize) -> Self {
        let mut payload_usize = *(base as *const SharedPayloadAsUsize);
        let compiled_offset = payload_usize[0];
        for (i, idx) in payload_usize.iter_mut().enumerate() {
            if i == 0 {
                continue
            }
            *idx = idx.wrapping_sub(compiled_offset).wrapping_add(base);
        }
        let raw_table: SharedPayloadRaw = mem::transmute(payload_usize);
        Self {
            shared_scheduler: raw_table.1,
            shared_add_task: raw_table.2,
            shared_pop_task: raw_table.3
        }
    }

    pub unsafe fn add_task(&self, handle: SharedTaskHandle) -> Option<SharedTaskHandle> {
        let f = self.shared_add_task;
        f(self.shared_scheduler, handle).into()
    }

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
