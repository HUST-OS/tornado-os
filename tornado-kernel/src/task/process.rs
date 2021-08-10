//! unused
use crate::hart::KernelHartInfo;
use crate::memory::{AddressSpaceId, Flags, MemorySet, VirtualAddress, STACK_SIZE};
use alloc::sync::Arc;
use core::ops::Range;
use lazy_static::lazy_static;
use spin::Mutex;

/// 进程的所有信息
#[derive(Debug)]
pub struct Process {
    /// 进程的编号
    id: ProcessId,
    /// 进程是否属于用户态进程
    is_user: bool,
    /// 可变部分
    inner: Mutex<ProcessInner>,
}

/// 进程所有信息的可变部分
#[derive(Debug)]
pub struct ProcessInner {
    /// 进程中所有任务的公用内存映射
    memory_set: MemorySet,
}

impl Process {
    /// 创建一个进程
    ///
    /// 如果内存分配失败，返回[`None`]
    pub fn new(memory_set: MemorySet) -> Option<Arc<Self>> {
        let process = Arc::new(Process {
            id: next_process_id(),
            is_user: false,
            inner: Mutex::new(ProcessInner { memory_set }),
        });
        unsafe {
            KernelHartInfo::load_address_space_id(process.address_space_id());
            KernelHartInfo::load_process(process.clone());
        };
        Some(process)
    }

    /// 创建一个用户进程
    ///
    /// 暂时和创建内核进程无太大区别，后续会思考这部分设计
    pub fn new_user(memory_set: MemorySet) -> Option<Arc<Self>> {
        let process = Arc::new(Process {
            id: next_process_id(),
            is_user: true,
            inner: Mutex::new(ProcessInner { memory_set }),
        });
        Some(process)
    }

    /// 得到进程*所在*的地址空间编号。进程不*对应*地址空间编号
    pub fn address_space_id(&self) -> AddressSpaceId {
        self.inner.lock().memory_set.address_space_id
    }

    /// 在本进程的地址空间下，分配一个新的任务栈
    pub fn alloc_stack(&self) -> Option<Range<VirtualAddress>> {
        let mut flags = Flags::READABLE | Flags::WRITABLE;
        if self.is_user {
            flags |= Flags::USER;
        }
        flags |= Flags::VALID;
        self.inner
            .lock()
            .memory_set
            .alloc_page_range(STACK_SIZE, flags)
    }
}

/// 进程的编号
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProcessId(usize);

lazy_static! {
    pub static ref PROCESS_COUNTER: Mutex<usize> = Mutex::new(1);
}

fn next_process_id() -> ProcessId {
    let mut pid = PROCESS_COUNTER.lock();
    let ans = *pid;
    *pid += 1;
    ProcessId(ans)
}
