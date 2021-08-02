use crate::memory::{PhysicalAddress, VirtualAddress};
use lazy_static::lazy_static;

pub const KERNEL_HEAP_SIZE: usize = 0x20_0000;

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



// ref: https://github.com/kaist-cp/rv6/blob/riscv/kernel-rs/src/arch/memlayout.rs
// thanks!

/// qemu puts platform-level interrupt controller (PLIC) here.
pub const PLIC_BASE: usize = 0xc000000 + KERNEL_MAP_OFFSET;

// qemu virtio disk mmio
pub const VIRTIO0: usize = 0x10001000 + KERNEL_MAP_OFFSET;
pub const VIRTIO0_IRQ: usize = 1;

