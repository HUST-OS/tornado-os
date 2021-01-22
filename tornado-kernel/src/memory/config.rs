use crate::memory::PhysicalAddress;
use lazy_static::lazy_static;

pub const KERNEL_HEAP_SIZE: usize = 0x80_0000;

pub const MEMORY_END_ADDRESS: PhysicalAddress = PhysicalAddress(0x8800_0000);

lazy_static! {
    pub static ref KERNEL_END_ADDRESS: PhysicalAddress = {
        extern "C" {
            fn kernel_end();
        }
        PhysicalAddress(kernel_end as usize)
    };
}

pub const PAGE_SIZE: usize = 4096;
