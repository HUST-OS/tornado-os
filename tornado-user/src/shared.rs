//！ 尝试在用户态给共享调度器添加任务

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct SharedTaskHandle {
    /// 处理核编号
    pub(crate) hart_id: usize,
    /// 地址空间编号
    pub(crate) address_space_id: AddressSpaceId,
    /// task_ptr 是 Arc<Task> 的虚拟地址
    pub(crate) task_ptr: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AddressSpaceId(u16);

impl AddressSpaceId {
    pub(crate) unsafe fn from_raw(asid: usize) -> AddressSpaceId {
        AddressSpaceId(asid as u16)
    }
    pub(crate) fn into_inner(self) -> usize {
        self.0 as usize
    }
}