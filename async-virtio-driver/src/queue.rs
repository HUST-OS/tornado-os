use super::config::*;
use super::dma::DMA;
use super::mmio::VirtIOHeader;
use super::*;
use crate::util::align_up_page;
use alloc::vec::Vec;
use bitflags::bitflags;
use core::ptr::NonNull;
use core::{
    mem::size_of,
    sync::atomic::{fence, Ordering},
};
/// 虚拟队列相关实现
/// ref: https://github.com/rcore-os/virtio-drivers/blob/master/src/queue.rs
/// thanks!
use volatile::Volatile;

type AvailableRing = Ring<Volatile<u16>>;
type UsedRing = Ring<UsedElement>;

/// Virtio 中的虚拟队列接口，前后端通信的桥梁
///
#[repr(C)]
pub struct VirtQueue {
    /// DMA 空间
    dma: DMA,
    /// 描述符表
    descriptor_table: NonNull<[Descriptor]>,
    /// 可用环
    avail_ring: NonNull<AvailableRing>,
    /// 已用环
    used_ring: NonNull<UsedRing>,
    /// 虚拟队列索引值
    /// 一个虚拟设备实现可能有多个虚拟队列
    queue_index: u32,
    /// 虚拟队列长度
    /// 等于描述符表中条目的数量
    queue_size: u16,
    /// 已经使用的描述符数目
    used_num: u16,
    /// 空闲描述符链表头
    /// 初始时所有描述符通过 next 指针依次相连形成空闲链表
    free_desc_head: u16,
    /// 可用环的索引值
    avail_index: u16,
    /// 设备上次已取的已用环元素的位置
    last_used_index: u16,
}

// 跨上下文递交所有权，结构体意义不变，可以 Send
unsafe impl Send for VirtQueue {}
// 可以跨上下文递交引用，也可以 Sync
unsafe impl Sync for VirtQueue {}

impl VirtQueue {
    pub fn new(header: &mut VirtIOHeader, index: usize, size: u16) -> Result<Self> {
        if header.queue_used(index as u32) {
            return Err(VirtIOError::QueueInUsed(index));
        }
        if !size.is_power_of_two() || header.max_queue_size() < size as u32 {
            return Err(VirtIOError::InvalidParameter);
        }
        let queue_layout = VirtQueueMemLayout::new(size);
        let dma = DMA::new(queue_layout.mem_size / PAGE_SIZE)?;
        println!("[virtio] DMA address: {:#x}", dma.start_physical_address());

        // 在 MMIO 接口中设置虚拟队列的相关信息
        header.queue_set(
            index as u32,
            size as u32,
            PAGE_SIZE as u32,
            dma.ppn() as u32,
        );

        // 描述符表起始地址
        let desc_table = unsafe {
            core::slice::from_raw_parts_mut(
                dma.start_virtual_address() as *mut Descriptor,
                size as usize,
            )
        };

        // 将空描述符连成链表
        for i in 0..(size - 1) {
            desc_table[i as usize].next.write(i + 1);
        }

        let descriptor_table = NonNull::new(desc_table).unwrap();
        let avail_ring = NonNull::new(unsafe {
            &mut *((dma.start_virtual_address() + queue_layout.avail_ring_offset)
                as *mut AvailableRing)
        })
        .unwrap();
        let used_ring = NonNull::new(unsafe {
            &mut *((dma.start_virtual_address() + queue_layout.used_ring_offset) as *mut UsedRing)
        })
        .unwrap();

        Ok(VirtQueue {
            dma,
            descriptor_table,
            avail_ring,
            used_ring,
            queue_size: size,
            queue_index: index as u32,
            used_num: 0,
            free_desc_head: 0,
            avail_index: 0,
            last_used_index: 0,
        })
    }

    pub async fn async_new(
        header: &mut VirtIOHeader,
        index: usize,
        size: u16,
    ) -> Result<VirtQueue> {
        if header.queue_used(index as u32) {
            return Err(VirtIOError::QueueInUsed(index));
        }
        if !size.is_power_of_two() || header.max_queue_size() < size as u32 {
            return Err(VirtIOError::InvalidParameter);
        }
        let queue_layout = VirtQueueMemLayout::new(size);
        let dma = DMA::alloc(queue_layout.mem_size / PAGE_SIZE).await;
        println!("[virtio] DMA address: {:#x}", dma.start_physical_address());

        // 在 MMIO 接口中设置虚拟队列的相关信息
        header.queue_set(
            index as u32,
            size as u32,
            PAGE_SIZE as u32,
            dma.ppn() as u32,
        );

        // 描述符表起始地址
        let desc_table = unsafe {
            core::slice::from_raw_parts_mut(
                dma.start_virtual_address() as *mut Descriptor,
                size as usize,
            )
        };

        // 将空描述符连成链表
        for i in 0..(size - 1) {
            desc_table[i as usize].next.write(i + 1);
        }

        let descriptor_table = NonNull::new(desc_table).unwrap();
        let avail_ring = NonNull::new(unsafe {
            &mut *((dma.start_virtual_address() + queue_layout.avail_ring_offset)
                as *mut AvailableRing)
        })
        .unwrap();
        let used_ring = NonNull::new(unsafe {
            &mut *((dma.start_virtual_address() + queue_layout.used_ring_offset) as *mut UsedRing)
        })
        .unwrap();

        Ok(VirtQueue {
            dma,
            descriptor_table,
            avail_ring,
            used_ring,
            queue_size: size,
            queue_index: index as u32,
            used_num: 0,
            free_desc_head: 0,
            avail_index: 0,
            last_used_index: 0,
        })
    }

