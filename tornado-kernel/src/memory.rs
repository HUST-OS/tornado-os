mod address;
mod config;
mod heap;
mod frame;

pub use address::{PhysicalAddress, PhysicalPageNumber};
pub(crate) use frame::FRAME_ALLOCATOR;

pub fn init() {
    heap::init()
}
