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

