//! 异步FAT32文件系统
//!
//! 目前支持的功能：
//! * 初始化fat32文件系统
//! * 读取文件
//! * 创建短文件名文件
//! * 写入文件数据，但文件大小需要在创建的时候指定
//!
//! # Example: 列出根目录下的所有文件和目录
//! ```
//! async {
//!     let fs = FAT32::init().await;
//!     let files = fs.list("/");
//!
//!     // 读取第一个文件的数据
//!     let content = fs.load_binary(files[0]).await.unwrap();
//! }
//! ```
mod bs_bpb;
mod dir_file;
mod entry;
mod fat;
mod tree;

use crate::cache::CACHE;
use crate::sdcard::AsyncSDCard;
use crate::virtio::async_blk::VirtIOAsyncBlock;
use alloc::{boxed::Box, string::String, sync::Arc, vec::Vec};
use bs_bpb::*;
use dir_file::*;
use entry::*;
use fat::*;
use tree::*;

const BLOCK_SIZE: usize = 512;

type Result<T = ()> = core::result::Result<T, FAT32Error>;

#[derive(Debug)]
pub enum FAT32Error {
    NotFound,
    CreateFileError,
}

/// FAT32 文件系统实现
pub struct FAT32 {
    bpb: [u8; BLOCK_SIZE],
    fat: Arc<FAT>,
    tree: NTree<String, Vec<u8>, Vec<u32>>,
}

