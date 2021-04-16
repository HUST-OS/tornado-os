//！ 尝试在用户态给共享调度器添加任务
use super::task::{TaskResult, UserTask};
use woke::waker_ref;
use alloc::sync::Arc;
use core::{mem, task::{Poll, Context}};

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
    pub(crate) fn into_inner(self) -> usize {
        self.0 as usize
    }
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
        // todo
        false
    }
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
            if task.is_sleeping() {
                mem::forget(task); // 不要释放内存
                push_task(handle);
                continue
            }
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
            TaskResult::ShouldYield => {
                //todo
                // crate::trap::switch_to_user()
            },
            TaskResult::Finished => return None
        }
    }
}