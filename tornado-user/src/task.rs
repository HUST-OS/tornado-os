// todo：重新整理


// 在用户的库中提供

/// 用户态任务
/// 
/// 目前只是暂时设计，将用户态任务硬编码在内核中

use alloc::sync::Arc;
use spin::Mutex;
use core::pin::Pin;
use alloc::boxed::Box;
use core::future::Future;
use core::sync::atomic::{AtomicUsize, Ordering};

/// 临时的用户态任务实现

pub struct UserTask {
    /// 任务的编号
    pub id: UserTaskId,
    /// 任务信息的可变部分
    pub inner: Mutex<UserTaskInner>,
    /// 任务的 future
    pub future: Mutex<Pin<Box<dyn Future<Output = ()> + 'static + Send + Sync>>> // 用UnsafeCell代替Mutex会好一点
}

/// 任务信息的可变部分
pub struct UserTaskInner {
    /// 任务是否在休眠
    pub sleeping: bool,
    /// 任务是否已经结束
    pub finished: bool
}

/// 用户任务的编号
#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub struct UserTaskId(usize);

impl UserTaskId {
    pub(crate) fn generate() -> UserTaskId {
        // 任务编号计数器，任务编号自增
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        if id > usize::max_value() / 2 {
            // TODO: 不让系统 Panic
            panic!("too many tasks!")
        }
        UserTaskId(id)
    }
}

impl UserTask {
    /// 创建一个用户态任务
    pub fn new(
        future: impl Future<Output = ()> + 'static + Send + Sync,
    ) -> Arc<UserTask> {
        // 得到新的用户任务编号
        let id = UserTaskId::generate();
        // 打包成用户态任务
        Arc::new(
            UserTask {
                id,
                inner: Mutex::new(UserTaskInner {
                    sleeping: false,
                    finished: false,
                }),
                future: Mutex::new(Box::pin(future))
            }
        )
    }

    /// 转换到共享的任务编号
    /// 危险：创建了一个没有边界的生命周期
    
    pub unsafe fn task_repr(self: Arc<Self>) -> usize {
        Arc::into_raw(self) as usize
    }
}

impl UserTask {
    fn mark_ready(&self) {
        self.inner.lock().sleeping = false;
    }
    pub(crate) fn is_sleeping(&self) -> bool {
        self.inner.lock().sleeping
    }

    pub(crate) fn mark_sleep(&self) {
        self.inner.lock().sleeping = true;
    }
}

impl woke::Woke for UserTask {
    fn wake_by_ref(task: &Arc<Self>) {
        task.mark_ready();
    }
}

/// 共享调度器返回的结果
#[derive(Debug)]
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
