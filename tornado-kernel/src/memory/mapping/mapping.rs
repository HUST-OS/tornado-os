use crate::memory::{AddressSpaceId, PhysicalPageNumber, PhysicalAddress, VirtualAddress, VirtualPageNumber, config::PAGE_SIZE, frame::FrameTracker, frame_alloc};
use super::{Flags, MapType, Segment, page_table::{PageTable, PageTableTracker}, page_table_entry::PageTableEntry};
use alloc::{collections::VecDeque, vec::Vec};
use core::ops::Range;
use core::ptr::slice_from_raw_parts_mut;

/// 一个上下文的内存映射关系
#[derive(Debug)]
pub struct Mapping {
    /// 使用到的所有页表
    page_tables: Vec<PageTableTracker>,
    /// 根页表的物理页号
    root_ppn: PhysicalPageNumber,
    /// 虚拟页到物理页的映射信息
    mapped_pairs: VecDeque<(VirtualPageNumber, FrameTracker)>,
}

impl Mapping {
    /// 分配一个有根节点的映射，包括分配地址空间编号
    pub fn new_alloc() -> Option<Mapping> {
        let root_table = PageTableTracker::new_zeroed(frame_alloc()?);
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
                let new_table = PageTableTracker::new_zeroed(frame_alloc()?);
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
    /// 找到虚拟页号对应的页表项，如果不存在则返回 None
    pub fn find_pte(&self, vpn: VirtualPageNumber) -> Option<&mut PageTableEntry> {
        let root_table_pa = self.root_ppn.start_address();
        let root_table: &mut PageTable = unsafe { root_table_pa.deref_linear_static() };
        let mut entry = &mut root_table.entries[vpn.levels()[0]];
        for vpn_i in &vpn.levels()[1..] {
            // 如果没有页表或者页表无效
            if entry.is_empty() || !entry.is_valid() {
                return None;
            }
            // 进入下一级页表
            let next_table_pa = entry.start_address();
            let next_table: &mut PageTable = unsafe { next_table_pa.deref_linear_static() };
            entry = &mut next_table.entries[*vpn_i];
        }
        // 解引用结束，entry 位于最后一级页表
        Some(entry)
    }
    /// 地址转换
    pub fn translate(&self, vpn: VirtualPageNumber) -> Option<PageTableEntry> {
        self.find_pte(vpn).map(
            |pte| {pte.clone()}
        )
    }
    /// 插入一项虚拟页号对物理页号的映射关系，Some表示成功
    pub fn map_one(&mut self, vpn: VirtualPageNumber, ppn: Option<PhysicalPageNumber>, flags: Flags) -> Option<()> {
        // 先找到页表项
        let entry_mut = self.find_or_insert_entry(vpn)?;
        // 要插入映射关系，页表项必须是空的
        assert!(entry_mut.is_empty(), "virtual address should not already be mapped");
        // 然后向空的页表项写入内容
        *entry_mut = PageTableEntry::new(ppn, flags);
        Some(())
    }
    /// 插入并映射一个段
    pub fn map_segment(
        &mut self, segment: &Segment, init_data: Option<&[u8]>
    ) -> Option<Vec<(VirtualPageNumber, FrameTracker)>> {
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
    /// 自由映射一个段
    pub fn map_defined(
        &mut self, va_range: &Range<VirtualAddress>, pa_range: &Range<PhysicalAddress>, flags: Flags
    ) {
        let vpn_range = range_vpn_contains_va(va_range.clone());
        let ppn_range = range_vpn_contains_pa(pa_range.clone());
        self.map_range(vpn_range, ppn_range, flags);
    }
    // 映射指定的虚拟页号和物理页号
    // 不能指定初始数据
    fn map_range(
        &mut self,
        vpn_range: Range<VirtualPageNumber>,
        ppn_range: Range<PhysicalPageNumber>,
        flags: Flags
    ) {
        let mut vpn_iter = vpn_step_iter(vpn_range);
        let mut ppn_iter = ppn_step_iter(ppn_range);
        assert_eq!(vpn_iter.len(), ppn_iter.len());
        // todo: 这里应该为 (VpnRangeIter, VpnRangeIter) 实现迭代器
        // 不对，这语义太复杂，可能两个区间不相同，这样就会出现问题--luojia65
        while let (Some(vpn), Some(ppn)) = (vpn_iter.next(), ppn_iter.next()) {
            self.map_one(vpn, Some(ppn), flags);
        }
    }
    // 插入和映射线性的段
    fn map_range_linear(
        &mut self, vpn_range: Range<VirtualPageNumber>, flags: Flags, init: Option<(&[u8], Range<VirtualAddress>)>
    ) -> Option<Vec<(VirtualPageNumber, FrameTracker)>> {
        for vpn in vpn_step_iter(vpn_range) {
            self.map_one(vpn, Some(vpn.physical_page_number_linear()), flags)?;
        }
        if let Some((src_data, range)) = init {
            let target_data = unsafe { range.start.deref_virtual() } as *mut u8;
            let target_len = range.end - range.start;
            let target_slice = unsafe { &mut *slice_from_raw_parts_mut(target_data, target_len) };
            target_slice.copy_from_slice(src_data);
        }
        Some(Vec::new())
    }
    // 插入和映射按帧分页的段
    fn map_range_framed(
        &mut self, vpn_range: Range<VirtualPageNumber>, flags: Flags, init: Option<(&[u8], Range<VirtualAddress>)>
    ) -> Option<Vec<(VirtualPageNumber, FrameTracker)>> {
        let mut allocated_pairs = Vec::new();
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
            let mut frame = frame_alloc()?;
            // 更新页表
            self.map_one(vpn, Some(frame.page_number()), flags)?;
            // 写入数据
            (*frame).copy_from_slice(&page_data);
            // 保存帧跟踪器，否则会被释放
            self.mapped_pairs.push_back((vpn, frame));
        }
        Some(allocated_pairs) // todo!
    }
    /// 把当前的映射保存到satp寄存器
    pub fn activate_on(&self, asid: AddressSpaceId) {
        use riscv::register::satp::{self, Mode};
        let asid = asid.into_inner();
        unsafe {
            // 将新的ppn和asid值写到satp寄存器
            satp::set(Mode::Sv39, asid, self.root_ppn.into());
            // 刷新页表。rs1=x0、rs2=asid，说明刷新与这个地址空间有关的所有地址
            asm!("sfence.vma x0, {asid}", asid = in(reg) asid);
        }
    }
    /// 获取当前映射的satp寄存器值
    pub fn get_satp(&self, asid: AddressSpaceId) -> usize {
        // 60..64 mode
        // 44..60 asid
        // 0..44 ppn
        use riscv::register::satp::Mode;
        use bit_field::BitField;
        let mut bits = 0usize;
        bits.set_bits(60..64, Mode::Sv39 as usize);
        bits.set_bits(44..60, asid.into_inner());
        bits.set_bits(0..44, self.root_ppn.into());
        bits
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

fn range_vpn_contains_pa(src: Range<PhysicalAddress>) -> Range<PhysicalPageNumber> {
    PhysicalPageNumber::floor(src.start)..PhysicalPageNumber::ceil(src.end)
}

// 一个虚拟页号段区间的迭代器
struct VpnRangeIter {
    // 区间结束，不包含
    end_addr: usize,
    // 区间开始，包含
    current_addr: usize,
}

impl VpnRangeIter {
    pub fn len(&self) -> usize {
        self.end_addr - self.current_addr
    }
}

impl Iterator for VpnRangeIter {
    type Item = VirtualPageNumber;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_addr == self.end_addr {
            return None;
        }
        // 这里只要右移12位即可，ceil和floor区别不大
        let current_vpn = VirtualPageNumber::ceil(VirtualAddress(self.current_addr));
        let next_addr = self.current_addr.wrapping_add(PAGE_SIZE);
        self.current_addr = next_addr;
        Some(current_vpn)
    }
}

// 一个物理页号段区间的迭代器
struct PpnRangeIter {
    // 区间结束，不包含
    end_addr: usize,
    // 区间开始，包含
    current_addr: usize,
}

impl PpnRangeIter {
    pub fn len(&self) -> usize {
        self.end_addr - self.current_addr
    }
}

impl Iterator for PpnRangeIter {
    type Item = PhysicalPageNumber;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_addr == self.end_addr {
            return None;
        }
        // 这里只要右移12位即可，ceil和floor区别不大
        let current_ppn = PhysicalPageNumber::ceil(PhysicalAddress(self.current_addr));
        let next_addr = self.current_addr.wrapping_add(PAGE_SIZE);
        self.current_addr = next_addr;
        Some(current_ppn)
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

fn ppn_step_iter(src: Range<PhysicalPageNumber>) -> PpnRangeIter {
    PpnRangeIter {
        end_addr: src.end.start_address().0,
        current_addr: src.start.start_address().0,
    }
}

// 我觉得应当换一种方式，而不是用这种迭代方法 --luojia65
// impl Iterator for (VpnRangeIter, PpnRangeIter) {
//     type Item = (VirtualPageNumber, PhysicalPageNumber);
//     // todo: 这里语法应该更严格一点
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.0.current_addr == self.0.end_addr || self.1.current_addr == self.1.end_addr {
//             return None;
//         }
//         let current_vpn = VirtualPageNumber::ceil(VirtualAddress(self.0.current_addr));
//         let next_va = self.0.current_addr.wrapping_add(PAGE_SIZE);
//         self.0.current_addr = next_va;
//         let current_ppn = PhysicalPageNumber::ceil(PhysicalAddress(self.1.current_addr));
//         let next_pa = self.1.current_addr.wrapping_add(PAGE_SIZE);
//         self.1.current_addr = next_pa;
//         Some(current_vpn, current_ppn)
//     }
// }
