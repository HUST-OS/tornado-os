use super::bs_bpb::cluster_offset_sectors;
use super::entry::{Attribute, DirectoryEntry, LongDirectoryEntry};
use super::fat::FAT;
use super::tree::AsNode;
use super::BLOCK_SIZE;
use crate::cache::CACHE;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;
use async_trait::async_trait;
use core::iter::FromIterator;

#[derive(Clone)]
struct Inner {
    /// 目录项
    entry: DirectoryEntry,
    /// `BPB`
    ///
    /// note: 这里假设一个块对应一个扇区
    bpb: Arc<[u8; BLOCK_SIZE]>,
    /// [`FAT`] 表
    fat: Arc<FAT>,
}

impl Inner {
    pub fn new(entry: DirectoryEntry, fat: Arc<FAT>, bpb: Arc<[u8; BLOCK_SIZE]>) -> Self {
        Self { entry, bpb, fat }
    }
    pub fn name(&self) -> String {
        self.entry.name()
    }
    pub async fn data(&self) -> Vec<u8> {
        self.entry.load(&self.fat, &self.bpb).await
    }
}

/// 短文件名目录
#[derive(Clone)]
pub struct Directory {
    inner: Inner,
}

impl Directory {
    /// 新建
    pub fn new(entry: DirectoryEntry, fat: Arc<FAT>, bpb: Arc<[u8; BLOCK_SIZE]>) -> Self {
        assert_eq!(entry.attribute, Attribute::ATTR_DIRECTORY);
        Self {
            inner: Inner::new(entry, fat, bpb),
        }
    }
    /// 目录名
    pub fn name(&self) -> String {
        self.inner.name()
    }
    /// 获取该目录对应的数据区数据，返回值以目录项大小为单位分割
    pub async fn children(&self) -> Vec<[u8; 32]> {
        let data = self.inner.data().await;
        data.chunks(32)
            .map(|b| {
                let mut entry = [0; 32];
                entry.copy_from_slice(b);
                entry
            })
            .collect()
    }
}

#[async_trait]
impl AsNode for Directory {
    type Ident = String;
    type Content = Vec<u8>;
    type ContentRef = Vec<u32>;
    fn identify(&self, ident: &Self::Ident) -> bool {
        self.name() == *ident
    }
    fn ident(&self) -> Self::Ident {
        self.name()
    }
    async fn content(&self) -> Self::Content {
        self.inner.data().await
    }
    async fn content_ref(&self) -> Self::ContentRef {
        self.inner.entry.clusters(&self.inner.fat).await
    }
}

/// 短文件名文件
pub struct File {
    inner: Inner,
}

impl File {
    /// 新建
    pub fn new(entry: DirectoryEntry, fat: Arc<FAT>, bpb: Arc<[u8; BLOCK_SIZE]>) -> Self {
        Self {
            inner: Inner::new(entry, fat, bpb),
        }
    }
    /// 文件名
    pub fn name(&self) -> String {
        self.inner.name()
    }
    /// 文件大小
    pub fn size(&self) -> u32 {
        self.inner.entry.file_size
    }
    /// 文件对应的数据区数据
    pub async fn data(&self) -> Vec<u8> {
        self.inner.data().await
    }
}

#[async_trait]
impl AsNode for File {
    type Ident = String;
    type Content = Vec<u8>;
    type ContentRef = Vec<u32>;
    fn identify(&self, ident: &Self::Ident) -> bool {
        self.name() == *ident
    }
    fn ident(&self) -> Self::Ident {
        self.name()
    }
    async fn content(&self) -> Self::Content {
        let ret = self.data().await;
        ret[..self.inner.entry.file_size as usize].to_vec()
    }
    async fn content_ref(&self) -> Self::ContentRef {
        self.inner.entry.clusters(&self.inner.fat).await
    }
}

#[derive(Clone)]
struct LongInner {
    /// 短目录项
    entry: DirectoryEntry,
    /// 长目录项
    ///
    /// 排序根据 `LongOrder` 从小到大排列
    long_entries: Vec<LongDirectoryEntry>,
    /// `BPB`
    ///
    /// note: 这里假设一个块对应一个扇区
    bpb: Arc<[u8; BLOCK_SIZE]>,
    /// `[FAT]` 表
    fat: Arc<FAT>,
}

