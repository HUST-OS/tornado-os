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
