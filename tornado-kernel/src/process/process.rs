use spin::Mutex;

use crate::memory::MemorySet;

/// 进程的所有信息
#[derive(Debug)]
pub struct Process {
    /// 进程是否属于用户态进程
    pub is_user: bool,
    /// 包装进程结构体中所有的可变变量
    pub inner: Mutex<ProcessInner>,    
}

/// 进程结构体中可变的变量
#[derive(Debug)]
pub struct ProcessInner {
    /// 进程中所有任务的公用内存映射
    pub memory_set: MemorySet,
}
