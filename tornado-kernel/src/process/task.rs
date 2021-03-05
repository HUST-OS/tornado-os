use alloc::sync::Arc;
use lazy_static::lazy_static;
use spin::Mutex;
use core::ops::Range;
use core::future::Future;
use alloc::boxed::Box;
use crate::{interrupt::TrapFrame, memory::VirtualAddress};
use crate::process::{Process, SharedTaskHandle};
use core::pin::Pin;
use core::fmt;
use core::sync::atomic::{AtomicUsize, Ordering};

// lazy_static! {
//     static ref TASK_ID_COUNTER: Mutex<usize> = Mutex::new(0);
// }

/// 任务的信息
// TODO: 只是内核任务，用户任务由用户自己定义表现方式
// 如果要运行用户的进程，首先切换到用户的地址空间，其中包含一个初始化好的栈和剩余空间，然后在里面增加用户的任务
pub struct Task {
    /// 任务的编号
    pub id: TaskId,
    /// 任务所属的进程
    pub process: Arc<Process>,
    /// 任务信息的可变部分
    pub inner: Mutex<TaskInner>,
    /// 任务的内容
    pub future: Mutex<Pin<Box<dyn Future<Output = ()> + 'static + Send + Sync>>> // 用UnsafeCell代替Mutex会好一点
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
    /// 任务的执行上下文；仅当遇到中断强制暂停时，这里是Some
    pub context: Option<TrapFrame>,
    /// 任务是否正在休眠
    pub sleeping: bool,
    /// 任务是否已经结束
    pub ended: bool,
}

impl Task {
    /// 创建一个任务，将会复用执行器的栈
    pub fn new_kernel(
        future: impl Future<Output = ()> + 'static + Send + Sync,
        process: Arc<Process>,
    ) -> Arc<Task> {
        // 任务编号自增
        // let task_id = {
        //     let counter = TASK_ID_COUNTER.lock();
        //     let ans = counter.wrapping_add(1);
        //     TaskId(ans)
        // };
        let task_id = TaskId::generate();
        // 打包为任务
        Arc::new(Task {
            id: task_id,
            process,
            inner: Mutex::new(TaskInner {
                stack: None,
                context: None,
                sleeping: false,
                ended: false,
            }),
            future: Mutex::new(Box::pin(future)),
        })
    }

    /// 转换到共享的任务编号
    ///
    /// note(unsafe): 创建了一个没有边界的生命周期
    pub unsafe fn shared_task_handle(self: Arc<Self>) -> SharedTaskHandle {
        SharedTaskHandle {
            address_space_id: self.process.address_space_id(),
            task_ptr: Arc::into_raw(self) as usize
        }
    }
}

impl Task {
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

impl woke::Woke for Task {
    fn wake_by_ref(task: &Arc<Self>) {
        task.mark_ready();
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Task) -> bool {
        self.id == other.id
    }
}

impl Eq for Task {}

impl core::hash::Hash for Task {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl fmt::Debug for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = self.inner.lock();
        f.debug_struct("Task")
            .field("task id", &self.id)
            .field("address space id", &self.process.address_space_id())
            .field("stack", &inner.stack)
            .field("context", &inner.context)
            .field("is_sleeping", &inner.sleeping)
            .field("is_ended", &inner.ended)
            .finish()
    }
}