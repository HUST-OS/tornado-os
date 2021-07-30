//! 异步 SD 卡驱动
use lazy_static::lazy_static;
use async_sd::SDCardWrapper;
use async_trait::async_trait;
use async_fat32::AsyncBlockDevive;
use alloc::sync::Arc;
use alloc::boxed::Box;

lazy_static!(
    pub static ref SD_CARD: Arc<AsyncSDCard> = Arc::new(AsyncSDCard::new());
);

pub struct AsyncSDCard(pub SDCardWrapper);

impl AsyncSDCard {
    pub fn new() -> Self {
        Self(SDCardWrapper::new())
    }
    pub async fn sd_read(&self, block_id: usize, buf: &mut [u8]) {
        self.0.read(block_id, buf).await
    }
    pub async fn sd_write(&self, block_id: usize, buf: &[u8]) {
        self.0.write(block_id, buf).await
    }
}

#[async_trait]
impl AsyncBlockDevive for AsyncSDCard {
    async fn read(&self, block_id: usize, buf: &mut [u8]) {
        self.0.read(block_id, buf).await
    }
    async fn write(&self, block_id: usize, buf: &[u8]) {
        self.0.write(block_id, buf).await
    }
}

pub async fn sdcard_test() {
    println!("sdcard init");
    let mut read_buf = [0u8; 512];
    let mut write_buf = [0u8; 512];
    for i in 0..512 {
        write_buf.iter_mut().for_each(|byte| *byte = i as u8);
        SD_CARD.sd_write(i as usize, &write_buf).await;
        SD_CARD.sd_read(i as usize, &mut read_buf).await;
        assert_eq!(read_buf, write_buf);
    }
    println!("sdcard test pass");
}