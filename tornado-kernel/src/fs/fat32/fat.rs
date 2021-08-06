use super::BLOCK_SIZE;
use crate::cache::CACHE;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::convert::TryInto;

/// `FAT` 数据结构
///
/// 这里假设一个块一个扇区
pub struct FAT {
    /// 该分区上 `FAT` 表数量
    pub fat_nums: u8,
    /// 每个 `FAT` 表占的扇区数
    pub fat_size: u32,
    /// `FAT1` 的起始扇区
    pub base: u32,
    /// 每扇区的字节数，从 `BPB` 中读出
    pub bytes_per_sector: u16,
}

impl FAT {
    /// 根据块号获取在 `FAT1` 中的扇区号
    pub fn fat_sector(&self, cluster: u32) -> u32 {
        self.base + cluster * 4 / self.bytes_per_sector as u32
    }

    /// 根据块号获取在 `FAT1` 中的扇区偏移
    pub fn fat_sector_offset(&self, cluster: u32) -> usize {
        (cluster as usize * 4) % self.bytes_per_sector as usize
    }

    /// 找到第一个空的 `FAT` 表项，返回对应数据区的块号
    ///
    /// note: 这里假设一个块对应一个扇区
    pub async fn first_blank(&self) -> Option<u32> {
        for sector_id in 0..self.fat_size {
            let block = CACHE.read_block((self.base + sector_id) as usize).await;
            for (idx, fat) in block.chunks(4).enumerate() {
                let value = u32::from_le_bytes(fat.try_into().unwrap());
                if value == 0 {
                    return Some(sector_id * self.bytes_per_sector as u32 / 4 + idx as u32);
                }
            }
        }
        None
    }

    /// 设置 `FAT` 表项的值
    ///
    /// 将块号为 `cluster` 在 `FAT` 表中的项的值设置为 `val`
    pub async fn set(&self, cluster: u32, val: u32) {
        // 获得对应扇区号
        let fat_sector = self.fat_sector(cluster) as usize;
        // 获得扇区内偏移
        let offset = self.fat_sector_offset(cluster);
        let mut block = CACHE.read_block(fat_sector).await;
        let value: [u8; 4] = val.to_le_bytes();
        block[offset..offset + 4].copy_from_slice(&value);
        CACHE.write_block(fat_sector, block).await;
    }

    /// 获得 `FAT` 表项链
    ///
    /// `first` 是第一个 `FAT` 表项对应的块号
    pub async fn get_link(&self, first: u32) -> Vec<u32> {
        let mut res = Vec::new();
        let mut fat_sector = self.fat_sector(first) as usize;
        let mut offset = self.fat_sector_offset(first);
        res.push(first);
        loop {
            let block = CACHE.read_block(fat_sector).await;
            let value = u32::from_le_bytes(block[offset..offset + 4].try_into().unwrap());
            match value {
                0x0 => {
                    panic!("FATContent must not be zero in one link!");
                }
                0xffffff8..=0xfffffff => {
                    // 文档里面提到的结束符判断伪代码：
                    // if (FATType == FAT32) {
                    //  if (FATContent >= 0xffffff8) {
                    //          IsEOF = TRUE;
                    //    }
                    // }
                    // `FAT` 链的结尾
                    break;
                }
                x => {
                    fat_sector = self.fat_sector(x) as usize;
                    offset = self.fat_sector_offset(x);
                    res.push(x);
                }
            }
        }
        res
    }
}
