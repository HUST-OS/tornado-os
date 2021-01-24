mod address;
mod config;
mod heap;
mod frame;
mod mapping;

pub use address::{PhysicalAddress, PhysicalPageNumber, VirtualAddress, VirtualPageNumber};
pub use mapping::{MemorySet, Mapping, Segment, MapType, Flags};
pub(crate) use frame::FRAME_ALLOCATOR;

pub fn init() {
    heap::init();
}

