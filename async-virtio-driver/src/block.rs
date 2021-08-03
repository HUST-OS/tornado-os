use super::config::*;
use super::mmio::VirtIOHeader;
use super::queue::VirtQueue;
use super::util::AsBuf;
use super::*;
use alloc::sync::Arc;
use event::Event;
/// 虚拟块设备前端驱动
/// ref: https://github.com/rcore-os/virtio-drivers/blob/master/src/blk.rs
/// thanks!
///
/// BlockFuture 的 Send 和 Sync：
/// + inner 成员是 Send 和 Sync 的，_req_type 和 response 成员暂时不会用到，因此从成员变量看来是 Send 和 Sync 的
/// + poll 方法借助了 Mutex 实现内部可变性，在并发场景下多个 poll 操作一起运行的时候，有锁机制保证操作的原子性，因此是 Sync 的
/// 因此个人觉得 BLockFuture 是 Send 和 Sync 的
///
/// VirtioBlock 设计需求分析：
/// + 需要在并发场景下执行 async_read 或 async_write 或 ack_interrupt 操作，
/// 因此这三个方法都必须是 &self 而不能是 &mut self，因此通过 Mutex 提供内部可变性，并保证并发安全
/// + 需要想清楚哪些操作必须是原子的，必须按顺序来，否则会出问题
/// + 比如多个协程都需要执行 async_read，这时候需要往虚拟队列中添加描述符，然后通知设备，
/// 如果添加描述符和通知设备两个操作不是原子的话，可能会出问题。（这里可能两个操作不应该是原子的，只是举个例子，说明系统里面可能会有这样的情况）
///
/// todo: 弄清楚哪些操作需要同步，哪些部分需要加锁
use bitflags::bitflags;
use core::cell::RefCell;
use core::future::Future;
use core::pin::Pin;
use core::ptr::NonNull;
use core::task::{Context, Poll};
use spin::Mutex;
use volatile::Volatile;

pub struct BlockFuture {
    /// 该块设备的内部结构，用于 poll 操作的时候判断请求是否完成
    /// 如果完成了也会对这里的值做相关处理
    inner: Arc<Mutex<VirtIOBlockInner>>,
    /// IO 请求的描述符链头部
    head: u16,
    /// IO 请求缓冲区
    req: NonNull<BlockReq>,
    /// IO 回应缓冲区
    resp: NonNull<BlockResp>,
    /// 是否是第一次 poll
    first_poll: RefCell<bool>,
}

impl Future for BlockFuture {
    type Output = Result<()>;
    // warn: 这里需要仔细考虑操作的原子性
    // 这里可能有外部中断进入
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut inner = self.inner.lock();
        let (h, q) = inner.header_and_queue_mut();
        unsafe {
            // 如果是第一次 Poll，则通知设备，直接返回 Pending
            if *(self.first_poll.as_ptr()) {
                h.notify(0);
                *(self.first_poll.borrow_mut()) = false;
                return Poll::Pending;
            }
        }
        match q.can_pop() {
            true => {
                let pop_ret = q.pop_used()?;
                assert_eq!(self.head, pop_ret.0);
                unsafe {
                    let resp = *self.resp.as_ptr();
                    match resp.status {
                        BlockRespStatus::Ok => {
                            if h.ack_interrupt() {
                                return Poll::Ready(Ok(()));
                            } else {
                                return Poll::Ready(Err(VirtIOError::AckInterruptError));
                            }
                        }
                        _ => return Poll::Ready(Err(VirtIOError::DeciveResponseError)),
                    }
                }
            }
            false => {
                // 这里不进行唤醒，直接返回 pending
                // 外部中断到来的时候在内核里面唤醒
                Poll::Pending
            }
        }
    }
}

unsafe impl Send for BlockFuture {}
unsafe impl Sync for BlockFuture {}

