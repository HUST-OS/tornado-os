use core::ops::{Deref, DerefMut};

use crate::memory::{PhysicalPageNumber, config::PAGE_SIZE, frame::FrameTracker};
use super::page_table_entry::PageTableEntry;

// repr(C)：保证内存对齐等设置，和RISC-V标准相同
#[repr(C)]
pub struct PageTable {
    pub entries: [PageTableEntry; PAGE_SIZE / 8],
}

impl PageTable {
    pub fn clear(&mut self) {
        self.entries = [Default::default(); PAGE_SIZE / 8];
    }
}

pub struct PageTableTracker(pub FrameTracker);

impl PageTableTracker {
    pub fn new_zeroed(frame_tracker: FrameTracker) -> Self {
        let mut page_table = Self(frame_tracker);
        page_table.clear(); // PageTableTracker (deref_mut->) PageTable
        page_table
    }
    /// 获取物理页号
    pub fn page_number(&self) -> PhysicalPageNumber {
        self.0.page_number()
    }
}

impl Deref for PageTableTracker {
    type Target = PageTable;
    fn deref(&self) -> &PageTable {
        unsafe { self.0.start_address().deref_linear_static() }
    }
}

impl DerefMut for PageTableTracker {
    fn deref_mut(&mut self) -> &mut PageTable {
        unsafe { self.0.start_address().deref_linear_static() }
    }
}
