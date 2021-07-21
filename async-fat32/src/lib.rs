//! Async FAT32 File System Implemetation

mod block_cache;
mod bs_bpb;
mod cache;
mod config;
mod dir_file;
mod entry;
mod fat;
mod fs;
mod tree;

pub use fs::FAT32;

extern crate alloc;

use async_trait::async_trait;
use config::*;

pub type ABC = block_cache::AsyncBlockCache<
    cache::LFUCache<usize, [u8; BLOCK_SIZE], CACHE_SIZE>,
    BLOCK_SIZE,
    CACHE_SIZE,
>;

pub type Result<T = ()> = core::result::Result<T, FAT32Error>;

#[derive(Debug)]
pub enum FAT32Error {
    NotFound,
    CreateFileError
}

#[async_trait]
pub trait AsyncBlockDevive {
    async fn read(&self, block_id: usize, buf: &mut [u8]);
    async fn write(&self, block_id: usize, buf: &[u8]);
}
