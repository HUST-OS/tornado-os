use alloc::sync::Arc;
use lazy_static::lazy_static;
use spin::Mutex;
use core::ops::Range;
use core::future::Future;
use alloc::boxed::Box;
use crate::{interrupt::TrapFrame, memory::VirtualAddress};
use crate::process::{Process, SharedTaskHandle, SharedAddressSpace};
use core::pin::Pin;

lazy_static! {
    static ref TASK_ID_COUNTER: Mutex<usize> = Mutex::new(0);
}

/// 任务的信息
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

/// 任务信息的可变部分
pub struct TaskInner {
    /// 本任务运行的栈；任务被强制中断暂停时，下一个任务使用新分配的栈
    pub stack: Range<VirtualAddress>,
    /// 任务的执行上下文；仅当遇到中断强制暂停时，这里是Some
    pub context: Option<TrapFrame>,
    /// 任务是否正在休眠
    pub sleeping: bool,
    /// 任务是否已经结束
    pub ended: bool,
}

impl Task {
    /// 创建一个任务，需要输入创建好的栈
    pub fn new_kernel(
        future: impl Future<Output = ()> + 'static + Send + Sync,
        process: Arc<Process>,
        stack: Range<VirtualAddress>
    ) -> Arc<Task> {
        // 构建上下文
        let stack_top: usize = stack.end.into();
        let context = TrapFrame::new_task_context(
            false,
            0,
            0,
            stack_top
        ); // todo: 逻辑是不是错了？
        // 任务编号自增
        let task_id = {
            let counter = TASK_ID_COUNTER.lock();
            let ans = counter.wrapping_add(1);
            TaskId(ans)
        };
        // 打包为任务
        Arc::new(Task {
            id: task_id,
            process,
            inner: Mutex::new(TaskInner {
                stack,
                context: Some(context),
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