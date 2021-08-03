//! 块缓冲层实现
//!
//! 作用：
//! * 提高 IO 性能
//! * 维护数据一致性
//!

mod lfu;

use crate::sdcard::{AsyncSDCard, SD_CARD};
use crate::virtio::{async_blk::VirtIOAsyncBlock, VIRTIO_BLOCK};
use alloc::sync::Arc;
use alloc::vec::Vec;
use async_mutex::AsyncMutex;
use core::mem::MaybeUninit;
use lazy_static::lazy_static;
use lfu::LFUCache;

const BLOCK_SIZE: usize = 512;
const CACHE_SIZE: usize = 4;

pub type BlockCache =
    AsyncBlockCache<LFUCache<usize, [u8; BLOCK_SIZE], CACHE_SIZE>, BLOCK_SIZE, CACHE_SIZE>;

#[cfg(feature = "qemu")]
type AsyncBlockDevice = Arc<VirtIOAsyncBlock>;

#[cfg(feature = "k210")]
type AsyncBlockDevice = Arc<AsyncSDCard>;

#[cfg(feature = "qemu")]
lazy_static! {
    pub static ref CACHE: BlockCache = BlockCache::init(Arc::clone(&VIRTIO_BLOCK));
};

#[cfg(feature = "k210")]
lazy_static! {
    pub static ref CACHE: BlockCache = BlockCache::init(Arc::clone(&SD_CARD));
};

/// 各种缓存替换算法需要实现的 trait
///
/// N: 缓存项的数量
pub trait Cache<const N: usize> {
    type Key;
    type Value;
    /// 根据 `Key` 返回对应的 `Value`
    fn get(&mut self, key: &Self::Key) -> Option<Self::Value>;
    /// 写入一对 `(Key, Value)`
    ///
    /// 如果有需要写回的值，将它返回
    fn put(&mut self, key: &Self::Key, value: Self::Value) -> Option<(Self::Key, Self::Value)>;
    /// 返回所有的缓存项，用于数据同步
    fn all(&mut self) -> Vec<(Self::Key, Self::Value)>;
}

/// 异步块缓存层
/// B: 一个块中的字节数
/// N: 块缓冲层的块数

pub struct AsyncBlockCache<
    C: Cache<N, Key = usize, Value = [u8; B]> + Send + Sync,
    const B: usize,
    const N: usize,
> {
    /// 可以是采用各种替换算法的缓存具体实现
    cache: AsyncMutex<C>,
    /// 异步块设备驱动
    device: AsyncBlockDevice,
}

impl AsyncBlockCache<LFUCache<usize, [u8; BLOCK_SIZE], CACHE_SIZE>, BLOCK_SIZE, CACHE_SIZE> {
    /// 初始化异步块缓存
    pub fn init(device: AsyncBlockDevice) -> Self {
        let mut data: [MaybeUninit<Node<usize, [u8; BLOCK_SIZE]>>; CACHE_SIZE] =
            unsafe { MaybeUninit::uninit().assume_init() };
        for elem in &mut data[..] {
            *elem = MaybeUninit::new(Node::new(0, [0; BLOCK_SIZE]));
        }
        let nodes =
            unsafe { core::mem::transmute::<_, [Node<usize, [u8; BLOCK_SIZE]>; CACHE_SIZE]>(data) };

        let lfu_cache = LFUCache::empty(nodes);
        Self {
            cache: AsyncMutex::new(lfu_cache),
            device,
        }
    }

    /// 异步方式从块缓存中读取一个块
    pub async fn read_block(&self, block_id: usize) -> [u8; BLOCK_SIZE] {
        {
            // 申请锁
            let mut s = self.cache.lock().await;
            if let Some(block) = s.get(&block_id) {
                // 如果想要读取的块在缓冲层中，则读出来直接返回，不用读写块设备
                return block;
            }
        } // 释放锁
          // 如果要读取的块不在缓冲层中，则需要从块设备读取
        let mut data = [0; BLOCK_SIZE];
        self.device.read_block(block_id, &mut data).await;
        // 将读取到的块写入到缓冲层
        let mut s = self.cache.lock().await; // 申请锁
        let write_back = s.put(&block_id, data.clone());
        drop(s); // 释放锁
        if let Some((id, mut block)) = write_back {
            // 如果有需要写回到块设备的数据，这里写回
            self.device.write_block(id, &mut block).await;
        }
        data
    }

    /// 异步方式往块缓冲中写入一个块
    pub async fn write_block(&self, block_id: usize, buf: [u8; BLOCK_SIZE]) {
        let mut s = self.cache.lock().await; // 申请锁
        let write_back = s.put(&block_id, buf);
        drop(s); // 释放锁
        if let Some((id, mut block)) = write_back {
            self.device.write_block(id, &mut block).await;
        }
    }

    /// 异步，写穿方式往缓冲区中写入一个块
    pub async fn write_sync(&self, block_id: usize, buf: [u8; BLOCK_SIZE]) {
        self.write_block(block_id, buf.clone()).await;
        self.device.write_block(block_id, &buf).await
    }

    /// 将缓冲层中的所有数据写回到块设备
    pub async fn sync(&self) {
        let mut s = self.cache.lock().await;
        for (id, block) in s.all() {
            self.device.write_block(id, &block).await;
        }
    }
}

/// 缓存项
///
/// 除了记录键值对，还记录访问次数，最后访问时间，是否写脏
#[derive(Clone, Copy)]
pub struct Node<K: Eq + PartialEq + Copy, V: Clone> {
    key: K,
    value: V,
    cnt: usize,
    time: usize,
    dirty: bool,
}

impl<K: Eq + PartialEq + Copy, V: Clone> Node<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            cnt: 0,
            time: 0,
            dirty: false,
        }
    }
}

impl<K: Eq + PartialEq + Copy, V: Clone> PartialEq for Node<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.cnt == other.cnt
    }
}

impl<K: Eq + PartialEq + Copy, V: Clone> Eq for Node<K, V> {}

impl<K: Eq + PartialEq + Copy, V: Clone> Ord for Node<K, V> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.cnt
            .cmp(&other.cnt)
            .then_with(|| self.time.cmp(&other.time))
    }
}

impl<K: Eq + PartialEq + Copy, V: Clone> PartialOrd for Node<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
