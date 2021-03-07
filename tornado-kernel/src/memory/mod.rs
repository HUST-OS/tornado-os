mod address;
mod config;
mod heap;
mod frame;
mod mapping;

pub use address::{PhysicalAddress, PhysicalPageNumber, VirtualAddress, VirtualPageNumber};
pub use mapping::{MemorySet, Mapping, Segment, MapType, Flags};
pub use frame::{frame_alloc, FrameTracker};
pub use config::*;

pub fn init() {
    heap::init();
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AddressSpaceId(u16); // in Sv39, [0, 2^16)

impl AddressSpaceId {
    /// 得到内核的地址空间编号
    pub fn kernel() -> AddressSpaceId {
        AddressSpaceId(0)
    }
}

use alloc::vec::Vec;

lazy_static::lazy_static! {
    static ref ADDRESS_SPACE_ID_COUNTER: spin::Mutex<(Vec<usize>, usize)> = 
        spin::Mutex::new((Vec::new(), 0)); // 剩余的空间；
}

pub fn riscv_max_asid() -> AddressSpaceId {
    #[cfg(target_pointer_width = "64")]
    let mut val: usize = ((1 << 16) - 1) << 44;
    #[cfg(target_pointer_width = "32")]
    let mut val: usize = ((1 << 9) - 1) << 22;
    unsafe { asm!("
        csrr    {tmp}, satp
        or      {val}, {tmp}, {val}
        csrw    satp, {val}
        csrrw   {val}, satp, {tmp}
    ", tmp = out(reg) _, val = inlateout(reg) val) };
    #[cfg(target_pointer_width = "64")]
    return AddressSpaceId(((val >> 44) & ((1 << 16) - 1)) as u16);
    #[cfg(target_pointer_width = "32")]
    return AddressSpaceId(((val >> 22) & ((1 << 9) - 1)) as u16);
}

// fn next_address_space_id() -> AddressSpaceId {
//     let mut pid = ADDRESS_SPACE_ID_COUNTER.lock();
//     let ans = *pid;
//     *pid += 1;
//     AddressSpaceId(ans)
// }