/// 虚拟块设备
/// 常量泛型参数：一个块中的扇区数
///
/// 扇区 vs 块：
/// * 扇区是存储介质的最小存储单位，是物理上的概念
/// * 块是文件系统的最小存储单位，是逻辑上的概念
pub struct VirtIOBlock<const N: usize> {
    /// 块设备的内部内容
    lock_inner: Arc<Mutex<VirtIOBlockInner>>,
    /// 不上锁的 inner，只读，用于中断处理的时候读取相应的状态
    ///
    /// todo: 不要通过 NonNull 所有权和生命周期机制，采用更加 Rust 的写法
    unlock_queue: NonNull<VirtQueue>,
    /// 容量
    capacity: usize,
    /// 扇区大小
    pub sector_size: u32,
    pub wake_ops: Event
}

// todo: 尽量让 VirtIOBlock 天然 Send 和 Sync
unsafe impl<const N: usize> Send for VirtIOBlock<N> {}
unsafe impl<const N: usize> Sync for VirtIOBlock<N> {}

/// 并发场景中经常需要 VirtIOHeader 和 VirtQueue 共同完成一些原子操作
/// 因此把这两者放到一个结构体里面
pub struct VirtIOBlockInner {
    /// MMIO 头部
    pub header: &'static mut VirtIOHeader,
    /// 虚拟队列
    pub queue: VirtQueue,
    /// IO 请求池
    pub req_pool: [BlockReq; VIRT_QUEUE_SIZE],
    /// IO 回应池
    pub resp_pool: [BlockResp; VIRT_QUEUE_SIZE],
}

impl VirtIOBlockInner {
    pub fn header_and_queue(&self) -> (&VirtIOHeader, &VirtQueue) {
        (self.header, &self.queue)
    }

    pub fn header_and_queue_mut(&mut self) -> (&mut VirtIOHeader, &mut VirtQueue) {
        (&mut self.header, &mut self.queue)
    }

    pub fn header_queue_req_resp(&self) -> (&VirtIOHeader, &VirtQueue, &[BlockReq], &[BlockResp]) {
        (self.header, &self.queue, &self.req_pool, &self.resp_pool)
    }

    pub fn header_queue_req_resp_mut(
        &mut self,
    ) -> (
        &mut VirtIOHeader,
        &mut VirtQueue,
        &mut [BlockReq],
        &mut [BlockResp],
    ) {
        (
            self.header,
            &mut self.queue,
            &mut self.req_pool,
            &mut self.resp_pool,
        )
    }
}

impl<const N: usize> VirtIOBlock<N> {
    /// 以异步方式创建虚拟块设备驱动
    pub async fn async_new(header: &'static mut VirtIOHeader) -> Result<VirtIOBlock<N>> {
        if !header.verify() {
            return Err(VirtIOError::HeaderVerifyError);
        }
        header.begin_init(|f| {
            let features = BlockFeature::from_bits_truncate(f);
            println!("[virtio] block device features: {:?}", features);
            // 对这些 features 进行谈判
            let supported_featuers = BlockFeature::empty();
            (features & supported_featuers).bits()
        });

        // 读取配置空间
        let config = unsafe { &mut *(header.config_space() as *mut BlockConfig) };
        println!("[virtio] config: {:?}", config);
        println!(
            "[virtio] found a block device of size {} KB",
            config.capacity.read() / 2
        );

        let queue = VirtQueue::async_new(header, 0, VIRT_QUEUE_SIZE as u16).await?;

        header.finish_init();

        let req_pool = [BlockReq {
            type_: BlockReqType::Discard,
            reserved: 0,
            sector: 0,
        }; VIRT_QUEUE_SIZE];
        let resp_pool = [BlockResp::default(); VIRT_QUEUE_SIZE];

        let inner = VirtIOBlockInner {
            header,
            queue,
            req_pool,
            resp_pool,
        };
        let lock_inner = Arc::new(Mutex::new(inner));
        let queue_ptr = lock_inner.lock().header_and_queue().1 as *const _ as *mut VirtQueue;

        Ok(VirtIOBlock {
            lock_inner,
            unlock_queue: NonNull::new(queue_ptr).unwrap(),
            capacity: config.capacity.read() as usize,
            sector_size: config.sector_size.read(),
            wake_ops: Event::new()
        })
    }

