//! `async-fat32` 测试
use async_fat32::{AsyncBlockDevive, FAT32};
use async_std::fs::File;
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
    }
}

#[async_std::main]
async fn main() -> Result<()> {
    let file = File::open("fs.img").await?;
    let device = BlockDevice {
        file: Mutex::new(file),
    };
    let fs = FAT32::init(Arc::new(device)).await;
    println!("fs init!");
    let files = fs.list("/");
    for f in files {
        println!("[root] {}", f);
    }
    fs.list("bbb").iter().for_each(|f| println!("[bbb] {}", f));
    fs.list("cccccccccc")
        .iter()
        .for_each(|f| println!("[cccccccccc] {}", f));
    let data_rs = fs.load_binary("test.rs").await.unwrap();
    let data_c = fs.load_binary("test.c").await.unwrap();
    let data_cpp = fs.load_binary("test.cpp").await.unwrap();
    let long_file = fs.load_binary("aaaaaaaaaa.rs").await.unwrap();
    let rs_txt = String::from_utf8(data_rs).unwrap();
    let c_txt = String::from_utf8(data_c).unwrap();
    let cpp_txt = String::from_utf8(data_cpp).unwrap();
    let aaaaaaaaaa_txt = String::from_utf8(long_file).unwrap();
    let ddd = fs.load_binary("ddd.rs").await.unwrap();
    let ddd = String::from_utf8(ddd).unwrap();
    println!("test.rs: {}", rs_txt);
    println!("test.c: {}", c_txt);
    println!("test.cpp: {}", cpp_txt);
    println!("aaaaaaaaaa.rs: {}", aaaaaaaaaa_txt);
    println!("ddd.rs: {}", ddd);
    Ok(())
}
