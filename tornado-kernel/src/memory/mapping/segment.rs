use crate::memory::address::VirtualAddress;
use crate::memory::mapping::Flags;
use core::ops::Range;

/// 映射的类型
#[derive(Debug)]
pub enum MapType {
    /// 线性映射，操作系统使用
    Linear,
    /// 按帧分配映射
    #[allow(unused)] // 后面给用户上下文用
    Framed,
}

/// 一个需要映射的程序片段
#[derive(Debug)]
pub struct Segment {
    /// 映射类型
    pub map_type: MapType,
    /// 要映射的虚拟地址区间
    pub range: Range<VirtualAddress>,
    /// 权限和标记
    pub flags: Flags,
}

impl Segment {
    
}
