use crate::{hart::KernelHartInfo, memory::VirtualAddress};

// 注意：这个模块是没有文件系统的前提下临时使用的，未来将在用户层的库中提供
// 请记得到时候删除这个模块！这不是协程内核设计的一部分。

/// 用户态任务
/// 
/// 目前只是暂时设计，将用户态任务硬编码在内核中

use super::process::Process;
use alloc::sync::Arc;
use spin::Mutex;
use core::{ops::Range, pin::Pin};
use alloc::boxed::Box;
use core::future::Future;
use core::sync::atomic::{AtomicUsize, Ordering};
use super::SharedTaskHandle;

/// 临时的用户态任务实现

pub struct UserTask {
    /// 任务的编号
    pub id: UserTaskId,
    /// 任务所属的进程
    pub process: Arc<Process>,
    /// 任务信息的可变部分
    pub inner: Mutex<UserTaskInner>,
    /// 任务的 future
    pub future: Mutex<Pin<Box<dyn Future<Output = ()> + 'static + Send + Sync>>> // 用UnsafeCell代替Mutex会好一点
}

/// 任务信息的可变部分
pub struct UserTaskInner {
    /// 任务栈（用户态）
    pub stack: Option<Range<VirtualAddress>>,
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
        process: Arc<Process>
    ) -> Arc<UserTask> {
        // 得到新的用户任务编号
        let id = UserTaskId::generate();
        // 打包成用户态任务
        Arc::new(
            UserTask {
                id,
                process,
                inner: Mutex::new(UserTaskInner {
                    stack: None,
                    sleeping: false,
                    finished: false,
                }),
                future: Mutex::new(Box::pin(future))
            }
        )
    }

    /// 给用户态任务分配一个栈
    
    pub fn set_user_stack(&mut self, stack: Range<VirtualAddress>) {
        self.inner.lock().stack = Some(stack);
    }

    /// 转换到共享的任务编号
    /// 危险：创建了一个没有边界的生命周期
    
    pub unsafe fn shared_task_handle(self: Arc<Self>) -> SharedTaskHandle {
        SharedTaskHandle {
            hart_id: KernelHartInfo::hart_id(), 
            address_space_id: self.process.address_space_id(),
            task_ptr: Arc::into_raw(self) as usize
        }
    }
}
