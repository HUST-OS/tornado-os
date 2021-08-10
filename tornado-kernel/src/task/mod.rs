mod executor;
mod kernel_task;
mod process;
mod shared;

pub use executor::{ext_intr_off, ext_intr_on, run_one, run_until_idle};
pub use kernel_task::{KernelTask, TaskId};
pub use process::{Process, ProcessId};
pub use rv_lock::{Lock, LockGuard};
pub use shared::{kernel_should_switch, SharedPayload, TaskState};
/// 共享调度器返回的结果
#[derive(Debug)]
#[repr(C)]
#[allow(dead_code)] // value is constructed elsewhere
pub enum TaskResult {
    /// 应当立即执行特定任务
    Task(usize),
    /// 其它地址空间的任务要运行，应当让出时间片
    ShouldYield(usize),
    /// 调度器中没有非睡眠任务
    NoWakeTask,
    /// 队列已空，所有任务已经结束
    Finished,
}

use alloc::sync::Arc;
use core::future::Future;
use core::ptr::NonNull;

/// 创建一个新的内核任务，打包它的环境
pub fn new_kernel(
    future: impl Future<Output = ()> + 'static + Send + Sync,
    process: Arc<Process>,
    shared_scheduler: NonNull<()>,
    set_task_state: unsafe extern "C" fn(NonNull<()>, usize, TaskState),
) -> Arc<KernelTaskRepr> {
    Arc::new(KernelTaskRepr(
        KernelTask::new(future, process),
        shared_scheduler.as_ptr() as usize,
        set_task_state,
    ))
}

#[derive(Debug)]
pub struct KernelTaskRepr(
    KernelTask,
    usize,
    unsafe extern "C" fn(NonNull<()>, usize, TaskState),
);

impl KernelTaskRepr {
    /// 转换到共享的任务编号
    ///
    /// note(unsafe): 创建了一个没有边界的生命周期
    pub unsafe fn task_repr(self: Arc<Self>) -> usize {
        Arc::into_raw(self) as usize
    }
    pub unsafe fn do_wake(self: &Arc<Self>) {
        let shared_scheduler = NonNull::new(self.1 as *mut ()).unwrap();
        let task_repr = Arc::as_ptr(self) as usize;
        (self.2)(shared_scheduler, task_repr, TaskState::Ready);
    }
    #[inline]
    pub fn task(&self) -> &KernelTask {
        &self.0
    }
}
