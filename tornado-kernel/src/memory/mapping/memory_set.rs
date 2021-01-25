use crate::memory::mapping::{Mapping, MapType, Segment, Flags};
use crate::memory::config::{FREE_MEMORY_START, MEMORY_END_ADDRESS};
use crate::memory::VirtualAddress;
use alloc::vec::Vec;

/// 一个上下文中，所有与内存空间有关的信息
pub struct MemorySet {
    /// 本上下文的页表和映射关系
    pub mapping: Mapping,
    /// 每个字段
    pub segments: Vec<Segment>,
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
        }
        
        println!("text:   {:x?}", VirtualAddress(_stext as usize)..VirtualAddress(_etext as usize));
        println!("rodata: {:x?}", VirtualAddress(_srodata as usize)..VirtualAddress(_erodata as usize));
        println!("data:   {:x?}", VirtualAddress(_sdata as usize)..VirtualAddress(_edata as usize));
        println!("bss:    {:x?}", VirtualAddress(_sbss as usize)..VirtualAddress(_ebss as usize));
        println!("free:   {:x?}", *FREE_MEMORY_START..MEMORY_END_ADDRESS.virtual_address_linear());

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
            // 剩余内存空间，rw-
            Segment {
                map_type: MapType::Linear,
                range: *FREE_MEMORY_START..MEMORY_END_ADDRESS.virtual_address_linear(),
                flags: Flags::READABLE | Flags::WRITABLE,
            },
        ];
        let mut mapping = Mapping::new_alloc()?;

        // 每个字段在页表中进行映射
        for segment in segments.iter() {
            mapping.map_segment(segment, None)?;
        }
        Some(MemorySet { mapping, segments })
    }

    /// 替换 `satp` 以激活页表
    ///
    /// 如果当前页表就是自身，则不会替换，但仍然会刷新 TLB。
    pub fn activate(&self) {
        self.mapping.activate()
    }
}
