//! 文件系统
use alloc::vec::Vec;
use async_fat32::FAT32;
use super::virtio::VIRTIO_BLOCK;
use super::sdcard::SD_CARD;
use alloc::sync::Arc;
use alloc::string::String;

pub struct FS(pub FAT32);

impl FS {
    #[cfg(feature = "qemu")]
    pub async fn init() -> Self {
        let device = Arc::clone(&VIRTIO_BLOCK);
        let fat32 = FAT32::init(device).await;
        Self(fat32)
    }
    #[cfg(feature = "k210")]
    pub async fn init() -> Self {
        let device = Arc::clone(&SD_CARD);
        let fat32 = FAT32::init(device).await;
        Self(fat32)
    }

    pub fn list<S: Into<String>>(&self, dir: S) -> Vec<String> {
        self.0.list(dir)
    }

    pub async fn load_binary<S: Into<String>>(&self, file: S) -> Vec<u8> {
        self.0.load_binary(file).await.expect("load binary")
    }

    pub async fn create<S: Into<String>>(&mut self, dir: S, file: S, size: u32) {
        self.0.create(dir, file, size).await.expect("create file");
    }

    pub async fn store_binary<S: Into<String>>(&mut self, file: S, src: &[u8]) {
        self.0.load_binary(file).await.expect("store binary");
    }
}