use alloc::sync::Arc;
use spin::Mutex;
use core::ops::Range;
use core::future::Future;
use alloc::boxed::Box;
use crate::memory::VirtualAddress;
use crate::task::Process;
use core::pin::Pin;
use core::fmt;
use core::sync::atomic::{AtomicUsize, Ordering};

/// 任务的信息
// TODO: 只是内核任务，用户任务由用户自己定义表现方式
// 如果要运行用户的进程，首先切换到用户的地址空间，其中包含一个初始化好的栈和剩余空间，然后在里面增加用户的任务
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
    ) -> Arc<KernelTask> {
        // 得到新的内核任务编号
        let task_id = TaskId::generate();
        // 打包为任务
        Arc::new(KernelTask {
            id: task_id,
            process,
            inner: Mutex::new(TaskInner {
                stack: None,
            }),
            future: Mutex::new(Box::pin(future)),
        })
    }

    /// 转换到共享的任务编号
    ///
    /// note(unsafe): 创建了一个没有边界的生命周期
    pub unsafe fn task_repr(self: Arc<Self>) -> usize {
        Arc::into_raw(self) as usize
    }
}

impl PartialEq for KernelTask {
    fn eq(&self, other: &KernelTask) -> bool {
        self.id == other.id
    }
}

impl Eq for KernelTask {}

impl core::hash::Hash for KernelTask {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl fmt::Debug for KernelTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = self.inner.lock();
        f.debug_struct("KernelTask")
            .field("task isd", &self.id)
            .field("address space id", &self.process.address_space_id())
            .field("stack", &inner.stack)
            .finish()
    }
}
