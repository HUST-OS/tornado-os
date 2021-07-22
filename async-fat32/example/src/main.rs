//! `async-fat32` 测试
use async_fat32::{AsyncBlockDevive, FAT32};
use async_std::fs::File;
use async_std::fs::OpenOptions;
use async_std::io::prelude::{ReadExt, SeekExt, WriteExt};
use async_std::io::Result;
use async_std::io::SeekFrom;
use async_std::sync::Arc;
use async_std::sync::Mutex;
use async_trait::async_trait;

const BLOCK_SIZE: u64 = 512;
struct BlockDevice {
    file: Mutex<File>,
}

#[async_trait]
impl AsyncBlockDevive for BlockDevice {
    async fn read(&self, block_id: usize, buf: &mut [u8]) {
        let mut f = self.file.lock().await;
        f.seek(SeekFrom::Start(block_id as u64 * BLOCK_SIZE))
            .await
            .unwrap();
        let _n = f.read(buf).await.unwrap();
    }
    async fn write(&self, block_id: usize, buf: &[u8]) {
        let mut f = self.file.lock().await;
        f.seek(SeekFrom::Start(block_id as u64 * BLOCK_SIZE))
            .await
            .unwrap();
        let _n = f.write(buf).await.unwrap();
        f.sync_data().await.unwrap();
    }
}

#[async_std::main]
async fn main() -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("fs.img")
        .await?;
    let device = BlockDevice {
        file: Mutex::new(file),
    };
    let mut fs = FAT32::init(Arc::new(device)).await;
    println!("fs init!");
    let files = fs.list("/");
    for f in files {
        println!("[root] {}", f);
    }
    fs.create("/", "test").await.expect("create file failed");
    fs.sync().await;
    Ok(())
}