    /// 添加 buffers 到虚拟队列，返回一个 token
    pub fn add_buf(&mut self, inputs: &[&[u8]], outputs: &[&mut [u8]]) -> Result<u16> {
        if inputs.is_empty() && outputs.is_empty() {
            return Err(VirtIOError::InvalidParameter);
        }
        if inputs.len() + outputs.len() + self.used_num as usize > self.queue_size as usize {
            // buffer 数量溢出
            return Err(VirtIOError::Overflow);
        }

        // 从空闲描述符表中分配描述符
        let head = self.free_desc_head;
        let mut tail = self.free_desc_head;
        let mut next_free_desc_head = self.free_desc_head;
        let descriptor_table = unsafe { self.descriptor_table.as_mut() };
        // 将输入缓冲区的信息写入描述符表
        inputs.iter().for_each(|input| {
            let desc = &mut descriptor_table[next_free_desc_head as usize];
            // 将 buffer 的信息写入描述符
            desc.set_buf(input);
            // 设置描述符的标识位
            desc.flags.write(DescriptorFlags::NEXT);
            tail = next_free_desc_head;
            next_free_desc_head = desc.next.read();
        });
        // 更新空闲描述符表头部
        self.free_desc_head = next_free_desc_head;
        // 将输出缓冲区的信息写入描述符表
        outputs.iter().for_each(|output| {
            let desc = &mut descriptor_table[next_free_desc_head as usize];
            desc.set_buf(output);
            desc.flags
                .write(DescriptorFlags::NEXT | DescriptorFlags::WRITE);
            tail = next_free_desc_head;
            next_free_desc_head = desc.next.read();
        });
        // 更新空闲描述符表头部
        self.free_desc_head = next_free_desc_head;
        // 清除描述符链的最后一个元素的 next 指针
        {
            let desc = &mut descriptor_table[tail as usize];
            let mut flags = desc.flags.read();
            flags.remove(DescriptorFlags::NEXT);
            desc.flags.write(flags);
        }
        // 更新已使用描述符数目
        self.used_num += (inputs.len() + outputs.len()) as u16;

        // 将描述符链的头部放入可用环中
        let avail_ring = unsafe { self.avail_ring.as_mut() };
        let avail_slot = self.avail_index & (self.queue_size - 1);
        avail_ring.ring[avail_slot as usize].write(head);

        // write barrier(内存屏障操作？)
        fence(Ordering::SeqCst);

        // 更新可用环的头部
        self.avail_index = self.avail_index.wrapping_add(1);
        avail_ring.idx.write(self.avail_index);
        Ok(head)
    }

    /// 是否可以从可用环中弹出没处理的项
    pub fn can_pop(&self) -> bool {
        let used_ring = unsafe { self.used_ring.as_ref() };
        self.last_used_index != used_ring.idx.read()
    }

    /// 可用的空闲描述符数量
    pub fn free_desc_num(&self) -> usize {
        (self.queue_size - self.used_num) as usize
    }

    /// 回收描述符
    /// 该方法将会把需要回收的描述符链放到空闲描述符链的头部
    fn recycle_descriptors(&mut self, mut head: u16) {
        let origin_desc_head = self.free_desc_head;
        self.free_desc_head = head;
        let descriptor_table = unsafe { self.descriptor_table.as_mut() };
        loop {
            let desc = &mut descriptor_table[head as usize];
            let flags = desc.flags.read();
            if flags.contains(DescriptorFlags::NEXT) {
                head = desc.next.read();
                self.used_num -= 1;
            } else {
                desc.next.write(origin_desc_head);
                self.used_num -= 1;
                return;
            }
        }
    }

    /// 从已用环中弹出一个 token，并返回长度
    /// ref: linux virtio_ring.c virtqueue_get_buf_ctx
    pub fn pop_used(&mut self) -> Result<(u16, u32)> {
        if !self.can_pop() {
            return Err(VirtIOError::UsedRingNotReady);
        }
        // read barrier
        fence(Ordering::SeqCst);

        let used_ring = unsafe { self.used_ring.as_mut() };
        let last_used_slot = self.last_used_index & (self.queue_size - 1);
        let index = used_ring.ring[last_used_slot as usize].id.read() as u16;
        let len = used_ring.ring[last_used_slot as usize].len.read();

        self.recycle_descriptors(index);
        self.last_used_index = self.last_used_index.wrapping_add(1);

        Ok((index, len))
    }

