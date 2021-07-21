//! FAT32 File System Implementation

use super::block_cache::AsyncBlockCache;
use super::bs_bpb::*;
use super::config::*;
use super::dir_file::*;
use super::entry::*;
use super::fat::FAT;
use super::tree::NTree;
use crate::tree::AsNode;
use crate::AsyncBlockDevive;
use crate::FAT32Error;
use crate::Result;
use crate::ABC;
use alloc::sync::Arc;

pub struct FAT32 {
    bpb: [u8; BLOCK_SIZE],
    fat: Arc<FAT>,
    tree: NTree<String, Vec<u8>>,
    device: Arc<ABC>,
}

impl FAT32 {
    /// 初始化文件系统
    pub async fn init(device: Arc<dyn AsyncBlockDevive + Send + Sync>) -> Self {
        let mut bpb = [0; BLOCK_SIZE];
        device.read(0, &mut bpb).await;
        // 根据第一个扇区获取 [`FAT`]
        let fat = Arc::new(fat1(&bpb));
        let bpb = Arc::new(bpb);
        // 获取异步块缓存
        let async_block_cache = Arc::new(AsyncBlockCache::init(device));
        // 获取根目录占用的块号集合
        let root_clusters = fat.get_link(&async_block_cache, 2).await;
        let root = RootDirectory::new(
            root_clusters.clone(),
            bpb.clone(),
            Arc::clone(&async_block_cache),
        );

        let mut tree = NTree::new(Box::new(root.clone()));
        let mut long_start = false;
        let mut long_entries = Vec::new();
        let mut dirs: Vec<Box<dyn AsNode<Ident = String, Content = Vec<u8>>>> = Vec::new();
        dirs.push(Box::new(root));
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
                            let dir = Directory::new(
                                e,
                                Arc::clone(&fat),
                                Arc::clone(&bpb),
                                Arc::clone(&async_block_cache),
                            );
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
                                Arc::clone(&async_block_cache),
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
                            let file = File::new(
                                e,
                                Arc::clone(&fat),
                                Arc::clone(&bpb),
                                Arc::clone(&async_block_cache),
                            );
                            node.insert(Box::new(file));
                        } else {
                            // 长文件名文件
                            long_start = false;
                            let mut v = Vec::new();
                            while let Some(l) = long_entries.pop() {
                                v.push(l);
                            }
                            let long_file = LongFile::new(
                                e,
                                v.into_iter(),
                                Arc::clone(&bpb),
                                Arc::clone(&fat),
                                Arc::clone(&async_block_cache),
                            );
                            node.insert(Box::new(long_file));
                        }
                    }
                    0x00 => {
                        break;
                    }
                    _ => {
                        panic!("unknown dir attribute!")
                    }
                }
            }
        }
        Self {
            bpb: *bpb,
            fat,
            tree,
            device: async_block_cache,
        }
    }
    pub fn list<S: Into<String>>(&self, dir: S) -> Vec<String> {
        match self.tree.find(dir) {
            Some(node) => node
                .children_iter()
                .iter()
                .map(|n| n.inner().ident())
                .collect(),
            None => Vec::new(),
        }
    }
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
    pub async fn create<S: Into<String>>(&mut self, dir: S, file: S) -> Result<()> {
        let node = self.tree.find_mut(dir);
        if node.is_none() { return Err(FAT32Error::NotFound); }
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
                    },
                    false => name.copy_from_slice(s.as_bytes())
                }
                if let Some(fst_cluster) = self.fat.first_blank(&*self.device).await {
                    // 标记 fat 表为已占用
                    self.fat.set(&*self.device, fst_cluster, 0xfffffff).await;
                    let entry = DirectoryEntry {
                        name,
                        ext_name,
                        attribute: Attribute::ATTR_ARCHIVE,
                        _reserved: 0,
                        fst_cluster,
                        ..Default::default()
                    };
                    let node = node.unwrap();
                    // todo: 将 entry 写入块设备
                    let file = File::new(entry, Arc::clone(&self.fat), Arc::new(self.bpb.clone()), Arc::clone(&self.device));
                    node.insert(Box::new(file));
                    Ok(())
                } else {
                    Err(FAT32Error::CreateFileError)
                }
            },
            true => {
                // 长文件名
                todo!()
            }
        }
    }
    fn is_long<S: AsRef<str>>(s: &S) -> bool {
        let s = s.as_ref();
        match s.len() {
            0 => panic!("empty name!"),
            1..=11 => {
                match s.contains(".") {
                    true => {
                        let mut s = s.split(".").collect::<Vec<_>>();
                        let ext = s.pop().unwrap();
                        if ext.len() > 3 {
                            true
                        } else {
                            let len = s.iter().fold(0, |acc, &x| acc + x.len());
                            len > 8
                        }
                    },
                    false => { s.len() > 8 }
                }
            },
            _ => true
        }
    }
}
