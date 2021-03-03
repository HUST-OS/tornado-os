mod task;
mod process;
mod executor;
mod lock;

pub use lock::Lock;
pub use task::{Task, TaskId};
pub use process::{Process, ProcessId};

use crate::algorithm::{Scheduler, FifoScheduler};
use crate::hart::ThreadPointer;

/// 所有任务的调度器
///
/// 注意：所有在.shared_data段分配的堆空间，必须也在.shared_data段里面，否则另一边的进程就无法访问共享的堆内存
#[link_section = ".shared_data"]
pub static SHARED_SCHEDULER: spin::Mutex<FifoScheduler<SharedTaskHandle>> = 
    spin::Mutex::new(FifoScheduler::new());

/// 共享的包含Future在用户空间的地址
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SharedTaskHandle {
    /// 地址空间的编号
    pub(crate) address_space_id: ProcessId,
    /// 对每个虚拟空间来说，task_ptr是Arc<Task>相应的虚拟地址
    /// 比如内核中是内核虚拟地址，用户中是用户的虚拟地址
    pub(crate) task_ptr: usize,
}

impl SharedTaskHandle {
    fn should_switch(&self) -> bool {
        // 如果当前和下一个任务间地址空间变化了，就说明应当切换上下文
        SharedAddressSpace::current().address_space_id != self.address_space_id
    }
}

#[link_section = ".shared_text"]
#[allow(unused)]
pub fn shared_add_task(handle: SharedTaskHandle) {
    SHARED_SCHEDULER.lock().add_task(handle);
}

#[link_section = ".shared_text"]
#[allow(unused)]
pub fn shared_pop_task() -> TaskResult {
    let mut scheduler = SHARED_SCHEDULER.lock();
    if let Some(task) = scheduler.peek_next_task() {
        if task.should_switch() {
            return TaskResult::ShouldYield
        }
        // note(unwrap): 前面peek已经返回Some了
        let next_task = scheduler.next_task().unwrap();
        drop(scheduler);
        TaskResult::Task(next_task)
    } else {
        TaskResult::Finished
    }
}

/// 共享调度器返回的结果
pub enum TaskResult {
    /// 应当立即执行特定任务
    Task(SharedTaskHandle),
    /// 其它地址空间的任务要运行，应当让出时间片
    ShouldYield,
    /// 队列已空，所有任务已经结束
    Finished,
}

/// 共享的地址空间
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SharedAddressSpace {
    /// 当前的进程编号
    pub address_space_id: ProcessId,
}

impl SharedAddressSpace {
    fn current<'a>() -> &'a SharedAddressSpace {
        unsafe { ThreadPointer::as_ref().expect("get current context") }
    }
}