    pub fn new(header: &'static mut VirtIOHeader) -> Result<Self> {
        if !header.verify() {
            return Err(VirtIOError::HeaderVerifyError);
        }
        header.begin_init(|f| {
            let features = BlockFeature::from_bits_truncate(f);
            println!("[virtio] block device features: {:?}", features);
            // 对这些 features 进行谈判
            let supported_featuers = BlockFeature::empty();
            (features & supported_featuers).bits()
        });

        // 读取配置空间
        let config = unsafe { &mut *(header.config_space() as *mut BlockConfig) };
        println!("[virtio] config: {:?}", config);
        println!(
            "[virtio] found a block device of size {} KB",
            config.capacity.read() / 2
        );

        let queue = VirtQueue::new(header, 0, VIRT_QUEUE_SIZE as u16)?;

        header.finish_init();

        let req_pool = [BlockReq {
            type_: BlockReqType::Discard,
            reserved: 0,
            sector: 0,
        }; VIRT_QUEUE_SIZE];
        let resp_pool = [BlockResp::default(); VIRT_QUEUE_SIZE];

        let inner = VirtIOBlockInner {
            header,
            queue,
            req_pool,
            resp_pool,
        };
        let lock_inner = Arc::new(Mutex::new(inner));
        let queue_ptr = lock_inner.lock().header_and_queue().1 as *const _ as *mut VirtQueue;
        Ok(VirtIOBlock {
            lock_inner,
            unlock_queue: NonNull::new(queue_ptr).unwrap(),
            capacity: config.capacity.read() as usize,
            sector_size: config.sector_size.read(),
            wake_ops: Event::new()
        })
    }

    /// 通知设备 virtio 外部中断已经处理完成
    pub fn ack_interrupt(&self) -> bool {
        self.lock_inner.lock().header.ack_interrupt()
    }

    /// 以异步方式读取一个扇区
    /// todo: 仔细考虑这里的操作原子性
    pub fn async_read_sector(&self, sector_id: usize, buf: &mut [u8]) -> BlockFuture {
        // 缓冲区大小必须等于扇区大小
        if buf.len() != self.sector_size as usize {
            panic!(
                "[virtio] buffer size must equal to sector size - {}!",
                self.sector_size
            );
        }
        let mut inner = self.lock_inner.lock();
        let (_h, q, reqs, resps) = inner.header_queue_req_resp_mut();

        // 空闲描述符表的头部
        let free_head = q.free_head();

        // IO 请求
        let req = &mut reqs[free_head as usize];
        req.type_ = BlockReqType::In;
        req.reserved = 0;
        req.sector = sector_id as u64;

        // IO 回应
        let resp = &mut resps[free_head as usize];

        let head = q
            .add_buf(&[req.as_buf()], &[buf, resp.as_buf_mut()])
            .expect("[virtio] virtual queue add buf error");

        let req_ptr = req.as_buf() as *const _ as *mut BlockReq;
        let resp_ptr = resp.as_buf() as *const _ as *mut BlockResp;

        // 不在这里通知设备，在 BlockFuture 第一次 poll 的时候通知
        // h.notify(0);
        BlockFuture {
            inner: Arc::clone(&self.lock_inner),
            head,
            req: NonNull::new(req_ptr).unwrap(),
            resp: NonNull::new(resp_ptr).unwrap(),
            first_poll: RefCell::new(true),
        }
    }

