use alloc::sync::Arc;
use core::future::Future;
use core::ptr::NonNull;
use shared::TaskState;
use user_task::UserTask;

pub mod shared;
pub mod user_task;

/// 共享调度器返回的结果
#[derive(Debug)]
pub enum TaskResult {
    /// 应当立即执行特定任务
    Task(usize, usize),
    /// 其它地址空间的任务要运行，应当让出时间片
    ShouldYield(usize, usize),
    /// 调度器中没有非睡眠任务
    NoWakeTask,
    /// 队列已空，所有任务已经结束
    Finished,
}

// 创建一个新的用户任务，打包它的环境
pub fn new_user(
    future: impl Future<Output = ()> + 'static + Send + Sync,
    shared_scheduler: NonNull<()>,
    set_task_state: unsafe extern "C" fn(NonNull<()>, usize, TaskState),
) -> Arc<UserTaskRepr> {
    Arc::new(UserTaskRepr(
        UserTask::new(future),
        shared_scheduler.as_ptr() as usize,
        set_task_state,
    ))
}

#[derive(Debug)]
pub struct UserTaskRepr(
    UserTask,
    usize,
    unsafe extern "C" fn(NonNull<()>, usize, TaskState),
);

impl UserTaskRepr {
    /// 转换到共享的任务编号
    ///
    /// note(unsafe): 创建了一个没有边界的生命周期
    pub unsafe fn task_repr(self: Arc<Self>) -> usize {
        Arc::into_raw(self) as usize
    }
    pub unsafe fn do_wake(self: &Arc<Self>) {
        let shared_scheduler = NonNull::new(self.1 as *mut ()).unwrap();
        let task_repr = Arc::as_ptr(self) as usize;
        (self.2)(shared_scheduler, task_repr, TaskState::Ready)
    }
    #[inline]
    pub fn task(&self) -> &UserTask {
        &self.0
    }
}

impl woke::Woke for UserTaskRepr {
    fn wake_by_ref(task: &Arc<Self>) {
        unsafe { task.do_wake() }
    }
}
