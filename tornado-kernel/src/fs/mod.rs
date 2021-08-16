//! 文件系统
//!
//! 飓风内核里面，基于异步块设备驱动，我们可以使用async/await语法编写异步文件系统，
//! 并且在编程模式上和同步代码没有太大区别。
//!
//! 我们已经实现了一款功能基本完善的异步FAT32文件系统，后面如果有时间可能会考虑支持更多的文件系统格式比如`EXTx`系列。
mod fat32;

#[allow(unused)]
use super::{sdcard::SD_CARD, virtio::VIRTIO_BLOCK};
use alloc::{string::String, sync::Arc, vec::Vec};
use async_mutex::AsyncMutex;
use core::mem::MaybeUninit;
use fat32::FAT32;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref FS: Arc<AsyncMutex<MaybeUninit<Fs>>> =
        Arc::new(AsyncMutex::new(MaybeUninit::uninit()));
}

pub struct Fs(pub FAT32);

impl Fs {
    /// 初始化文件系统
    pub async fn init() -> Self {
        let fat32 = FAT32::init().await;
        Self(fat32)
    }
    /// 列出目录下的文件/目录
    pub fn list<S: Into<String>>(&self, dir: S) -> Vec<String> {
        self.0.list(dir)
    }
    /// 加载文件的数据到内存
    pub async fn load_binary<S: Into<String>>(&self, file: S) -> Vec<u8> {
        self.0.load_binary(file).await.expect("load binary")
    }
    /// 写入文件
    #[allow(unused)]
    pub async fn store_binary<S: Into<String>>(&mut self, file: S, src: &[u8]) {
        self.0.store_binary(file, src).await.expect("store binary");
    }
    /// 创建文件
    #[allow(unused)]
    pub async fn create<S: Into<String>>(&mut self, dir: S, file: S, size: u32) {
        self.0.create(dir, file, size).await.expect("create file");
    }
}

/// 已一个异步任务的方式初始化文件系统
///
/// 通常在内核态中通过内核执行器去运行这个任务，将文件系统初始化，
/// 然后加载用户程序。
///
/// 在初始化过程中，将得到的数据结构写入到全局变量中
pub async fn fs_init() {
    let fs = Fs::init().await;
    let mut s = FS.lock().await;
    let ptr = s.as_mut_ptr();
    unsafe {
        ptr.write(fs);
    }
    println!("[kernel] fs init");
    let children = unsafe { s.assume_init_ref().list("/") };
    print!("[/]: ");
    for child in children {
        print!("{} ", child);
    }
    println!("");
}
