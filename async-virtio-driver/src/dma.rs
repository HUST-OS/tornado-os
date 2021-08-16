use super::config::*;
use super::*;
/// "DMA" 实现
/// ref: https://github.com/rcore-os/virtio-drivers/blob/master/src/hal.rs
/// thanks!
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

type VirtualAddress = usize;
type PhysicalAddress = usize;

#[derive(Debug)]
pub struct DMA {
    paddr: usize,
    pages: usize,
}

/// 这些函数在操作系统内核里面定义
extern "C" {
    /// 申请分配一定页数的连续的内存，返回起始物理地址
    fn virtio_dma_alloc(pages: usize) -> PhysicalAddress;
    /// 回收一定页数的内存
    fn virtio_dma_dealloc(paddr: PhysicalAddress, pages: usize) -> i32;
    /// 内核提供的物理地址到虚拟地址的转换函数
    fn virtio_phys_to_virt(paddr: PhysicalAddress) -> VirtualAddress;
}

pub struct DMAAllocFuture {
    pages: u32,
    count: u32,
}

impl Future for DMAAllocFuture {
    type Output = DMA;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.count > MAX_DMA_ALLOC_COUNT {
            panic!("[virtio] no memory space avaiable for dma");
        }
        let paddr = unsafe { virtio_dma_alloc(self.pages as usize) };
        match paddr {
            0 => {
                self.count += 1;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            pa => Poll::Ready(DMA {
                paddr: pa,
                pages: self.pages as usize,
            }),
        }
    }
}

unsafe impl Send for DMAAllocFuture {}
unsafe impl Sync for DMAAllocFuture {}

impl DMA {
    // 先用这种方式创建 DMA，后面考虑使用 alloc 方法
    pub fn new(pages: usize) -> Result<Self> {
        let paddr = unsafe { virtio_dma_alloc(pages) };
        if paddr == 0 {
            return Err(VirtIOError::DMAAllocError);
        }
        Ok(DMA { paddr, pages })
    }

    /// 向操作系统内核申请分配 DMA 空间
    pub fn alloc(pages: usize) -> DMAAllocFuture {
        DMAAllocFuture {
            pages: pages as u32,
            count: 0,
        }
    }

    /// 起始物理地址
    pub fn start_physical_address(&self) -> usize {
        self.paddr as usize
    }

    /// 起始虚拟地址
    pub fn start_virtual_address(&self) -> usize {
        unsafe { virtio_phys_to_virt(self.paddr as usize) }
    }

    /// 物理页号
    pub fn ppn(&self) -> usize {
        self.paddr >> 12
    }

    /// 转换成 buffer
    #[allow(unused)]
    pub unsafe fn as_buf(&self) -> &'static mut [u8] {
        core::slice::from_raw_parts_mut(
            self.start_virtual_address() as _,
            PAGE_SIZE * self.pages as usize,
        )
    }
}

/// DMA 在生命周期结束的时候需要在内核里面回收相应的内存空间
impl Drop for DMA {
    fn drop(&mut self) {
        let err = unsafe { virtio_dma_dealloc(self.paddr, self.pages) };
        assert_eq!(err, 0, "[virtio] failed to deallocate DMA");
    }
}
