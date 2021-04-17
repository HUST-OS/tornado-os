use alloc::sync::Arc;

use super::MemorySet;

/// 一个内存映射的句柄
pub struct MemorySetHandle {
    /// 一个映射对应的 satp 寄存器值
    pub satp: usize,
    /// 一个内存映射的裸指针
    pub mms_ptr: usize,
}

impl MemorySetHandle {
    pub fn new(satp: usize, mms_ptr: usize) -> Self {
        Self {
            satp,
            mms_ptr
        }
    }

    /// 转到 MemorySet
    pub unsafe fn memory_set(&self) -> Arc<MemorySet> {
        Arc::from_raw(self.mms_ptr as *mut _)
    }
}