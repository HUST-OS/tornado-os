use super::bs_bpb::cluster_offset_sectors;
use crate::{config::BLOCK_SIZE, fat::FAT, ABC};
use alloc::format;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use bit_field::BitField;
use core::convert::TryInto;

/// 短文件名目录项
#[derive(Clone, Default)]
pub struct DirectoryEntry {
    /// 文件名
    pub name: [u8; 8],
    /// 文件扩展名
    pub ext_name: [u8; 3],
    /// 文件属性
    pub attribute: Attribute,
    /// Windows NT 保留项
    pub _reserved: u8,
    /// millisecond stamp at file creation time
    pub tenth: u8,
    /// 文件创建时间
    pub crt_time: Time,
    /// 文件创建日期
    pub crt_date: Date,
    /// 文件最后一次访问日期
    pub last_acc_date: Date,
    /// 该目录的第一个块号
    pub fst_cluster: u32,
    /// 最后一次写操作的时间
    pub wrt_time: Time,
    /// 最后一次写操作的日期
    pub wrt_date: Date,
    /// 文件字节大小
    pub file_size: u32,
}

impl From<[u8; 32]> for DirectoryEntry {
    fn from(src: [u8; 32]) -> Self {
        let mut name = [0; 8];
        name.copy_from_slice(&src[0..8]);
        let mut ext_name = [0; 3];
        ext_name.copy_from_slice(&src[8..11]);
        let attribute = Attribute::from(src[11]);
        let _reserved = src[12];
        let tenth = src[13];
        let crt_time = u16::from_le_bytes(src[14..16].try_into().unwrap());
        let crt_time = Time(crt_time);
        let crt_date = u16::from_le_bytes(src[16..18].try_into().unwrap());
        let crt_date = Date(crt_date);
        let last_acc_date = u16::from_le_bytes(src[18..20].try_into().unwrap());
        let last_acc_date = Date(last_acc_date);
        let mut l = [0; 2];
        let mut h = [0; 2];
        l.copy_from_slice(&src[26..28]);
        h.copy_from_slice(&src[20..22]);
        let fst_cluster = [l, h].concat();
        let fst_cluster = u32::from_le_bytes(fst_cluster.try_into().unwrap());
        let wrt_time = u16::from_le_bytes(src[22..24].try_into().unwrap());
        let wrt_time = Time(wrt_time);
        let wrt_date = u16::from_le_bytes(src[24..26].try_into().unwrap());
        let wrt_date = Date(wrt_date);
        let file_size = u32::from_le_bytes(src[28..32].try_into().unwrap());
        Self {
            name,
            ext_name,
            attribute,
            _reserved,
            tenth,
            crt_time,
            crt_date,
            last_acc_date,
            fst_cluster,
            wrt_time,
            wrt_date,
            file_size,
        }
    }
}

impl Into<[u8; 32]> for DirectoryEntry {
    fn into(self) -> [u8; 32] {
        let mut res = [0; 32];
        res[0..8].copy_from_slice(&self.name);
        res[8..11].copy_from_slice(&self.ext_name);
        res[11] = self.attribute as u8;
        res[12] = self._reserved;
        res[13] = self.tenth;
        let crt_time: [u8; 2] = self.crt_time.0.to_le_bytes();
        res[14..16].copy_from_slice(&crt_time);
        let crt_date: [u8; 2] = self.crt_date.0.to_le_bytes();
        res[16..18].copy_from_slice(&crt_date);
        let last_acc_date: [u8; 2] = self.last_acc_date.0.to_le_bytes();
        res[18..20].copy_from_slice(&last_acc_date);
        let fst_cluster: [u8; 4] = self.fst_cluster.to_le_bytes();
        res[20..22].copy_from_slice(&fst_cluster[2..4]);
        res[26..28].copy_from_slice(&fst_cluster[0..2]);
        let wrt_time: [u8; 2] = self.wrt_time.0.to_le_bytes();
        res[22..24].copy_from_slice(&wrt_time);
        let wrt_date: [u8; 2] = self.wrt_date.0.to_le_bytes();
        res[24..26].copy_from_slice(&wrt_date);
        let file_size: [u8; 4] = self.file_size.to_le_bytes();
        res[28..32].copy_from_slice(&file_size);
        res
    }
}