impl FAT32 {
    /// 初始化文件系统
    ///
    /// # Example:
    ///
    /// ```
    /// async {
    ///     let fs = FAT32::init().await;       
    /// }
    /// ```
    pub async fn init() -> Self {
        let bpb = CACHE.read_block(0usize).await;
        // 根据第一个扇区获取 [`FAT`]
        let fat = Arc::new(fat1(&bpb));
        let bpb = Arc::new(bpb);
        // 获取根目录占用的块号集合
        let root_clusters = fat.get_link(2).await;
        let root = RootDirectory::new(root_clusters.clone(), bpb.clone());
        /*
        let fat_offset = fat1_offset_bytes(&*bpb);
        let root_offset = cluster_offset_bytes(&*bpb, 2);
        println!(
            "fat offset: {:x}, root offset: {:x}",
            fat_offset, root_offset
        );*/
        let mut tree = NTree::new(Box::new(root.clone()));
        let mut long_start = false;
        let mut long_entries = Vec::new();
        let mut dirs: Vec<
            Box<dyn AsNode<Ident = String, Content = Vec<u8>, ContentRef = Vec<u32>> + Send + Sync>,
        > = Vec::new();
        dirs.push(Box::new(root));
        // 利用栈`dirs`遍历`FAT32`文件系统中所有子目录
        while let Some(dir) = dirs.pop() {
            let data = dir.content().await;
            let node = tree.find_mut(dir.ident()).expect("can not found node");
            for entry in data.as_slice().chunks(32) {
                let mut e = [0; 32];
                e.copy_from_slice(entry);
                match entry[11] {
                    0x10 => {
                        let e = DirectoryEntry::from(e);
                        if e.is_dot() || e.is_dotdot() || e.is_free() || e.is_deleted() {
                            // 忽略 `.` `..` 还有空的和被删除的目录项
                            if long_start {
                                long_entries.clear();
                                long_start = false;
                            }
                            continue;
                        }
                        if !long_start {
                            // 短文件名目录，直接插入到目录树
                            let dir = Directory::new(e, Arc::clone(&fat), Arc::clone(&bpb));
                            dirs.push(Box::new(dir.clone()));
                            node.insert(Box::new(dir));
                        } else {
                            // 长文件名目录
                            long_start = false;
                            let mut v = Vec::new();
                            // 将长文件名目录项存放顺序倒过来
                            while let Some(l) = long_entries.pop() {
                                v.push(l);
                            }
                            let long_dir = LongDirectory::new(
                                e,
                                v.into_iter(),
                                Arc::clone(&bpb),
                                Arc::clone(&fat),
                            );
                            dirs.push(Box::new(long_dir.clone()));
                            node.insert(Box::new(long_dir));
                        }
                    }
                    0x0f => {
                        // 长文件名
                        long_start = true;
                        let e = LongDirectoryEntry::from(e);
                        long_entries.push(e);
                    }
                    0x01 | 0x02 | 0x04 | 0x08 | 0x20 => {
                        let e = DirectoryEntry::from(e);
                        if e.is_dot() || e.is_dotdot() || e.is_free() || e.is_deleted() {
                            // 忽略 `.` `..` 还有空的和被删除的目录
                            if long_start {
                                long_entries.clear();
                                long_start = false;
                            }
                            continue;
                        }
                        if !long_start {
                            // 短文件名文件
                            let file = File::new(e, Arc::clone(&fat), Arc::clone(&bpb));
                            node.insert(Box::new(file));
                        } else {
                            // 长文件名文件
                            long_start = false;
                            let mut v = Vec::new();
                            while let Some(l) = long_entries.pop() {
                                v.push(l);
                            }
                            let long_file =
                                LongFile::new(e, v.into_iter(), Arc::clone(&bpb), Arc::clone(&fat));
                            node.insert(Box::new(long_file));
                        }
                    }
                    0x00 => {
                        break;
                    }
                    x => {
                        panic!("unknown dir attribute: {}!", x)
                    }
                }
            }
        }
        Self {
            bpb: *bpb,
            fat,
            tree,
        }
    }
    /// 列出某个子目录下的所有文件和目录
    ///
    /// # Example:
    ///
    /// ```
    /// async {
    ///     let fs = FAT32::init().await;
    ///     let files = fs.list("/").await;       
    /// }
    /// ```
    pub fn list<S: Into<String>>(&self, dir: S) -> Vec<String> {
        match self.tree.find(dir) {
            Some(node) => node
                .children_ref()
                .iter()
                .map(|n| n.inner().ident())
                .collect(),
            None => Vec::new(),
        }
    }
    /// 加载某个文件或目录的二进制数据
    ///
    /// # Example:
    ///
    /// ```
    /// async {
    ///     let fs = FAT32::init().await;
    ///     let files = fs.list("/").await;
    ///
    ///     let content = fs.load_binary(files[0]).await.unwrap();       
    /// }
    /// ```
    pub async fn load_binary<S: Into<String>>(&self, file: S) -> Result<Vec<u8>> {
        match self.tree.find(file) {
            Some(node) => Ok(node.inner().content().await),
            None => Err(FAT32Error::NotFound),
        }
    }
    /// 创建空文件(unfinished)
    ///
    /// warn: 这里可能会出现数据冲突的情况
    /// 比如一个文件在创建的时候在找到空 `FAT` 表项到把 `0xfffffff` 写入到该 `FAT` 表项的时间之内，
    /// 另外一个任务在读写这个 `FAT` 表项。
    ///
    /// 一种解决办法：在内存里创建一个数据结构来对 `FAT` 表进行管理
    ///
    /// # Example:
    ///
    /// ```
    /// async {
    ///     let mut fs = FAT32::init().await;
    ///     fs.create("/", "test.rs", 500).unwrap();       
    /// }
    /// ```
    pub async fn create<S: Into<String>>(&mut self, dir: S, file: S, size: u32) -> Result<()> {
        let node = self.tree.find_mut(dir);
        if node.is_none() {
            return Err(FAT32Error::NotFound);
        }
        let s: String = file.into();
        match Self::is_long(&s) {
            false => {
                // 短文件名
                let mut name = [0x20; 8];
                let mut ext_name = [0x20; 3];
                match s.contains(".") {
                    true => {
                        let mut v = s.split(".").collect::<Vec<_>>();
                        let last = v.pop().unwrap();
                        ext_name.copy_from_slice(last.as_bytes());
                        for (idx, c) in v.iter().flat_map(|ss| ss.chars()).enumerate() {
                            name[idx] = c as u8;
                        }
                    }
                    false => name[0..s.len()].copy_from_slice(s.as_bytes()),
                }
                if let Some(fst_cluster) = self.fat.first_blank().await {
                    // 标记 `fat` 表为已占用
                    self.fat.set(fst_cluster, 0xfffffff).await;
                    let mut last = fst_cluster;
                    // 分配足够的 `FAT` 表项
                    for _ in 0..size - 1 {
                        let new_cluster = self
                            .fat
                            .first_blank()
                            .await
                            .map_or_else(|| panic!("no avaiable space!"), |x| x);
                        self.fat.set(last, new_cluster).await;
                        last = new_cluster;
                    }
                    // 更新最后一项 `FAT` 表
                    self.fat.set(last, 0xfffffff).await;
                    let entry = DirectoryEntry {
                        name,
                        ext_name,
                        attribute: Attribute::ATTR_ARCHIVE,
                        _reserved: 0,
                        file_size: size * BLOCK_SIZE as u32,
                        fst_cluster,
                        ..Default::default()
                    };
                    let node = node.unwrap();
                    // 下面将 entry 写入块设备
                    // 获取父节点结点占用的块号
                    let clusters = node.inner().content_ref().await;
                    let mut has_free = false;
                    let mut free_entry = (0, 0);
                    for cluster in &clusters {
                        // 获取块号对应的扇区偏移
                        let sector = cluster_offset_sectors(&self.bpb, *cluster);
                        let block = CACHE.read_block(sector as usize).await;
                        for (idx, fat) in block.chunks(32).enumerate() {
                            if fat.iter().all(|b| *b == 0x0) {
                                has_free = true;
                                free_entry = (sector, idx);
                                break;
                            }
                        }
                        if has_free {
                            break;
                        }
                    }
                    if has_free {
                        // 如果有空的 `FAT` 表项
                        let mut block = CACHE.read_block(free_entry.0 as usize).await;
                        for (idx, e) in block.chunks_mut(32).enumerate() {
                            if idx == free_entry.1 {
                                let new_e: [u8; 32] = entry.clone().into();
                                e.copy_from_slice(&new_e);
                                break;
                            }
                        }
                        // 写回块设备
                        CACHE.write_block(free_entry.0 as usize, block).await;
                    } else {
                        // 如果父节点占据的块里面所有目录项都被占用了，则需要申请新的块
                        if let Some(new_cluster) = self.fat.first_blank().await {
                            // 父节点最后的块号
                            let last = *clusters.last().unwrap();
                            // 更新 `FAT` 表
                            self.fat.set(last, new_cluster).await;
                            self.fat.set(new_cluster, 0xfffffff).await;
                            // 将新的块读取进内存
                            let sector = cluster_offset_sectors(&self.bpb, new_cluster);
                            let mut block = CACHE.read_block(sector as usize).await;
                            // 设置第一项的值
                            let e: [u8; 32] = entry.clone().into();
                            block[0..32].copy_from_slice(&e);
                            // 写回块设备
                            CACHE.write_block(sector as usize, block).await;
                        } else {
                            panic!("no avaiable space in disk!")
                        }
                    }
                    let file = File::new(entry, Arc::clone(&self.fat), Arc::new(self.bpb.clone()));
                    // 更新目录树
                    node.insert(Box::new(file));
                    Ok(())
                } else {
                    panic!("no avaiable space in disk!")
                }
            }
            true => {
                // 长文件名
                todo!()
            }
        }
    }
    /// 写入文件
    ///
    /// note: 这里语法上不需要可变引用，语义上需要
    ///
    /// # Example:
    ///
    /// ```
    /// todo!()
    /// ```
    pub async fn store_binary<S: Into<String>>(&mut self, file: S, src: &[u8]) -> Result<()> {
        if let Some(node) = self.tree.find(file) {
            // 需要的块数
            let size = src.len() / BLOCK_SIZE + 1;
            let mut clusters = node.inner().content_ref().await;
            if size > clusters.len() {
                // 需要分配新的块
                let diff = size - clusters.len();
                let mut last = *clusters.last().unwrap();
                for _ in 0..diff {
                    let new_cluster = self
                        .fat
                        .first_blank()
                        .await
                        .map_or_else(|| panic!("no avaiable space!"), |x| x);
                    self.fat.set(last, new_cluster).await;
                    last = new_cluster;
                }
                self.fat.set(last, 0xfffffff).await;
                // 更新完 `FAT` 表重新获得文件占用的块数
                clusters = node.inner().content_ref().await;
            }
            for b in src.chunks(BLOCK_SIZE) {
                let cluster = clusters.remove(0);
                let sector = cluster_offset_sectors(&self.bpb, cluster) as usize;
                let mut block = CACHE.read_block(sector).await;
                block[0..b.len()].copy_from_slice(b);
                block[b.len()..].fill(0);
                CACHE.write_block(sector, block).await;
            }
            // 清空剩余的块
            for cluster in clusters {
                let sector = cluster_offset_sectors(&self.bpb, cluster) as usize;
                let mut block = CACHE.read_block(sector).await;
                block.fill(0);
                CACHE.write_block(sector, block).await;
            }
            Ok(())
        } else {
            Err(FAT32Error::NotFound)
        }
    }

    /// 判断是否是长文件名
    ///
    /// # Example:
    ///
    /// ```
    /// todo!()
    /// ```
    fn is_long<S: AsRef<str>>(s: &S) -> bool {
        let s = s.as_ref();
        match s.len() {
            0 => panic!("empty name!"),
            1..=11 => match s.contains(".") {
                true => {
                    let mut s = s.split(".").collect::<Vec<_>>();
                    let ext = s.pop().unwrap();
                    if ext.len() > 3 {
                        true
                    } else {
                        let len = s.iter().fold(0, |acc, &x| acc + x.len());
                        len > 8
                    }
                }
                false => s.len() > 8,
            },
            _ => true,
        }
    }
}
