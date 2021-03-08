use crate::memory::config::{FREE_MEMORY_START, MEMORY_END_ADDRESS, PAGE_SIZE};
use crate::memory::{Mapping, MapType, Segment, Flags, VirtualAddress, VirtualPageNumber, FrameTracker, AddressSpaceId};
use alloc::vec::Vec;
use core::ops::Range;

/// 一个地址空间中，所有与内存空间有关的信息
#[derive(Debug)]
pub struct MemorySet {
    /// 本空间的页表和映射关系
    pub mapping: Mapping,
    /// 每个字段
    pub segments: Vec<Segment>,
    /// 所有分配的物理页面映射信息
    pub allocated_pairs: Vec<(VirtualPageNumber, FrameTracker)>,
    /// 这个映射关系的地址空间编号
    pub address_space_id: AddressSpaceId
}

impl MemorySet {
    /// 创建内核重映射
    pub fn new_kernel() -> Option<MemorySet> {
        // 各个字段的起始和结束点，在连接器脚本中给出
        extern "C" {
            fn _stext();
            fn _etext();
            fn _srodata();
            fn _erodata();
            fn _sdata();
            fn _edata();
            fn _sbss();
            fn _ebss();
            fn _sshared_data();
            fn _eshared_data();
            fn _sshared_text();
            fn _eshared_text();
        }
        
        // println!("text:   {:x?}", VirtualAddress(_stext as usize)..VirtualAddress(_etext as usize));
        // println!("rodata: {:x?}", VirtualAddress(_srodata as usize)..VirtualAddress(_erodata as usize));
        // println!("data:   {:x?}", VirtualAddress(_sdata as usize)..VirtualAddress(_edata as usize));
        // println!("bss:    {:x?}", VirtualAddress(_sbss as usize)..VirtualAddress(_ebss as usize));
        // println!("shared_data: {:x?}", VirtualAddress(_sshared_data as usize)..VirtualAddress(_eshared_data as usize));
        // println!("shared_text: {:x?}", VirtualAddress(_sshared_text as usize)..VirtualAddress(_eshared_text as usize));
        // println!("free:   {:x?}", *FREE_MEMORY_START..MEMORY_END_ADDRESS.virtual_address_linear());

        // 建立字段
        let segments = vec![
            // .text 段，r-x
            Segment {
                map_type: MapType::Linear,
                range: VirtualAddress(_stext as usize)..VirtualAddress(_etext as usize),
                flags: Flags::READABLE | Flags::EXECUTABLE,
            },
            // .rodata 段，r--
            Segment {
                map_type: MapType::Linear,
                range: VirtualAddress(_srodata as usize)..VirtualAddress(_erodata as usize),
                flags: Flags::READABLE,
            },
            // .data 段，rw-
            Segment {
                map_type: MapType::Linear,
                range: VirtualAddress(_sdata as usize)..VirtualAddress(_edata as usize),
                flags: Flags::READABLE | Flags::WRITABLE,
            },
            // .bss 段，rw-
            Segment {
                map_type: MapType::Linear,
                range: VirtualAddress(_sbss as usize)..VirtualAddress(_ebss as usize),
                flags: Flags::READABLE | Flags::WRITABLE,
            },
            // 共享段的内核映射部分
            Segment {
                map_type: MapType::Linear,
                range: VirtualAddress(_sshared_data as usize)..VirtualAddress(_eshared_data as usize),
                flags: Flags::READABLE | Flags::WRITABLE
            },
            Segment {
                map_type: MapType::Linear,
                range: VirtualAddress(_sshared_text as usize)..VirtualAddress(_eshared_text as usize),
                flags: Flags::READABLE | Flags::EXECUTABLE // todo: remove READABLE
            },
            // 剩余内存空间，rw-
            Segment {
                map_type: MapType::Linear,
                range: *FREE_MEMORY_START..MEMORY_END_ADDRESS.virtual_address_linear(),
                flags: Flags::READABLE | Flags::WRITABLE,
            },
        ];
        let mut mapping = Mapping::new_alloc()?;
        // 准备保存所有新分配的物理页面
        let mut allocated_pairs = Vec::new();

        // 每个字段在页表中进行映射
        for segment in segments.iter() {
            mapping.map_segment(segment, None)?;
        }
        let address_space_id = AddressSpaceId::kernel();
        Some(MemorySet { mapping, segments, allocated_pairs, address_space_id })
    }    
    /// 检测一段内存区域和已有的是否存在重叠区域
    pub fn overlap_with(&self, range: Range<VirtualPageNumber>) -> bool {
        fn range_overlap<T: core::cmp::Ord>(a: &Range<T>, b: &Range<T>) -> bool {
            <&T>::min(&a.end, &b.end) > <&T>::max(&a.start, &b.start)
        }
        for seg in self.segments.iter() {
            if range_overlap(&range, &seg.page_range()) {
                return true;
            }
        }
        false
    }
    /// 添加一个 [`Segment`] 的内存映射
    pub fn add_segment(&mut self, segment: Segment, init_data: Option<&[u8]>) -> Option<()> {
        // 检测 segment 没有重合
        assert!(!self.overlap_with(segment.page_range()));
        // 映射并将新分配的页面保存下来
        self.allocated_pairs
            .extend(self.mapping.map_segment(&segment, init_data)?);
        self.segments.push(segment);
        Some(())
    }

    /// 分配一定数量的连续虚拟空间
    ///
    /// 在本映射中，找到一段给定长度的未占用虚拟地址空间，分配物理页面并建立映射。返回对应的页面区间。
    ///
    /// `flags` 包含r、w、x和user。
    pub fn alloc_page_range(
        &mut self,
        size: usize,
        flags: Flags,
    ) -> Option<Range<VirtualAddress>> {
        // memory_set 只能按页分配，所以让 size 向上取整页
        let alloc_size = (size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
        // 从 memory_set 中找一段不会发生重叠的空间
        let mut range = VirtualAddress(0x1000000)..VirtualAddress(0x1000000 + alloc_size);
        while self.overlap_with(range_vpn_from_range_va(&range)) {
            range.start += alloc_size;
            range.end += alloc_size;
        }
        // 分配物理页面，建立映射
        self.add_segment(
            Segment {
                map_type: MapType::Framed,
                range: range.clone(),
                flags,
            },
            None,
        )?;
        // 返回地址区间（使用参数 size，而非向上取整的 alloc_size）
        Some(range.start..(range.start + size))
    }

    /// 替换 `satp` 以激活页表
    ///
    /// 如果当前页表就是自身，则不会替换，但仍然会刷新 TLB。
    pub fn activate(&self) {
        println!("Activating memory set in asid {:?}", self.address_space_id);
        self.mapping.activate(self.address_space_id)
    }
}

fn range_vpn_from_range_va(src: &Range<VirtualAddress>) -> Range<VirtualPageNumber> {
    VirtualPageNumber::floor(src.start)..VirtualPageNumber::floor(src.end.into())
}
