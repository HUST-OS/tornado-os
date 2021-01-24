use crate::memory::{
    FRAME_ALLOCATOR, config::PAGE_SIZE,
    PhysicalPageNumber, VirtualAddress, VirtualPageNumber, 
    frame::FrameTracker
};
use super::{Flags, MapType, Segment, page_table::{PageTable, PageTableTracker}, page_table_entry::PageTableEntry};
use alloc::{collections::VecDeque, vec::Vec};
use core::ops::Range;
use core::ptr::slice_from_raw_parts_mut;

/// 一个上下文的内存映射关系
pub struct Mapping {
    /// 使用到的所有页表
    page_tables: Vec<PageTableTracker>,
    /// 根页表的物理页号
    root_ppn: PhysicalPageNumber,
    /// 虚拟页到物理页的映射信息
    mapped_pairs: VecDeque<(VirtualPageNumber, FrameTracker)>,
}

impl Mapping {
    /// 分配一个有根节点的映射
    pub fn new_alloc() -> Option<Mapping> {
        let root_table = PageTableTracker::new_zeroed(FRAME_ALLOCATOR.lock().alloc()?);
        let root_ppn = root_table.page_number();
        Some(Mapping {
            page_tables: vec![root_table],
            root_ppn,
            mapped_pairs: VecDeque::new(),
        })
    }
    /// 软件找到虚拟页号对应最终的页表项。这个页表项合上偏移地址，就是物理地址了
    pub fn find_or_insert_entry(&mut self, vpn: VirtualPageNumber) -> Option<&mut PageTableEntry> {
        let root_table_pa = self.root_ppn.start_address();
        let root_table: &mut PageTable = unsafe { root_table_pa.deref_linear_static() };
        let mut entry = &mut root_table.entries[vpn.levels()[0]];
        for vpn_i in &vpn.levels()[1..] {
            // 这个地方没有页表
            if entry.is_empty() {
                // 分配一个新的页表
                let new_table = PageTableTracker::new_zeroed(FRAME_ALLOCATOR.lock().alloc()?);
                let new_ppn = new_table.page_number();
                // 把新的页表写到页表项里
                *entry = PageTableEntry::new(Some(new_ppn), Flags::VALID);
                // 保存页表的跟踪器到结构体中，结构体拥有所有权，否则新页表会被释放
                self.page_tables.push(new_table);
            }
            // 进入下一级页表
            let next_table_pa = entry.start_address();
            let next_table: &mut PageTable = unsafe { next_table_pa.deref_linear_static() };
            entry = &mut next_table.entries[*vpn_i];
        }
        // 解引用结束，entry位于最后一级页表
        Some(entry)
    }

    /// 插入一项虚拟页号对物理页号的映射关系，Some表示成功
    fn map_one(&mut self, vpn: VirtualPageNumber, ppn: Option<PhysicalPageNumber>, flags: Flags) -> Option<()> {
        // 先找到页表项
        let entry_mut = self.find_or_insert_entry(vpn)?;
        // 要插入映射关系，页表项必须是空的
        assert!(entry_mut.is_empty(), "virtual address should not already be mapped");
        // 然后向空的页表项写入内容
        *entry_mut = PageTableEntry::new(ppn, flags);
        Some(())
    }

    /// 插入并映射一个段
    pub fn map_segment(&mut self, segment: &Segment, init_data: Option<&[u8]>) -> Option<()> {
        match segment.map_type {
            MapType::Linear => self.map_range_linear(
                range_vpn_contains_va(segment.range.clone()), 
                segment.flags,
                init_data.map(|slice| (slice, segment.range.clone()))
            ),
            MapType::Framed => self.map_range_framed(
                range_vpn_contains_va(segment.range.clone()), 
                segment.flags,
                init_data.map(|slice| (slice, segment.range.clone()))
            ),
        }
    }

