use lazy_static::lazy_static;
use alloc::sync::Arc;
use alloc::boxed::Box;
use core::ops::Range;
use spin::Mutex;
use crate::process::SharedAddressSpace;
use crate::memory::{AddressSpaceId, Flags, MemorySet, STACK_SIZE, VirtualAddress};

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
    /// 进程的地址空间编号
    address_space_id: AddressSpaceId,
}

impl Process {
    /// 创建一个内核进程
    ///
    /// 如果内存分配失败，返回None
    pub fn new_kernel() -> Option<Arc<Self>> {
        let shared_address_space = Box::new(SharedAddressSpace {
            address_space_id: AddressSpaceId::kernel()
        });
        let tp = Box::into_raw(shared_address_space) as usize; // todo: 这里有内存泄漏，要在drop里处理
        println!("Process::new_kernel, tp = {:x}", tp);
        unsafe { crate::hart::write_tp(tp) };
        Some(Arc::new(Process {
            id: next_process_id(),
            is_user: false,
            inner: Mutex::new(ProcessInner {
                memory_set: MemorySet::new_kernel()?,
                address_space_id: AddressSpaceId::kernel(),
            })
        }))
    }
    // /// 得到进程编号
    // pub fn process_id(&self) -> ProcessId {
    //     self.id
    // }
    /// 得到进程对应的地址空间编号
    pub fn address_space_id(&self) -> AddressSpaceId {
        self.inner.lock().address_space_id
    }
    /// 在本进程的地址空间下，分配一个新的任务栈
    pub fn alloc_stack(&self) -> Option<Range<VirtualAddress>> {
        let mut flags = Flags::READABLE | Flags::WRITABLE;
        if self.is_user {
            flags |= Flags::USER;
        }
        flags |= Flags::VALID;
        self.inner.lock().memory_set.alloc_page_range(STACK_SIZE, flags)
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
