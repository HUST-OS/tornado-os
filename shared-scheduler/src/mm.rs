//! 内存相关
/// 地址空间编号
///
/// 内核地址空间编号为0，用户地址空间编号从1开始增长
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct AddressSpaceId(u16); // in Sv39, [0, 2^16)

impl AddressSpaceId {
    /// 地址空间编号转为[`usize`]
    pub(crate) fn into_inner(self) -> usize {
        self.0 as usize
    }
}
