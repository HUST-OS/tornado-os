//! Boot Sector and BPB Structure
//!
//! 这是 `FAT32` 文件系统的第一个扇区，只从块设备中读取一次保存到内存
//!
//! 照着文档写的: https://d1.amobbs.com/bbs_upload782111/files_7/armok01151038.pdf
//! (写完才发现有中文文档)
use crate::fat::FAT;
use alloc::string::String;
use core::convert::TryInto;

/// Boot Sector 各字段的偏移
enum BootSectorOffset {
    /// Jump instruction to boot code
    JmpBoot = 0,
    /// OEM name
    OEMName = 3,
    /// Drive number
    DrvNum = 64,
    /// Reserved
    Reserved1 = 65,
    /// Extended boot signature
    BootSig = 66,
    /// Volume serial number
    VolID = 67,
    /// Volume label
    VolLab = 71,
    /// One of the strings "FAT12", "FAT16", "FAT32"
    FilSysType = 82,
}

impl BootSectorOffset {
    pub fn jmp_boot(sector: &[u8]) -> u32 {
        u32::from_le_bytes(
            Self::split(Self::JmpBoot, Self::OEMName, sector)
                .try_into()
                .unwrap(),
        )
    }
    pub fn oem_name(sector: &[u8]) -> String {
        String::from_utf8(Self::split(Self::OEMName, Self::DrvNum, sector).to_vec()).unwrap()
    }
    pub fn drv_num(sector: &[u8]) -> u8 {
        u8::from_le_bytes(
            Self::split(Self::DrvNum, Self::Reserved1, sector)
                .try_into()
                .unwrap(),
        )
    }
    pub fn boot_sig(sector: &[u8]) -> u8 {
        u8::from_le_bytes(
            Self::split(Self::BootSig, Self::VolID, sector)
                .try_into()
                .unwrap(),
        )
    }
    pub fn vol_id(sector: &[u8]) -> u32 {
        u32::from_le_bytes(
            Self::split(Self::VolID, Self::VolLab, sector)
                .try_into()
                .unwrap(),
        )
    }
    fn split(start: Self, end: Self, buf: &[u8]) -> &[u8] {
        &buf[start as usize..end as usize]
    }
}

/// BPB 各字段的偏移
enum BPBOffset {
    /// Count of bytes per sector
    /// This value may take on only the following values: 512, 1024, 2048 or 4096
    BytsPerSec = 11,
    /// Number of sectors per allocation unit
    /// The legal values are 1, 2, 4, 8
    SecPerClus = 13,
    /// Number of reserved sectors in the Reserved region of the volume
    RsvdSecCnt = 14,
    /// The count of FAT data structures on the volume
    NumFATs = 16,
    /// For FAT32 volumes, this field must be set to 0
    RootEntCnt = 17,
    /// Old 16-bit total count of sectors on the volume
    TotSec16 = 19,
    /// ignored
    Media = 21,
    /// On FAT32 volumes this field mut be 0
    FATSz16 = 22,
    /// ignored
    SecPerTrk = 24,
    /// ignored
    NumHeads = 26,
    /// Count of hidden sectors preceding the partition that contains this FAT volume
    HiddSec = 28,
    /// The new 32-bit total count of sectors on the volume
    TotSec32 = 32,
    /// FAT32 32-bit count of sectors occupied by ONE FAT
    FATSz32 = 36,
    /// Extern Flags
    ExtFlags = 40,
    /// High bype is major revision number.
    /// Low byte is minor revision number.
    FSVer = 42,
    /// The cluster number of the first cluster of the root directory
    /// Usually 2 but not required to be 2.
    RootClus = 44,
    /// Sector number of FSINFO structure in the reserved area of the FAT32 volume
    /// Usually 1
    FSInfo = 48,
    /// ignored
    BkBootSec = 50,
    /// Reserved
    Reserved = 52,
}

