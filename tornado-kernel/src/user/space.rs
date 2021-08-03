//! 管理分配给用户程序的内存空间
use crate::memory::AddressSpaceId;

const BASE: usize = 0x8600_0000;

/// 用户内存管理器
///
/// N: 最大 N 页内存
/// B: 用户空间起始地址
pub struct UserSpaceManager<const N: usize, const B: usize> {
    data: [AddressSpaceId; N]
}