    /// 以异步方式写入一个扇区
    /// todo: 仔细考虑这里的操作原子性
    pub fn async_write_sector(&self, sector_id: usize, buf: &[u8]) -> BlockFuture {
        if buf.len() != self.sector_size as usize {
            panic!(
                "[virtio] buffer size must equal to sector size - {}!",
                self.sector_size
            );
        }

        let mut inner = self.lock_inner.lock();
        let (_h, q, reqs, resps) = inner.header_queue_req_resp_mut();

        // 空闲描述符表头部
        let free_head = q.free_head();

        // IO 请求
        let req = &mut reqs[free_head as usize];
        req.type_ = BlockReqType::Out;
        req.reserved = 0;
        req.sector = sector_id as u64;

        // IO 回应
        let resp = &mut resps[free_head as usize];

        let head = q
            .add_buf(&[req.as_buf(), buf], &[resp.as_buf_mut()])
            .expect("[virtio] virtual queue add buf error");

        let req_ptr = req.as_buf() as *const _ as *mut BlockReq;
        let resp_ptr = resp.as_buf() as *const _ as *mut BlockResp;

        // 不在这里通知设备，在 BlockFuture 第一次 poll 的时候通知
        // h.notify(0);
        BlockFuture {
            inner: Arc::clone(&self.lock_inner),
            head,
            req: NonNull::new(req_ptr).unwrap(),
            resp: NonNull::new(resp_ptr).unwrap(),
            first_poll: RefCell::new(true),
        }
    }

    /// 异步方式读取一个块
    pub async fn async_read_block(&self, block_id: usize, buf: &mut [u8]) -> Result<()> {
        // 块大小 = 一个块中的扇区数 * 扇区大小
        let block_size = self.sector_size as usize * N;
        if buf.len() != block_size {
            panic!(
                "[virtio] buffer size must equal to block size - {}!",
                block_size
            );
        }
        for (idx, b) in buf.chunks_mut(self.sector_size as usize).enumerate() {
            self.async_read_sector(block_id + idx, b).await?;
        }
        Ok(())
    }

    /// 异步方式写入一个块
    pub async fn async_write_block(&self, block_id: usize, buf: &[u8]) -> Result<()> {
        // 块大小 = 一个块中的扇区数 * 扇区大小
        let block_size = self.sector_size as usize * N;
        if buf.len() != block_size {
            panic!(
                "[virtio] buffer size must equal to block size - {}!",
                block_size
            );
        }
        for (idx, b) in buf.chunks(self.sector_size as usize).enumerate() {
            self.async_write_sector(block_id + idx, b).await?;
        }
        Ok(())
    }

    pub async fn read_sector_event(&self, sector_id: usize, buf: &mut [u8]) -> Result<()> {
        // 开始监听
        let listener = self.wake_ops.listen();
        // 缓冲区大小必须等于扇区大小
        if buf.len() != self.sector_size as usize {
            panic!(
                "[virtio] buffer size must equal to sector size - {}!",
                self.sector_size
            );
        }
        let req = BlockReq {
            type_: BlockReqType::In,
            reserved: 0,
            sector: sector_id as u64,
        };
        let mut inner = self.lock_inner.lock();
        let mut resp = BlockResp::default();

        let (h, q) = inner.header_and_queue_mut();

        q.add_buf(&[req.as_buf()], &[buf, resp.as_buf_mut()])
            .expect("[virtio] virtual queue add buf error");

        h.notify(0);

        listener.await;
        
        q.pop_used()?;
        match resp.status {
            BlockRespStatus::Ok => Ok(()),
            _ => Err(VirtIOError::IOError),
        }
    }

    pub async fn write_serctor_event(&self, sector_id: usize, buf: &[u8]) -> Result<()> {
        // 开始监听
        let listener = self.wake_ops.listen();
        // 缓冲区大小必须等于扇区大小
        if buf.len() != self.sector_size as usize {
            panic!(
                "[virtio] buffer size must equal to sector size - {}!",
                self.sector_size
            );
        }
        let req = BlockReq {
            type_: BlockReqType::Out,
            reserved: 0,
            sector: sector_id as u64,
        };
        let mut inner = self.lock_inner.lock();
        let mut resp = BlockResp::default();

        let (h, q) = inner.header_and_queue_mut();

        q.add_buf(&[req.as_buf(), buf], &[resp.as_buf_mut()])
            .expect("[virtio] virtual queue add buf error");

        h.notify(0);

        listener.await;
        
        q.pop_used()?;
        match resp.status {
            BlockRespStatus::Ok => Ok(()),
            _ => Err(VirtIOError::IOError),
        }
    }

