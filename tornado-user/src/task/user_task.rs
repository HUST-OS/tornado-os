use alloc::boxed::Box;
use core::fmt;
use core::future::Future;
use core::pin::Pin;
use core::sync::atomic::{AtomicUsize, Ordering};
use spin::Mutex;

/// 临时的用户态任务实现
pub struct UserTask {
    /// 任务的编号
    pub id: UserTaskId,
    /// 任务信息的可变部分
    pub inner: Mutex<UserTaskInner>,
    /// 任务的 future
    pub future: Mutex<Pin<Box<dyn Future<Output = ()> + 'static + Send + Sync>>>, // 用UnsafeCell代替Mutex会好一点
}

/// 任务信息的可变部分
#[derive(Debug)]
pub struct UserTaskInner {
    /// 任务是否在休眠
    pub sleeping: bool,
    /// 任务是否已经结束
    pub finished: bool,
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
    pub fn new(future: impl Future<Output = ()> + 'static + Send + Sync) -> UserTask {
        // 得到新的用户任务编号
        let id = UserTaskId::generate();
        // 打包成用户态任务
        UserTask {
            id,
            inner: Mutex::new(UserTaskInner {
                sleeping: false,
                finished: false,
            }),
            future: Mutex::new(Box::pin(future)),
        }
    }
}

impl fmt::Debug for UserTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = self.inner.lock();
        f.debug_struct("UserTask")
            .field("task_id", &self.id)
            .field("is sleeping", &inner.sleeping)
            .field("is finished", &inner.finished)
            .finish()
    }
}