impl LongInner {
    pub fn new<I: Iterator<Item = LongDirectoryEntry>>(
        entry: DirectoryEntry,
        long_entries: I,
        bpb: Arc<[u8; BLOCK_SIZE]>,
        fat: Arc<FAT>,
    ) -> Self {
        Self {
            entry,
            long_entries: Vec::from_iter(long_entries),
            bpb,
            fat,
        }
    }
    pub fn name(&self) -> String {
        let mut name = String::new();
        for l in &self.long_entries {
            l.name().iter().for_each(|c| name.push(*c));
        }
        name
    }
    pub async fn data(&self) -> Vec<u8> {
        self.entry.load(&self.fat, &self.bpb).await
    }
}

/// 长文件名目录
#[derive(Clone)]
pub struct LongDirectory {
    inner: LongInner,
}

impl LongDirectory {
    /// 新建
    pub fn new<I: Iterator<Item = LongDirectoryEntry>>(
        entry: DirectoryEntry,
        long_entries: I,
        bpb: Arc<[u8; BLOCK_SIZE]>,
        fat: Arc<FAT>,
    ) -> Self {
        assert_eq!(entry.attribute, Attribute::ATTR_DIRECTORY);
        Self {
            inner: LongInner::new(entry, long_entries, bpb, fat),
        }
    }
    /// 目录名
    pub fn name(&self) -> String {
        self.inner.name()
    }
    /// 同 [`Directory`]
    pub async fn children(&self) -> Vec<[u8; 32]> {
        let data = self.inner.data().await;
        data.chunks(32)
            .map(|b| {
                let mut entry = [0; 32];
                entry.copy_from_slice(b);
                entry
            })
            .collect()
    }
}

#[async_trait]
impl AsNode for LongDirectory {
    type Ident = String;
    type Content = Vec<u8>;
    type ContentRef = Vec<u32>;
    fn identify(&self, ident: &Self::Ident) -> bool {
        self.name() == *ident
    }
    fn ident(&self) -> Self::Ident {
        self.name()
    }
    async fn content(&self) -> Self::Content {
        self.inner.data().await
    }
    async fn content_ref(&self) -> Self::ContentRef {
        self.inner.entry.clusters(&self.inner.fat).await
    }
}

/// 长文件名文件
pub struct LongFile {
    inner: LongInner,
}

impl LongFile {
    /// 新建
    pub fn new<I: Iterator<Item = LongDirectoryEntry>>(
        entry: DirectoryEntry,
        long_entries: I,
        bpb: Arc<[u8; BLOCK_SIZE]>,
        fat: Arc<FAT>,
    ) -> Self {
        Self {
            inner: LongInner::new(entry, long_entries, bpb, fat),
        }
    }
    /// 文件名
    pub fn name(&self) -> String {
        self.inner.name()
    }
    /// 同 [`File`]
    pub async fn data(&self) -> Vec<u8> {
        self.inner.data().await
    }
}

#[async_trait]
impl AsNode for LongFile {
    type Ident = String;
    type Content = Vec<u8>;
    type ContentRef = Vec<u32>;
    fn identify(&self, ident: &Self::Ident) -> bool {
        self.name() == *ident
    }
    fn ident(&self) -> Self::Ident {
        self.name()
    }
    async fn content(&self) -> Self::Content {
        self.data().await[..self.inner.entry.file_size as usize].to_vec()
    }
    async fn content_ref(&self) -> Self::ContentRef {
        self.inner.entry.clusters(&self.inner.fat).await
    }
}

/// 根目录
#[derive(Clone)]
pub struct RootDirectory {
    name: String,
    bpb: Arc<[u8; BLOCK_SIZE]>,
    /// 占用的块号
    clusters: Vec<u32>,
}

impl RootDirectory {
    pub fn new(clusters: Vec<u32>, bpb: Arc<[u8; BLOCK_SIZE]>) -> Self {
        Self {
            name: "/".to_string(),
            bpb,
            clusters,
        }
    }
}

#[async_trait]
impl AsNode for RootDirectory {
    type Ident = String;
    type Content = Vec<u8>;
    type ContentRef = Vec<u32>;
    fn identify(&self, ident: &Self::Ident) -> bool {
        &self.name == ident
    }
    fn ident(&self) -> Self::Ident {
        self.name.clone()
    }
    async fn content(&self) -> Self::Content {
        let mut ret = Vec::new();
        for cluster in &self.clusters {
            let cluster = cluster_offset_sectors(&*self.bpb, *cluster);
            let block = CACHE.read_block(cluster as usize).await;
            block.iter().for_each(|b| ret.push(*b));
        }
        ret
    }
    async fn content_ref(&self) -> Self::ContentRef {
        self.clusters.clone()
    }
}
