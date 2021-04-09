use crate::memory::{PhysicalAddress, VirtualAddress};
use lazy_static::lazy_static;

pub const KERNEL_HEAP_SIZE: usize = 0x80_0000;

pub const MEMORY_END_ADDRESS: PhysicalAddress = PhysicalAddress(0x8800_0000);

lazy_static! {
    pub static ref FREE_MEMORY_START: VirtualAddress = {
        extern "C" {
            fn free_memory_start();
        }
        VirtualAddress(free_memory_start as usize)
    };
}

pub const PAGE_SIZE: usize = 4096;

pub const KERNEL_MAP_OFFSET: usize = 0xffff_ffff_4000_0000;

/// 每个线程的运行栈大小 512 KB
pub const STACK_SIZE: usize = 0x8_0000;

/// .swap 段的虚拟地址，用户和内核在该地址上有相同的映射关系
/// 映射关系的虚拟地址是地址空间的最高处（不管是用户还是内核）
pub const SWAP_FRAME_VA: usize = usize::MAX - PAGE_SIZE + 1;

/// 用户态和内核态切换时上下文保存的地址
/// 用户和内核在该地址上同样有相同的映射关系
pub const SWAP_CONTEXT_VA: usize = SWAP_FRAME_VA - PAGE_SIZE;

/// 用户态栈的虚拟地址
/// 放在切换时保存的上下文下面
/// todo: 这个后面要移除
pub const USER_STACK_BOTTOM_VA: usize = SWAP_CONTEXT_VA - PAGE_SIZE;

/// 共享数据段在用户态中的虚拟地址
pub const USER_SHARED_DATA_VA: usize = 0xffff_ffff_ffff_0000;

/// 共享代码段在用户态中的虚拟地址
pub const USER_SHARED_TEXT_VA: usize = 0xffff_ffff_fffe_0000;
