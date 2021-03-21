use crate::memory::{PhysicalAddress, PhysicalPageNumber};
use bit_field::BitField;

#[derive(Copy, Clone, Default)]
pub struct PageTableEntry(usize);

const FLAG_RANGE: core::ops::Range<usize> = 0..8;
const PAGE_NUMBER_RANGE: core::ops::Range<usize> = 10..54;

impl PageTableEntry {
    /// 创建一个新的页表项
    pub fn new(page_number: Option<PhysicalPageNumber>, mut flags: Flags) -> Self {
        flags.set(Flags::VALID, page_number.is_some());
        let inner = *0usize
            .set_bits(FLAG_RANGE, flags.bits())
            .set_bits(PAGE_NUMBER_RANGE, page_number
                .map(|a| a.into()).unwrap_or(0));
        PageTableEntry(inner)
    }
    /// 获取页号
    pub fn page_number(&self) -> PhysicalPageNumber {
        PhysicalPageNumber::from(self.0.get_bits(10..54))
    }
    /// 获取包含的物理页的起始地址，用于软件找到下一级页表的位置
    pub fn start_address(&self) -> PhysicalAddress {
        self.page_number().start_address()
    }
    /// 获取标志位
    pub fn flags(&self) -> Flags {
        unsafe { Flags::from_bits_unchecked(self.0.get_bits(FLAG_RANGE)) }
    }
    /// 如果为空，说明页表不存在，需要分配新的页表
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
    /// 是否有效
    pub fn is_valid(&self) -> bool {
        (self.flags() & Flags::VALID) != Flags::empty()
    }
}

impl core::fmt::Debug for PageTableEntry {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter
            .debug_struct("PageTableEntry")
            .field("value", &self.0)
            .field("page_number", &self.page_number())
            .field("flags", &self.flags())
            .finish()
    }
}

bitflags::bitflags! {
    /// 页表项中的 8 个标志位
    #[derive(Default)]
    pub struct Flags: usize {
        /// 有效位
        const VALID =       1 << 0;
        /// 可读位
        const READABLE =    1 << 1;
        /// 可写位
        const WRITABLE =    1 << 2;
        /// 可执行位
        const EXECUTABLE =  1 << 3;
        /// 用户位
        const USER =        1 << 4;
        /// 全局位
        const GLOBAL =      1 << 5;
        /// 已使用位，用于替换算法
        const ACCESSED =    1 << 6;
        /// 已修改位，用于替换算法
        const DIRTY =       1 << 7;
    }
}
