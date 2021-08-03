//! qemu virtio 前端驱动

use crate::memory::{
    frame_alloc, FrameTracker, PhysicalAddress, PhysicalPageNumber, Satp, VirtualAddress,
    VirtualPageNumber, VIRTIO0,
};
use alloc::{sync::Arc, vec::Vec};
use async_blk::VirtIOAsyncBlock;
use bit_field::BitField;
use core::ops::Add;
use lazy_static::*;
use riscv::register::satp;
use spin::Mutex;

pub mod async_blk;

lazy_static! {
    static ref QUEUE_FRAMES: Mutex<Vec<FrameTracker>> = Mutex::new(Vec::new());
    pub static ref VIRTIO_BLOCK: Arc<VirtIOAsyncBlock> = Arc::new(VirtIOAsyncBlock::new());
}

#[no_mangle]
pub extern "C" fn virtio_dma_alloc(pages: usize) -> PhysicalAddress {
    let mut ppn_base = 0;
    for i in 0..pages {
        let frame = frame_alloc().unwrap();
        if i == 0 {
            ppn_base = frame.page_number().into();
        }
        let frame_ppn: usize = frame.page_number().into();
        assert_eq!(frame_ppn, ppn_base + i);
        QUEUE_FRAMES.lock().push(frame);
    }
    PhysicalPageNumber::from(ppn_base).start_address()
}

// todo: 检查这里
#[no_mangle]
pub extern "C" fn virtio_dma_dealloc(pa: PhysicalAddress, pages: usize) -> i32 {
    let ppn = PhysicalPageNumber::floor(pa);
    let mut remove_idx = -1;
    let mut q = QUEUE_FRAMES.lock();
    for (idx, frame) in q.iter().enumerate() {
        if frame.page_number() == ppn {
            remove_idx = idx as i32;
        }
    }
    if remove_idx != -1 {
        for _ in 0..pages {
            let pop_frame = q.remove(remove_idx as usize);
            // 最终会调用 FrameTracker::drop()，在帧分配器中释放持有的帧内存
            drop(pop_frame);
        }
    } else {
        return -1;
    }
    0
}

// 这里可以直接使用线性映射的关系
#[no_mangle]
pub extern "C" fn virtio_phys_to_virt(paddr: PhysicalAddress) -> VirtualAddress {
    paddr.virtual_address_linear()
}

// 这里需要查页表
#[no_mangle]
pub extern "C" fn virtio_virt_to_phys(vaddr: VirtualAddress) -> PhysicalAddress {
    let offset = vaddr.0.get_bits(0..12); // Sv39 低 12 位是页内偏移
    let satp = Satp::new(satp::read().bits());
    let vpn = VirtualPageNumber::floor(vaddr);
    let ppn = satp
        .translate(vpn)
        .expect("virtio virtual address not map!");
    ppn.start_address().add(offset)
}

pub async fn async_virtio_blk_test() {
    let mut read_buf = [0u8; 512];
    let mut write_buf = [0u8; 512];
    for i in 0..512 {
        write_buf.iter_mut().for_each(|byte| *byte = i as u8);
        VIRTIO_BLOCK.write_block(i as usize, &write_buf).await;
        VIRTIO_BLOCK.read_block(i as usize, &mut read_buf).await;
        assert_eq!(read_buf, write_buf);
    }
    println!("async_virtio_blk_test pass");
}