impl DirectoryEntry {
    /// 文件名
    ///
    /// note: 返回的字符串包含空格，比如文件系统中有个文件为 `test`,
    /// 这里返回的 `String` 为 "test    "
    pub fn name(&self) -> String {
        let name = String::from_utf8(self.name.to_vec()).unwrap();
        let mut has_ext = false;
        let mut ext = String::new();
        for c in self.ext_name.iter() {
            ext.push(*c as char);
            if *c != 0x20 {
                has_ext = true;
            }
        }
        match has_ext {
            true => format!("{}.{}", name, ext),
            false => name,
        }
    }

    /// 该目录项曾经使用过，但已经被删除
    pub fn is_deleted(&self) -> bool {
        self.name[0] == 0xe5
    }

    /// 空目录
    ///
    /// The special 0 value, rather than the 0xE5 value, indicates to FAT file system driver code that the
    /// rest of the entries in this directory do not need to be examined because they are all free.
    pub fn is_free(&self) -> bool {
        self.name[0] == 0x00
    }

    /// `.` 目录
    pub fn is_dot(&self) -> bool {
        self.name[0] == '.' as u8 && self.name[1..].iter().all(|c| *c == 0x20)
    }

    /// `..` 目录
    pub fn is_dotdot(&self) -> bool {
        self.name[0] == '.' as u8
            && self.name[1] == '.' as u8
            && self.name[2..].iter().all(|c| *c == 0x20)
    }

    /// 校验和，用于长目录项
    ///
    /// 文档里面计算校验和的伪代码：
    /// ```
    /// unsigned char ChkSum (unsigned char *pFcbName)
    /// {
    ///     short FcbNameLen;
    ///     unsigned char Sum;
    ///     Sum = 0;
    ///     for (FcbNameLen=11; FcbNameLen!=0; FcbNameLen--) {
    ///         // NOTE: The operation is an unsigned char rotate right
    ///         Sum = ((Sum & 1) ? 0x80 : 0) + (Sum >> 1) + *pFcbName++;
    ///     }
    ///     return (Sum);
    /// }
    /// ```
    pub fn checksum(&self) -> u8 {
        let mut sum = 0;
        for c in self.name.iter() {
            let temp = (sum >> 1) + *c;
            if sum.get_bit(0) {
                sum += 0x80;
            }
            sum += temp;
        }
        for c in self.ext_name.iter() {
            let temp = (sum >> 1) + *c;
            if sum.get_bit(0) {
                sum += 0x80;
            }
            sum += temp;
        }
        sum
    }

    /// 获取该目录项占据的块号
    pub async fn clusters(&self, async_block_cache: &Arc<ABC>, fat: &Arc<FAT>) -> Vec<u32> {
        fat.get_link(async_block_cache, self.fst_cluster).await
    }

    /// 读取该目录项占据的块设备数据
    pub async fn load(
        &self,
        async_block_cache: &Arc<ABC>,
        fat: &Arc<FAT>,
        bpb: &Arc<[u8; BLOCK_SIZE]>,
    ) -> Vec<u8> {
        let fst_cluster = self.fst_cluster;
        let clusters_link = fat.get_link(async_block_cache, fst_cluster).await;
        let mut ret = Vec::new();
        for cluster in clusters_link {
            let cluster = cluster_offset_sectors(&**bpb, cluster);
            let block = async_block_cache.read_block(cluster as usize).await;
            block.iter().for_each(|b| ret.push(*b));
        }
        ret
    }
}

/// 长文件名目录项
#[derive(Clone)]
pub struct LongDirectoryEntry {
    pub order: LongOrder,
    pub name1: [u8; 10],
    pub attribute: Attribute,
    _type: u8,
    pub checksum: u8,
    pub name2: [u8; 12],
    pub name3: [u8; 4],
}

