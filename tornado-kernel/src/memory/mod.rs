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
pub struct AddressSpaceId(usize);

impl AddressSpaceId {
    /// 得到内核的地址空间编号
    pub fn kernel() -> AddressSpaceId {
        AddressSpaceId(0)
    }
}

lazy_static::lazy_static! {
    static ref ADDRESS_SPACE_ID_COUNTER: spin::Mutex<usize> = spin::Mutex::new(1);
}

fn next_address_space_id() -> AddressSpaceId {
    let mut pid = ADDRESS_SPACE_ID_COUNTER.lock();
    let ans = *pid;
    *pid += 1;
    AddressSpaceId(ans)
}
