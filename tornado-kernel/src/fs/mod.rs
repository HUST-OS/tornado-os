//! 文件系统
mod fat32;

use core::intrinsics::copy;
use alloc::vec::Vec;
use alloc::sync::Arc;
use alloc::string::String;
use core::mem::MaybeUninit;
use async_mutex::AsyncMutex;
use fat32::FAT32;
use lazy_static::lazy_static;
use super::virtio::VIRTIO_BLOCK;
use super::sdcard::SD_CARD;
use super::memory::{
    PhysicalAddress,
    PhysicalPageNumber,
    PAGE_SIZE,
    KERNEL_MAP_OFFSET,
    MemorySet
};

lazy_static!(
    pub static ref FS: Arc<AsyncMutex<MaybeUninit<Fs>>> = unsafe { Arc::new(AsyncMutex::new(MaybeUninit::uninit())) };
);

pub struct Fs(pub FAT32);

impl Fs {
    pub async fn init() -> Self {
        let fat32 = FAT32::init().await;
        Self(fat32)
    }
    pub fn list<S: Into<String>>(&self, dir: S) -> Vec<String> {
        self.0.list(dir)
    }
    pub async fn load_binary<S: Into<String>>(&self, file: S) -> Vec<u8> {
        self.0.load_binary(file).await.expect("load binary")
    }
    pub async fn store_binary<S: Into<String>>(&mut self, file: S, src: &[u8]) {
        self.0.load_binary(file).await.expect("store binary");
    }
    pub async fn load_user<S: Into<String>>(&self, user: S, pa: PhysicalAddress) -> MemorySet {
        let data = self.load_binary(user).await;
        let pages = data.len() / PAGE_SIZE + (data.len() % PAGE_SIZE != 0) as usize;
        unsafe {
            let src = data.as_ptr();
            let dst = (pa.0 + KERNEL_MAP_OFFSET) as *const () as *mut u8;
            copy(src, dst, data.len());
        }
        let mm_set = MemorySet::new_bin(pa.0, pages).expect("create user memory set");
        mm_set
    }
    pub async fn create<S: Into<String>>(&mut self, dir: S, file: S, size: u32) {
        self.0.create(dir, file, size).await.expect("create file");
    }
}

pub async fn fs_init() {
    let fs = Fs::init().await;
    let mut s = FS.lock().await;
    let ptr = s.as_mut_ptr();
    unsafe {
        ptr.write(fs);
    }
    println!("fs init");
    let children = unsafe { s.assume_init_ref().list("/") };
    print!("[/]: ");
    for child in children {
        print!("{} ", child);
    }
    println!("");
}
