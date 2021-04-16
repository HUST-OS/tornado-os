//! 内存相关实现

/// 地址空间编号
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AddressSpaceId(u16); // in Sv39, [0, 2^16)

impl AddressSpaceId {
    pub(crate) unsafe fn from_raw(asid: usize) -> AddressSpaceId {
        AddressSpaceId(asid as u16)
    }
    pub(crate) fn into_inner(self) -> usize {
        self.0 as usize
    }
}