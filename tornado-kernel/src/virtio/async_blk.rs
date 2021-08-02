/// 异步虚拟块设备接口

use async_virtio_driver::mmio::VirtIOHeader;
use async_virtio_driver::block::*;
use alloc::sync::Arc;
use crate::memory::VIRTIO0;
use async_trait::async_trait;
use alloc::boxed::Box;
use async_fat32::AsyncBlockDevive;

pub struct VirtIOAsyncBlock(pub VirtIOBlock<1>);

impl VirtIOAsyncBlock {
    pub async fn async_new() -> VirtIOAsyncBlock {
        let header = unsafe {
            &mut *(VIRTIO0 as *mut VirtIOHeader)
        };
        let async_blk = VirtIOBlock::async_new(header).await.unwrap();
        Self(async_blk)
    }

    pub fn new() -> Self {
        let header = unsafe {
            &mut *(VIRTIO0 as *mut VirtIOHeader)
        };
        let blk = VirtIOBlock::new(header).unwrap();
        Self(blk)
    }

    pub async fn read_block(&self, block_id: usize, buf: &mut [u8]) {
        self.0.async_read_block(block_id, buf).await.expect("failed to read block from async_virtio_block!");
    }

    pub async fn write_block(&self, block_id: usize, buf: &[u8]) {
        self.0.async_write_block(block_id, buf).await.expect("failed to write block from async_virtio_block!");
    }

    pub async fn read_block_event(&self, block_id: usize, buf: &mut [u8]) {
        self.0.read_block_event(block_id, buf).await.expect("read block with event");
    }

    pub async fn write_block_event(&self, block_id: usize, buf: &[u8]) {
        self.0.write_block_event(block_id, buf).await.expect("write block with event");
    }

    pub unsafe fn handle_interrupt(&self) -> Option<u64> {
        let ret = self.0.handle_interrupt().expect("handle virtio interrupt error!");
        match ret {
            InterruptRet::Read(sector) => {
                println!("virtio read block {:#x} succeed", sector);
                return Some(sector);
            }
            InterruptRet::Write(sector) => {
                println!("virtio write block {:#x} succeed", sector);
                return Some(sector);
            }
            _other => {
                return None;
            }
        }
    }
}

#[async_trait]
impl AsyncBlockDevive for VirtIOAsyncBlock {
    async fn read(&self, block_id: usize, buf: &mut [u8]) {
        self.read_block(block_id, buf).await
    }
    async fn write(&self, block_id: usize, buf: &[u8]) {
        self.write_block(block_id, buf).await
    }
}