mod address;
mod config;
mod heap;
mod frame;
mod mapping;

pub use address::{PhysicalAddress, PhysicalPageNumber, VirtualAddress, VirtualPageNumber};
pub use mapping::{MemorySet, Mapping, Segment, MapType, Flags, MemorySetHandle, Satp};
pub use frame::{frame_alloc, FrameTracker};
pub use config::*;

pub fn init() {
    heap::init();
}

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

pub fn max_asid() -> AddressSpaceId {
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