    /// 从已用环中取出下一个 token，但不弹出
    pub fn next_used(&self) -> Result<(u16, u32)> {
        if !self.can_pop() {
            return Err(VirtIOError::UsedRingNotReady);
        }

        // read barrier
        fence(Ordering::SeqCst);

        let used_ring = unsafe { self.used_ring.as_ref() };
        let last_used_slot = self.last_used_index & (self.queue_size - 1);
        let index = used_ring.ring[last_used_slot as usize].id.read() as u16;
        let len = used_ring.ring[last_used_slot as usize].len.read();

        Ok((index, len))
    }

    pub fn free_head(&self) -> u16 {
        self.free_desc_head
    }

    pub fn descriptor(&self, index: usize) -> Descriptor {
        unsafe { self.descriptor_table.as_ref()[index].clone() }
    }

    pub fn desc_table(&self) -> &[Descriptor] {
        unsafe { self.descriptor_table.as_ref() }
    }

    pub fn desc_table_mut(&mut self) -> &mut [Descriptor] {
        unsafe { self.descriptor_table.as_mut() }
    }

    pub fn avail_ring(&self) -> &AvailableRing {
        unsafe { self.avail_ring.as_ref() }
    }

    pub fn avail_ring_mut(&mut self) -> &mut AvailableRing {
        unsafe { self.avail_ring.as_mut() }
    }

    pub fn used_ring(&self) -> &UsedRing {
        unsafe { self.used_ring.as_ref() }
    }

    pub fn used_ring_mut(&mut self) -> &mut UsedRing {
        unsafe { self.used_ring.as_mut() }
    }

    /// 返回给定头部的描述符链
    pub fn descriptor_link(&self, head: u16) -> Vec<&Descriptor> {
        let desc_table = self.desc_table();
        let mut ret = Vec::new();
        let mut pos = head;
        while desc_table[pos as usize]
            .flags
            .read()
            .contains(DescriptorFlags::NEXT)
        {
            let desc = &desc_table[pos as usize];
            ret.push(desc);
            pos = desc.next.read();
        }
        // last one
        ret.push(&desc_table[pos as usize]);
        ret
    }
}

/// 虚拟队列内存布局信息
struct VirtQueueMemLayout {
    /// 可用环地址偏移
    avail_ring_offset: usize,
    /// 已用环地址偏移
    used_ring_offset: usize,
    /// 总大小
    mem_size: usize,
}

impl VirtQueueMemLayout {
    fn new(queue_size: u16) -> Self {
        assert!(
            queue_size.is_power_of_two(),
            "[virtio] queue size must be a power off 2"
        );
        let q_size = queue_size as usize;
        let descriptors_size = size_of::<Descriptor>() * q_size;
        let avail_ring_size = size_of::<u16>() * (3 + q_size);
        let used_ring_size = size_of::<u16>() * 3 + size_of::<UsedElement>() * q_size;
        VirtQueueMemLayout {
            avail_ring_offset: descriptors_size,
            used_ring_offset: align_up_page(descriptors_size + avail_ring_size),
            mem_size: align_up_page(descriptors_size + avail_ring_size)
                + align_up_page(used_ring_size),
        }
    }
}

/// 描述符
#[repr(C, align(16))]
#[derive(Debug)]
pub struct Descriptor {
    /// buffer 的物理地址
    pub paddr: Volatile<u64>,
    /// buffer 的长度
    len: Volatile<u32>,
    /// 标识
    flags: Volatile<DescriptorFlags>,
    /// 下一个描述符的指针
    next: Volatile<u16>,
}

impl Clone for Descriptor {
    fn clone(&self) -> Self {
        Self {
            paddr: Volatile::<u64>::new(self.paddr.read()),
            len: Volatile::<u32>::new(self.len.read()),
            flags: Volatile::<DescriptorFlags>::new(self.flags.read()),
            next: Volatile::<u16>::new(self.next.read()),
        }
    }
}

impl Descriptor {
    /// 把特定 buffer 的信息写入到描述符
    fn set_buf(&mut self, buf: &[u8]) {
        let buf_paddr = unsafe { virtio_virt_to_phys(buf.as_ptr() as usize) as u64 };
        self.paddr.write(buf_paddr);
        self.len.write(buf.len() as u32);
    }
}

bitflags! {
    /// 描述符的标识
    struct DescriptorFlags: u16 {
        const NEXT = 1;
        const WRITE = 2;
        const INDIRECT = 4;
    }
}

/// 环
/// 通过泛型对可用环和已用环进行统一抽象
#[repr(C)]
#[derive(Debug)]
pub struct Ring<Entry: Sized> {
    /// 与通知机制相关
    flags: Volatile<u16>,
    idx: Volatile<u16>,
    pub ring: [Entry; VIRT_QUEUE_SIZE],
    // unused
    event: Volatile<u16>,
}

/// 已用环中的项
#[repr(C)]
#[derive(Debug)]
pub struct UsedElement {
    id: Volatile<u32>,
    len: Volatile<u32>,
}

extern "C" {
    fn virtio_virt_to_phys(vaddr: usize) -> usize;
}