impl LongDirectoryEntry {
    /// 名字
    pub fn name(&self) -> Vec<char> {
        let iter = [
            self.name1.iter().as_slice(),
            self.name2.iter().as_slice(),
            self.name3.iter().as_slice(),
        ];
        iter.iter()
            .flat_map(|b| b.iter())
            // 去除掉空字符
            .filter(|c| **c != 0x0 && **c != 0xff)
            .map(|c| *c as char)
            .collect::<Vec<char>>()
    }
    /// 是否是最后一个长文件名目录项
    pub fn is_last(&self) -> bool {
        self.order.is_last()
    }
}

impl From<[u8; 32]> for LongDirectoryEntry {
    fn from(src: [u8; 32]) -> Self {
        let order = LongOrder(src[0]);
        let mut name1 = [0; 10];
        name1.copy_from_slice(&src[1..11]);
        let attribute = Attribute::from(src[11]);
        assert_eq!(attribute, Attribute::ATTR_LONG_NAME);
        let _type = src[12];
        let checksum = src[13];
        let mut name2 = [0; 12];
        name2.copy_from_slice(&src[14..26]);
        let mut name3 = [0; 4];
        name3.copy_from_slice(&src[28..32]);
        Self {
            order,
            name1,
            attribute,
            _type,
            checksum,
            name2,
            name3,
        }
    }
}

impl Into<[u8; 32]> for LongDirectoryEntry {
    fn into(self) -> [u8; 32] {
        let mut res = [0; 32];
        res[0] = self.order.0;
        res[1..11].copy_from_slice(&self.name1);
        res[11] = self.attribute as u8;
        res[12] = self._type;
        res[13] = self.checksum;
        res[14..26].copy_from_slice(&self.name2);
        res[28..32].copy_from_slice(&self.name3);
        res
    }
}

#[derive(Clone)]
pub struct LongOrder(u8);
impl LongOrder {
    pub fn order(&self) -> u8 {
        self.0.get_bits(0..5)
    }

    pub fn is_last(&self) -> bool {
        self.0.get_bit(6)
    }
}

/// Date Format
///
/// 0..4: Day of month, valid value range 1-31 inclusive
/// 5..8: Month of year, 1 = January, valid value range 1-12 inclusive
/// 9..15: Count of years from 1980, valid value range 0-127 inclusive(1980-2107)
#[derive(Clone, Default)]
pub struct Date(u16);
impl Date {
    pub fn day(&self) -> u16 {
        self.0.get_bits(0..5)
    }
    pub fn month(&self) -> u16 {
        self.0.get_bits(5..9)
    }
    pub fn year(&self) -> u16 {
        self.0.get_bits(9..15) + 1980
    }
}

/// Time Format
///
/// 0..4: 2-second count, valid value range 0-29 inclusive(0-58 seconds)
/// 5..10: Minutes, valid value range 0-59 inclusive
/// 11-15: Hours, valid value range 0-23 inclusive
#[derive(Clone, Default)]
pub struct Time(u16);

impl Time {
    pub fn seconds(&self) -> u16 {
        self.0.get_bits(0..5) * 2
    }
    pub fn minutes(&self) -> u16 {
        self.0.get_bits(5..11)
    }
    pub fn hours(&self) -> u16 {
        self.0.get_bits(11..15)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Attribute {
    ATTR_READ_ONLY = 0x01,
    ATTR_HIDDEN = 0x02,
    ATTR_SYSTEM = 0x04,
    ATTR_VOLUME_ID = 0x08,
    /// 标识该目录项对应一个目录而不是文件
    ATTR_DIRECTORY = 0x10,
    ATTR_ARCHIVE = 0x20,
    /// 长文件名目录项
    ATTR_LONG_NAME = 0x0f,
}

impl From<u8> for Attribute {
    fn from(x: u8) -> Self {
        match x {
            0x01 => Self::ATTR_READ_ONLY,
            0x02 => Self::ATTR_HIDDEN,
            0x04 => Self::ATTR_SYSTEM,
            0x08 => Self::ATTR_VOLUME_ID,
            0x10 => Self::ATTR_DIRECTORY,
            0x20 => Self::ATTR_ARCHIVE,
            0x0f => Self::ATTR_LONG_NAME,
            _ => panic!("invalid attirbute value"),
        }
    }
}

impl Default for Attribute {
    fn default() -> Self {
        Self::ATTR_ARCHIVE
    }
}
