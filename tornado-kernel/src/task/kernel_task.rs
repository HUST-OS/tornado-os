use crate::{memory::VirtualAddress, task::Process};
use alloc::{boxed::Box, sync::Arc};
use core::{
    fmt,
    future::Future,
    ops::Range,
    pin::Pin,
    sync::atomic::{AtomicUsize, Ordering},
};
use spin::Mutex;

/// 任务的信息
///
/// 如果要运行用户的进程，首先切换到用户的地址空间，其中包含一个初始化好的栈和剩余空间，然后在里面增加用户的任务
pub struct KernelTask {
    /// 任务的编号
    pub id: TaskId,
    /// 任务所属的进程
    pub process: Arc<Process>,
    /// 任务信息的可变部分
    pub inner: Mutex<TaskInner>,
    /// 任务的内容
    pub future: Mutex<Pin<Box<dyn Future<Output = ()> + 'static + Send + Sync>>>, // 用UnsafeCell代替Mutex会好一点
}

/// 任务的编号
#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub struct TaskId(usize);

impl TaskId {
    pub(crate) fn generate() -> TaskId {
        // 任务编号计数器，任务编号自增
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        if id > usize::max_value() / 2 {
            // TODO: 不让系统 Panic
            panic!("too many tasks!")
        }
        TaskId(id)
    }
}

/// 任务信息的可变部分
pub struct TaskInner {
    /// 本任务运行的栈
    ///
    /// 内核任务复用执行器的栈。用户任务占有一个栈，下一个任务复用此栈。强制中断暂停时，下一个任务使用新分配的栈。
    pub stack: Option<Range<VirtualAddress>>,
}

impl KernelTask {
    /// 创建一个任务，将会复用执行器的栈
    pub fn new(
        future: impl Future<Output = ()> + 'static + Send + Sync,
        process: Arc<Process>,
    ) -> KernelTask {
        // 得到新的内核任务编号
        let task_id = TaskId::generate();
        // 打包为任务
        KernelTask {
            id: task_id,
            process,
            inner: Mutex::new(TaskInner { stack: None }),
            future: Mutex::new(Box::pin(future)),
        }
    }
}

impl fmt::Debug for KernelTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = self.inner.lock();
        f.debug_struct("KernelTask")
            .field("task id", &self.id)
            .field("address space id", &self.process.address_space_id())
            .field("stack", &inner.stack)
            .finish()
    }
}
