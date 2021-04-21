//! 内存相关实现

/// 地址空间编号
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct AddressSpaceId(u16); // in Sv39, [0, 2^16)

impl AddressSpaceId {
    pub(crate) fn into_inner(self) -> usize {
        self.0 as usize
    }
}
