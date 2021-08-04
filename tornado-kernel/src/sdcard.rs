//! 异步 SD 卡驱动
use alloc::boxed::Box;
use alloc::sync::Arc;
use async_sd::SDCardWrapper;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SD_CARD: Arc<AsyncSDCard> = Arc::new(AsyncSDCard::new());
}

pub struct AsyncSDCard(pub SDCardWrapper);

impl AsyncSDCard {
    pub fn new() -> Self {
        Self(SDCardWrapper::new())
    }
    pub async fn read_block(&self, block_id: usize, buf: &mut [u8]) {
        self.0.read(block_id, buf).await
    }
    pub async fn write_block(&self, block_id: usize, buf: &[u8]) {
        self.0.write(block_id, buf).await
    }
}

pub async fn sdcard_test() {
    println!("sdcard init");
    let mut read_buf = [0u8; 512];
    let mut write_buf = [0u8; 512];
    for i in 0..512 {
        write_buf.iter_mut().for_each(|byte| *byte = i as u8);
        SD_CARD.write_block(i as usize, &write_buf).await;
        SD_CARD.read_block(i as usize, &mut read_buf).await;
        assert_eq!(read_buf, write_buf);
    }
    println!("sdcard test pass");
}