impl BPBOffset {
    /// 每个扇区的字节数
    pub fn bytes_per_sector(sector: &[u8]) -> u16 {
        u16::from_le_bytes(
            Self::split(Self::BytsPerSec, Self::SecPerClus, sector)
                .try_into()
                .unwrap(),
        )
    }
    /// 每个块对应的扇区数
    pub fn sector_per_cluster(sector: &[u8]) -> u8 {
        u8::from_le_bytes(
            Self::split(Self::SecPerClus, Self::RsvdSecCnt, sector)
                .try_into()
                .unwrap(),
        )
    }
    /// 保留扇区数
    pub fn reserved_sector_number(sector: &[u8]) -> u16 {
        u16::from_le_bytes(
            Self::split(Self::RsvdSecCnt, Self::NumFATs, sector)
                .try_into()
                .unwrap(),
        )
    }
    /// `FAT` 表数目
    pub fn fats_number(sector: &[u8]) -> u8 {
        u8::from_le_bytes(
            Self::split(Self::NumFATs, Self::RootEntCnt, sector)
                .try_into()
                .unwrap(),
        )
    }
    /// 根目录的目录项数量，对于 `FAT32` 来说，这个域必须是 0
    pub fn root_entry_count(sector: &[u8]) -> u16 {
        u16::from_le_bytes(
            Self::split(Self::RootEntCnt, Self::TotSec16, sector)
                .try_into()
                .unwrap(),
        )
    }
    /// 该分区的总扇区数，该域可以是 0，如果该域是 0，`BPB_TotSec32` 域必须非零
    pub fn total_sector_16(sector: &[u8]) -> u16 {
        u16::from_le_bytes(
            Self::split(Self::TotSec16, Self::Media, sector)
                .try_into()
                .unwrap(),
        )
    }
    /// 在 `FAT32` 上必须为 0
    pub fn fat_size_16(sector: &[u8]) -> u16 {
        u16::from_le_bytes(
            Self::split(Self::FATSz16, Self::SecPerTrk, sector)
                .try_into()
                .unwrap(),
        )
    }
    /// 隐藏分区数
    pub fn hidden_sector(sector: &[u8]) -> u32 {
        u32::from_le_bytes(
            Self::split(Self::HiddSec, Self::TotSec32, sector)
                .try_into()
                .unwrap(),
        )
    }
    /// 该分区的总扇区数，如果 `BPB_TotSec16` 域为零，该域必须是非零
    pub fn total_sector_32(sector: &[u8]) -> u32 {
        u32::from_le_bytes(
            Self::split(Self::TotSec32, Self::FATSz32, sector)
                .try_into()
                .unwrap(),
        )
    }
    /// 一个 `FAT` 表包含的扇区数
    pub fn fat_size_32(sector: &[u8]) -> u32 {
        u32::from_le_bytes(
            Self::split(Self::FATSz32, Self::ExtFlags, sector)
                .try_into()
                .unwrap(),
        )
    }
    pub fn extern_flags(sector: &[u8]) -> u16 {
        u16::from_le_bytes(
            Self::split(Self::ExtFlags, Self::FSVer, sector)
                .try_into()
                .unwrap(),
        )
    }
    pub fn root_cluster(sector: &[u8]) -> u32 {
        u32::from_le_bytes(
            Self::split(Self::RootClus, Self::FSInfo, sector)
                .try_into()
                .unwrap(),
        )
    }
    pub fn fs_info(sector: &[u8]) -> u16 {
        u16::from_le_bytes(
            Self::split(Self::FSInfo, Self::BkBootSec, sector)
                .try_into()
                .unwrap(),
        )
    }
    fn split(start: Self, end: Self, sector: &[u8]) -> &[u8] {
        &sector[start as usize..end as usize]
    }
}

/// 获取 `FAT` 表所占的分区数
pub(crate) fn fat_size(sector0: &[u8]) -> u32 {
    match BPBOffset::fat_size_16(sector0) {
        0 => {
            let size = BPBOffset::fat_size_32(sector0);
            assert!(size != 0);
            size
        }
        size => size as u32,
    }
}

/// 根据块号获取字节偏移量
pub(crate) fn cluster_offset_bytes(sector0: &[u8], cluster: u32) -> usize {
    let fat_size = fat_size(sector0) as usize;
    (BPBOffset::reserved_sector_number(sector0) as usize
        + BPBOffset::hidden_sector(sector0) as usize
        + BPBOffset::fats_number(sector0) as usize * fat_size
        + (cluster as usize - 2) * BPBOffset::sector_per_cluster(sector0) as usize)
        * BPBOffset::bytes_per_sector(sector0) as usize
}

/// 根据块号获取扇区偏移量
pub(crate) fn cluster_offset_sectors(sector0: &[u8], cluster: u32) -> u32 {
    let fat_size = fat_size(sector0);
    BPBOffset::reserved_sector_number(sector0) as u32
        + BPBOffset::hidden_sector(sector0)
        + BPBOffset::fats_number(sector0) as u32 * fat_size
        + (cluster - 2) * BPBOffset::sector_per_cluster(sector0) as u32
}

/// 获取 `FAT1` 的字节偏移量
pub(crate) fn fat1_offset_bytes(sector0: &[u8]) -> usize {
    (BPBOffset::reserved_sector_number(sector0) as usize
        + BPBOffset::hidden_sector(sector0) as usize)
        * BPBOffset::bytes_per_sector(sector0) as usize
}

/// 获取 `FAT1` 的扇区偏移量
pub(crate) fn fat1_offset_sectors(sector0: &[u8]) -> u32 {
    BPBOffset::reserved_sector_number(sector0) as u32 + BPBOffset::hidden_sector(sector0)
}

/// 由该分区的第一个扇区来生成 [`FAT`] 数据结构
pub(crate) fn fat1(sector0: &[u8]) -> FAT {
    let fat_nums = BPBOffset::fats_number(sector0);
    let fat_size = fat_size(sector0);
    let base = fat1_offset_sectors(sector0);
    let bytes_per_sector = BPBOffset::bytes_per_sector(sector0);
    FAT {
        fat_nums,
        fat_size,
        base,
        bytes_per_sector,
    }
}
