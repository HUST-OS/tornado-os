use crate::memory::{
    config::{FREE_MEMORY_START, MEMORY_END_ADDRESS, PAGE_SIZE, SWAP_FRAME_VA},
    PhysicalPageNumber, KERNEL_MAP_OFFSET, PLIC_BASE, swap_contex_va, VIRTIO0,
};
use crate::memory::{
    AddressSpaceId, Flags, FrameTracker, MapType, Mapping, PhysicalAddress, Segment,
    VirtualAddress, VirtualPageNumber,
};
use crate::SHAREDPAYLOAD_BASE;
use alloc::{sync::Arc, vec::Vec};
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
    pub address_space_id: AddressSpaceId,
}

impl MemorySet {
    /// 创建内核重映射
    pub fn new_kernel() -> Option<MemorySet> {
        // 各个字段的起始和结束点，在链接器脚本中给出
        extern "C" {
            fn _stext();
            fn _etext();
            fn _srodata();
            fn _erodata();
            fn _sdata();
            fn _edata();
            fn _sbss();
            fn _ebss();
            fn _swap_frame();
        }

        println!(
            "text:   {:x?}",
            VirtualAddress(_stext as usize)..VirtualAddress(_etext as usize)
        );
        println!(
            "rodata: {:x?}",
            VirtualAddress(_srodata as usize)..VirtualAddress(_erodata as usize)
        );
        println!(
            "data:   {:x?}",
            VirtualAddress(_sdata as usize)..VirtualAddress(_edata as usize)
        );
        println!(
            "bss:    {:x?}",
            VirtualAddress(_sbss as usize)..VirtualAddress(_ebss as usize)
        );
        println!(
            "swap frame: {:x?}",
            VirtualAddress(_swap_frame as usize)..VirtualAddress(_etext as usize)
        );
        println!(
            "free:   {:x?}",
            *FREE_MEMORY_START..MEMORY_END_ADDRESS.virtual_address_linear()
        );

        // 建立字段
        let segments = vec![
            // .text 段，r-x
            Segment {
                map_type: MapType::Linear,
                range: VirtualAddress(_stext as usize)..VirtualAddress(_swap_frame as usize),
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
        // 准备保存所有新分配的物理页面
        let allocated_pairs = Vec::new();

        // 每个字段在页表中进行映射
        for segment in segments.iter() {
            mapping.map_segment(segment, None)?;
        }

        map_mmio(&mut mapping);

        // 映射共享载荷，目前地址是写死的
        let va_range =
            VirtualAddress(SHAREDPAYLOAD_BASE)..VirtualAddress(SHAREDPAYLOAD_BASE + 0x40_0000);
        let pa_range =
            PhysicalAddress(SHAREDPAYLOAD_BASE)..PhysicalAddress(SHAREDPAYLOAD_BASE + 0x40_0000);
        mapping.map_defined(
            &va_range,
            &pa_range,
            Flags::WRITABLE | Flags::READABLE | Flags::EXECUTABLE,
        );

        // 映射 _swap_frame
        let swap_frame_va = VirtualAddress(SWAP_FRAME_VA);
        let swap_frame_vpn = VirtualPageNumber::floor(swap_frame_va);
        let swap_frame_pa = VirtualAddress(_swap_frame as usize).physical_address_linear();
        let swap_frame_ppn = PhysicalPageNumber::floor(swap_frame_pa);
        mapping.map_one(
            swap_frame_vpn,
            Some(swap_frame_ppn),
            Flags::EXECUTABLE | Flags::READABLE | Flags::WRITABLE,
        )?;

        let address_space_id = crate::hart::KernelHartInfo::alloc_address_space_id()?;
        println!("Kernel new asid = {:?}", address_space_id);

        Some(MemorySet {
            mapping,
            segments,
            allocated_pairs,
            address_space_id,
        })
    }

    /// 通过一个 bin 文件创建用户态映射
    pub fn new_bin(base: usize, pages: usize, asid: AddressSpaceId) -> Option<MemorySet> {
        extern "C" {
            fn _swap_frame();
        }
        let mut mapping = Mapping::new_alloc()?;
        let allocated_pairs = Vec::new();

        let va_range = VirtualAddress(0)..VirtualAddress(PAGE_SIZE * pages);
        let pa_range = PhysicalAddress(base)..PhysicalAddress(base + PAGE_SIZE * pages);
        mapping.map_defined(
            &va_range,
            &pa_range,
            Flags::EXECUTABLE | Flags::READABLE | Flags::WRITABLE | Flags::USER,
        );

        // 映射 _swap_frame
        let swap_frame_va = VirtualAddress(SWAP_FRAME_VA);
        let swap_frame_vpn = VirtualPageNumber::floor(swap_frame_va);
        let swap_frame_pa = VirtualAddress(_swap_frame as usize).physical_address_linear();
        let swap_frame_ppn = PhysicalPageNumber::floor(swap_frame_pa);
        mapping.map_one(
            swap_frame_vpn,
            Some(swap_frame_ppn),
            Flags::EXECUTABLE | Flags::READABLE | Flags::WRITABLE,
        );

        // 映射 SwapContext
        let swap_cx_va = VirtualAddress(swap_contex_va(asid.into_inner()));
        mapping.map_segment(
            &Segment {
                map_type: MapType::Framed,
                range: swap_cx_va..swap_cx_va + PAGE_SIZE,
                flags: Flags::READABLE | Flags::WRITABLE,
            },
            None,
        )?;

        // 映射共享负荷
        let va_range =
            VirtualAddress(SHAREDPAYLOAD_BASE)..VirtualAddress(SHAREDPAYLOAD_BASE + 0x80_0000);
        let pa_range =
            PhysicalAddress(SHAREDPAYLOAD_BASE)..PhysicalAddress(SHAREDPAYLOAD_BASE + 0x80_0000);
        mapping.map_defined(
            &va_range,
            &pa_range,
            Flags::WRITABLE | Flags::READABLE | Flags::EXECUTABLE | Flags::USER,
        );

        Some(MemorySet {
            mapping,
            segments: Vec::new(),
            allocated_pairs,
            address_space_id: asid,
        })
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
    pub fn alloc_page_range(&mut self, size: usize, flags: Flags) -> Option<Range<VirtualAddress>> {
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
        self.mapping.activate_on(self.address_space_id)
    }
}

fn range_vpn_from_range_va(src: &Range<VirtualAddress>) -> Range<VirtualPageNumber> {
    VirtualPageNumber::floor(src.start)..VirtualPageNumber::floor(src.end.into())
}

#[cfg(feature = "qemu")]
fn map_mmio(mapping: &mut Mapping) {
    // 映射 PLIC
    let plic_va_start = VirtualAddress(PLIC_BASE);
    let plic_va_end = VirtualAddress(PLIC_BASE + 0x400000);
    mapping.map_defined(
        &(plic_va_start..plic_va_end),
        &(plic_va_start.physical_address_linear()..plic_va_end.physical_address_linear()),
        Flags::READABLE | Flags::WRITABLE,
    );

    // 映射 virtio disk mmio
    let virtio_va = VirtualAddress(VIRTIO0);
    let virtio_pa = VirtualAddress(VIRTIO0).physical_address_linear();
    mapping.map_one(
        VirtualPageNumber::floor(virtio_va),
        Some(PhysicalPageNumber::floor(virtio_pa)),
        Flags::WRITABLE | Flags::READABLE,
    );
}

#[cfg(feature = "k210")]
fn map_mmio(mapping: &mut Mapping) {
    // (0x3800_0000, 0x1000),      /* UARTHS    */
    let va = VirtualAddress(0x3800_0000);
    let pa = PhysicalAddress(0x3800_0000);
    mapping.map_one(
        VirtualPageNumber::floor(va),
        Some(PhysicalPageNumber::floor(pa)),
        Flags::WRITABLE | Flags::READABLE,
    );

    // (0x3800_1000, 0x1000),      /* GPIOHS    */
    let va = VirtualAddress(0x3800_1000);
    let pa = PhysicalAddress(0x3800_1000);
    mapping.map_one(
        VirtualPageNumber::floor(va),
        Some(PhysicalPageNumber::floor(pa)),
        Flags::WRITABLE | Flags::READABLE,
    );

    // (0x5020_0000, 0x1000),      /* GPIO      */
    let va = VirtualAddress(0x5020_0000);
    let pa = PhysicalAddress(0x5020_0000);
    mapping.map_one(
        VirtualPageNumber::floor(va),
        Some(PhysicalPageNumber::floor(pa)),
        Flags::WRITABLE | Flags::READABLE,
    );

    // (0x5024_0000, 0x1000),      /* SPI_SLAVE */
    let va = VirtualAddress(0x5024_0000);
    let pa = PhysicalAddress(0x5024_0000);
    mapping.map_one(
        VirtualPageNumber::floor(va),
        Some(PhysicalPageNumber::floor(pa)),
        Flags::WRITABLE | Flags::READABLE,
    );

    // (0x502B_0000, 0x1000),      /* FPIOA     */
    let va = VirtualAddress(0x502B_0000);
    let pa = PhysicalAddress(0x502B_0000);
    mapping.map_one(
        VirtualPageNumber::floor(va),
        Some(PhysicalPageNumber::floor(pa)),
        Flags::WRITABLE | Flags::READABLE,
    );

    // (0x5044_0000, 0x1000),      /* SYSCTL    */
    let va = VirtualAddress(0x5044_0000);
    let pa = PhysicalAddress(0x5044_0000);
    mapping.map_one(
        VirtualPageNumber::floor(va),
        Some(PhysicalPageNumber::floor(pa)),
        Flags::WRITABLE | Flags::READABLE,
    );

    // (0x5200_0000, 0x1000),      /* SPI0      */
    let va = VirtualAddress(0x5200_0000);
    let pa = PhysicalAddress(0x5200_0000);
    mapping.map_one(
        VirtualPageNumber::floor(va),
        Some(PhysicalPageNumber::floor(pa)),
        Flags::WRITABLE | Flags::READABLE,
    );

    // (0x5300_0000, 0x1000),      /* SPI1      */
    let va = VirtualAddress(0x5300_0000);
    let pa = PhysicalAddress(0x5300_0000);
    mapping.map_one(
        VirtualPageNumber::floor(va),
        Some(PhysicalPageNumber::floor(pa)),
        Flags::WRITABLE | Flags::READABLE,
    );

    // (0x5400_0000, 0x1000),      /* SPI2      */
    let va = VirtualAddress(0x5400_0000);
    let pa = PhysicalAddress(0x5400_0000);
    mapping.map_one(
        VirtualPageNumber::floor(va),
        Some(PhysicalPageNumber::floor(pa)),
        Flags::WRITABLE | Flags::READABLE,
    );
}