    pub async fn read_block_event(&self, block_id: usize, buf: &mut [u8]) -> Result<()> {
        // 块大小 = 一个块中的扇区数 * 扇区大小
        let block_size = self.sector_size as usize * N;
        if buf.len() != block_size {
            panic!(
                "[virtio] buffer size {} not equal to block size - {}!",
                buf.len(),
                block_size
            );
        }
        for (idx, b) in buf.chunks_mut(self.sector_size as usize).enumerate() {
            self.read_sector_event(block_id + idx, b).await?;
        }
        Ok(())
    }
    
    pub async fn write_block_event(&self, block_id: usize, buf: &[u8]) -> Result<()> {
        // 块大小 = 一个块中的扇区数 * 扇区大小
        let block_size = self.sector_size as usize * N;
        if buf.len() != block_size {
            panic!(
                "[virtio] buffer size must equal to block size - {}!",
                block_size
            );
        }
        for (idx, b) in buf.chunks(self.sector_size as usize).enumerate() {
            self.write_serctor_event(block_id + idx, b).await?;
        }
        Ok(())
    }
    
    /// unused
    pub fn read_sector(&self, block_id: usize, buf: &mut [u8]) -> Result<()> {
        // 缓冲区大小必须等于扇区大小
        if buf.len() != self.sector_size as usize {
            panic!(
                "[virtio] buffer size must equal to sector size - {}!",
                self.sector_size
            );
        }
        let req = BlockReq {
            type_: BlockReqType::In,
            reserved: 0,
            sector: block_id as u64,
        };
        let mut inner = self.lock_inner.lock();
        let mut resp = BlockResp::default();

        let (h, q) = inner.header_and_queue_mut();

        q.add_buf(&[req.as_buf()], &[buf, resp.as_buf_mut()])
            .expect("[virtio] virtual queue add buf error");

        h.notify(0);

        while !q.can_pop() {}
        q.pop_used()?;
        match resp.status {
            BlockRespStatus::Ok => Ok(()),
            _ => Err(VirtIOError::IOError),
        }
    }

    /// unused
    pub fn write_sector(&self, block_id: usize, buf: &[u8]) -> Result<()> {
        // 缓冲区大小必须等于扇区大小
        if buf.len() != self.sector_size as usize {
            panic!(
                "[virtio] buffer size must equal to sector size - {}!",
                self.sector_size
            );
        }
        let req = BlockReq {
            type_: BlockReqType::Out,
            reserved: 0,
            sector: block_id as u64,
        };
        let mut inner = self.lock_inner.lock();
        let mut resp = BlockResp::default();

        let (h, q) = inner.header_and_queue_mut();

        q.add_buf(&[req.as_buf(), buf], &[resp.as_buf_mut()])
            .expect("[virtio] virtual queue add buf error");

        h.notify(0);

        while !q.can_pop() {}
        q.pop_used()?;
        match resp.status {
            BlockRespStatus::Ok => Ok(()),
            _ => Err(VirtIOError::IOError),
        }
    }

    /// 处理 virtio 外部中断
    /// todo: 仔细考虑这里的操作原子性
    pub unsafe fn handle_interrupt(&self) -> Result<InterruptRet> {
        // 这里使用获取不加锁的 inner
        let q = self.unlock_queue.as_ref();
        if !q.can_pop() {
            return Err(VirtIOError::IOError);
        }
        let (idx, _len) = q.next_used()?;
        let desc = q.descriptor(idx as usize);
        let req_va = virtio_phys_to_virt(desc.paddr.read() as usize);
        let req = &*(req_va as *const BlockReq);
        let ret = match req.type_ {
            BlockReqType::In => InterruptRet::Read(req.sector),
            BlockReqType::Out => InterruptRet::Write(req.sector),
            _ => InterruptRet::Other,
        };
        Ok(ret)
    }
}