    // 插入和映射线性的段
    fn map_range_linear(&mut self, vpn_range: Range<VirtualPageNumber>, flags: Flags, init: Option<(&[u8], Range<VirtualAddress>)>) -> Option<()> {
        for vpn in vpn_step_iter(vpn_range) {
            self.map_one(vpn, Some(vpn.physical_page_number_linear()), flags)?;
        }
        if let Some((src_data, range)) = init {
            let target_data = unsafe { range.start.deref_virtual() } as *mut u8;
            let target_len = range.end - range.start;
            let target_slice = unsafe { &mut *slice_from_raw_parts_mut(target_data, target_len) };
            target_slice.copy_from_slice(src_data);
        }
        Some(())
    }
    // 插入和映射按帧分页的段
    fn map_range_framed(&mut self, vpn_range: Range<VirtualPageNumber>, flags: Flags, init: Option<(&[u8], Range<VirtualAddress>)>) -> Option<()> {
        for vpn in vpn_step_iter(vpn_range) {
            // 新页面的内容
            let mut page_data = [0u8; PAGE_SIZE];
            if let Some((src_data, ref va_range)) = init {
                let page_start_va = vpn.start_address();
                let start = if va_range.start > page_start_va {
                    va_range.start - page_start_va
                } else {
                    0
                };
                let end = usize::min(PAGE_SIZE, va_range.end - page_start_va);
                let dst_slice = &mut page_data[start..end];
                let src_slice = &src_data[
                    (page_start_va + start - va_range.start)
                    ..(page_start_va + end - va_range.start)
                ];
                dst_slice.copy_from_slice(src_slice);
            }
            // 分配新的页帧，用于映射
            let mut frame = FRAME_ALLOCATOR.lock().alloc()?;
            // 更新页表
            self.map_one(vpn, Some(frame.page_number()), flags)?;
            // 写入数据
            (*frame).copy_from_slice(&page_data);
            // 保存帧跟踪器，否则会被释放
            self.mapped_pairs.push_back((vpn, frame));
        }
        None
    }
    /// 把当前的映射保存到satp寄存器
    #[cfg(riscv)]
    pub fn activate(&self) {
        use riscv::{register::satp::{self, Mode}, asm};
        unsafe {
            // 保存到satp寄存器
            satp::set(Mode::Sv39, 0 /* asid */, self.root_ppn.into());
            // 刷新页表缓存
            asm::sfence_vma_all();
        }
    }
}

// 找到包含虚拟地址段的所有虚拟页
// 比如 0xFFF是一个地址最大的偏移量，包含0x11fff..0x14000的所有页有：0x11，0x12，0x13
// 此时返回 0x11..0x14 (0x11 ..= 0x13)
// 包含0x11fff..0x14001有0x11，0x12，0x13，0x14
// 返回 0x11..0x15（0x11 ..= 0x14）
fn range_vpn_contains_va(src: Range<VirtualAddress>) -> Range<VirtualPageNumber> {
    VirtualPageNumber::floor(src.start)..VirtualPageNumber::ceil(src.end)
}

// 一个虚拟页号段区间的迭代器
struct VpnRangeIter {
    end_addr: usize,
    current_addr: usize,
}

impl Iterator for VpnRangeIter {
    type Item = VirtualPageNumber;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_addr == self.end_addr {
            return None;
        }
        let next_addr = self.current_addr.wrapping_add(PAGE_SIZE);
        let next_va = VirtualAddress(next_addr);
        let next_vpn = VirtualPageNumber::ceil(next_va);
        self.current_addr = next_addr;
        Some(next_vpn)
    }
}

// 等到Step trait稳定之后，可以用trait Step的迭代器实现
// 目前先自己实现迭代器
fn vpn_step_iter(src: Range<VirtualPageNumber>) -> VpnRangeIter {
    VpnRangeIter {
        end_addr: src.end.start_address().0,
        current_addr: src.start.start_address().0,
    }
}
