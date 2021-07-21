//! 块缓冲层实现
use super::cache::{Cache, LFUCache, Node};
use super::config::*;
use crate::AsyncBlockDevive;
use alloc::sync::Arc;
use core::mem::MaybeUninit;
use spin::Mutex;

// B: 一个块中的字节数
// N: 块缓冲层的块数
pub struct AsyncBlockCache<
    C: Cache<N, Key = usize, Value = [u8; B]>,
    const B: usize,
    const N: usize,
> {
    block_device: Arc<dyn AsyncBlockDevive + Send + Sync>,
    cache: Mutex<C>,
}

impl AsyncBlockCache<LFUCache<usize, [u8; BLOCK_SIZE], CACHE_SIZE>, BLOCK_SIZE, CACHE_SIZE> {
    /// 初始化异步块缓冲
    pub fn init(device: Arc<dyn AsyncBlockDevive + Send + Sync>) -> Self {
        let mut data: [MaybeUninit<Node<usize, [u8; BLOCK_SIZE]>>; CACHE_SIZE] =
            unsafe { MaybeUninit::uninit().assume_init() };
        for elem in &mut data[..] {
            *elem = MaybeUninit::new(Node::new(0, [0; BLOCK_SIZE]));
        }
        let nodes =
            unsafe { core::mem::transmute::<_, [Node<usize, [u8; BLOCK_SIZE]>; CACHE_SIZE]>(data) };
        let lfu_cache = LFUCache::empty(nodes);
        Self {
            block_device: device,
            cache: Mutex::new(lfu_cache),
        }
    }

    /// 异步方式从块缓冲中读取一个块
    pub async fn read_block(&self, block_id: usize) -> [u8; BLOCK_SIZE] {
        {
            // 申请锁
            let mut s = self.cache.lock();
            if let Some(block) = s.get(&block_id) {
                // 如果想要读取的块在缓冲层中，则读出来直接返回，不用读写块设备
                return block;
            }
        } // 释放锁
          // 如果要读取的块不在缓冲层中，则需要从块设备读取
        let mut data = [0; BLOCK_SIZE];
        self.block_device.read(block_id, &mut data).await;
        // 将读取到的块写入到缓冲层
        let mut s = self.cache.lock(); // 申请锁
        let write_back = s.put(&block_id, data.clone());
        drop(s); // 释放锁
        if let Some((id, mut block)) = write_back {
            // 如果有需要写回到块设备的数据，这里写回
            self.block_device.write(id, &mut block).await;
        }
        data
    }

    /// 异步方式往块缓冲中写入一个块
    pub async fn write_block(&self, block_id: usize, buf: [u8; BLOCK_SIZE]) {
        let mut s = self.cache.lock(); // 申请锁
        let write_back = s.put(&block_id, buf);
        drop(s); // 释放锁
        if let Some((id, mut block)) = write_back {
            self.block_device.write(id, &mut block).await;
        }
    }
}

// use async_trait::async_trait;
// #[derive(Default)]
// struct TestDevice {}

// #[async_trait]
// impl AsyncBlockDevive for TestDevice {
//     async fn read(&self, block_id: usize, buf: &mut [u8]) {
//         todo!()
//     }
//     async fn write(&self, block_id: usize, buf: &[u8]) {
//         todo!()
//     }
// }

// lazy_static::lazy_static! {
//     pub static ref AsyncBlockCache: AsyncBlockCache<LFUCache<usize, [u8; BLOCK_SIZE], CACHE_SIZE>, BLOCK_SIZE, CACHE_SIZE> = AsyncBlockCache::init(Arc::new(TestDevice::default()));
// }