bitflags! {
    struct BlockFeature: u64 {
        /// Device supports request barriers. (legacy)
        const BARRIER       = 1 << 0;
        /// Maximum size of any single segment is in `size_max`.
        const SIZE_MAX      = 1 << 1;
        /// Maximum number of segments in a request is in `seg_max`.
        const SEG_MAX       = 1 << 2;
        /// Disk-style geometry specified in geometry.
        const GEOMETRY      = 1 << 4;
        /// Device is read-only.
        const RO            = 1 << 5;
        /// Block size of disk is in `blk_size`.
        const BLK_SIZE      = 1 << 6;
        /// Device supports scsi packet commands. (legacy)
        const SCSI          = 1 << 7;
        /// Cache flush command support.
        const FLUSH         = 1 << 9;
        /// Device exports information on optimal I/O alignment.
        const TOPOLOGY      = 1 << 10;
        /// Device can toggle its cache between writeback and writethrough modes.
        const CONFIG_WCE    = 1 << 11;
        /// Device can support discard command, maximum discard sectors size in
        /// `max_discard_sectors` and maximum discard segment number in
        /// `max_discard_seg`.
        const DISCARD       = 1 << 13;
        /// Device can support write zeroes command, maximum write zeroes sectors
        /// size in `max_write_zeroes_sectors` and maximum write zeroes segment
        /// number in `max_write_zeroes_seg`.
        const WRITE_ZEROES  = 1 << 14;

        // device independent
        const NOTIFY_ON_EMPTY       = 1 << 24; // legacy
        const ANY_LAYOUT            = 1 << 27; // legacy
        const RING_INDIRECT_DESC    = 1 << 28;
        const RING_EVENT_IDX        = 1 << 29;
        const UNUSED                = 1 << 30; // legacy
        const VERSION_1             = 1 << 32; // detect legacy

        // the following since virtio v1.1
        const ACCESS_PLATFORM       = 1 << 33;
        const RING_PACKED           = 1 << 34;
        const IN_ORDER              = 1 << 35;
        const ORDER_PLATFORM        = 1 << 36;
        const SR_IOV                = 1 << 37;
        const NOTIFICATION_DATA     = 1 << 38;
    }
}

/// 块设备配置
#[repr(C)]
#[derive(Debug)]
struct BlockConfig {
    /// 扇区数目
    capacity: Volatile<u64>,
    size_max: Volatile<u32>,
    seg_max: Volatile<u32>,
    cylinders: Volatile<u16>,
    heads: Volatile<u8>,
    sectors: Volatile<u8>,
    /// 扇区大小
    sector_size: Volatile<u32>,
    physical_block_exp: Volatile<u8>,
    alignment_offset: Volatile<u8>,
    min_io_size: Volatile<u16>,
    opt_io_size: Volatile<u32>,
    // ... ignored
}

/// 块设备请求
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BlockReq {
    type_: BlockReqType,
    reserved: u32,
    sector: u64,
}

/// 块设备回应
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BlockResp {
    status: BlockRespStatus,
}

/// 块设备请求类型
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
enum BlockReqType {
    In = 0,
    Out = 1,
    Flush = 4,
    Discard = 11,
    WriteZeroes = 13,
}

/// 块设备回应状态
#[repr(u8)]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum BlockRespStatus {
    Ok = 0,
    IoErr = 1,
    Unsupported = 2,
    NotReady = 3,
}

impl Default for BlockResp {
    fn default() -> Self {
        BlockResp {
            status: BlockRespStatus::NotReady,
        }
    }
}

unsafe impl AsBuf for BlockReq {}
unsafe impl AsBuf for BlockResp {}

#[derive(Debug)]
/// 中断响应返回值
pub enum InterruptRet {
    /// 读请求完成的块
    Read(u64),
    /// 写请求完成的块
    Write(u64),
    /// 其他
    Other,
}

extern "C" {
    /// 内核提供的物理地址到虚拟地址的转换函数
    fn virtio_phys_to_virt(paddr: usize) -> usize;
}
