mod address;
mod config;
mod frame;
mod mapping;

pub use address::{PhysicalAddress, PhysicalPageNumber, VirtualAddress, VirtualPageNumber};
pub use config::*;
pub use frame::{frame_alloc, FrameTracker};
pub use mapping::{Flags, MapType, Mapping, MemorySet, Satp, Segment};

// pub fn init() {
//     heap::init();
// }

pub type AddressSpaceId = crate::mm::AddressSpaceId;
