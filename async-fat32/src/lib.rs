//! 异步 `FAT32` 文件系统实现
//!
//! 这是个以异步方式读写 `FAT32` 文件系统的库。
//!
//! 该库支持 `no_std` 环境，适用于嵌入式软件或操作系统内核开发，使用者需要提供`异步块设备驱动`和`异步运行时`。
//!
//! [异步块设备驱动]: https://github.com/HUST-OS/async-virtio-driver
//! [异步运行时]: https://rust-lang.github.io/async-book/02_execution/01_chapter.html
//!
//! note:
//! 1. 该 crate 默认 `FAT32` 文件系统的一个`块`对应一个`扇区`，每个`扇区`的大小为 `512` 字节
//! 2. 目前对文件读取已经有了比较好的支持
//! 3. 目前只能创建短文件名文件，并且文件大小在创建的时候写死
//! 4. 支持对已有文件写入数据
//!
//! # Example
//!
//! ```no_run
//! todo!()
//! ```
#![no_std]
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
use alloc::boxed::Box;

/// 块缓冲层的类型
pub type ABC = block_cache::AsyncBlockCache<
    cache::LFUCache<usize, [u8; BLOCK_SIZE], CACHE_SIZE>,
    BLOCK_SIZE,
    CACHE_SIZE,
>;
pub type Result<T = ()> = core::result::Result<T, FAT32Error>;

#[derive(Debug)]
pub enum FAT32Error {
    NotFound,
    CreateFileError,
}

/// 异步块设备驱动需要实现的 trait
#[async_trait]
pub trait AsyncBlockDevive {
    /// 根据块号读取一个块
    async fn read(&self, block_id: usize, buf: &mut [u8]);
    /// 根据块号写入一个块
    async fn write(&self, block_id: usize, buf: &[u8]);
}
