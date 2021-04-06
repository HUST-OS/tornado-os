use super::config::{PAGE_SIZE, KERNEL_MAP_OFFSET};
use bit_field::BitField;

/// 物理地址，其实可以比usize要长
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysicalAddress(pub usize);

impl PhysicalAddress {
    /// 从虚拟地址取得某类型的 &mut 引用
    pub unsafe fn deref_linear_static<T>(self) -> &'static mut T {
        self.virtual_address_linear().deref_virtual()
    }
    // 线性映射下，得到物理地址对应的虚拟地址
    pub fn virtual_address_linear(self) -> VirtualAddress {
        let va = self.0.wrapping_add(KERNEL_MAP_OFFSET);
        VirtualAddress(va)
    }
}

impl core::ops::Add<usize> for PhysicalAddress {
    type Output = PhysicalAddress;
    fn add(self, rhs: usize) -> Self::Output {
        // 物理地址和偏移回环相加，得到物理地址
        PhysicalAddress(self.0.wrapping_add(rhs))
    }
}

impl core::ops::AddAssign<usize> for PhysicalAddress {
    fn add_assign(&mut self, rhs: usize) {
        self.0 = self.0.wrapping_add(rhs)
    }
}

impl core::ops::Sub<PhysicalAddress> for PhysicalAddress {
    type Output = usize;
    fn sub(self, rhs: PhysicalAddress) -> Self::Output {
        // 物理地址回环相减，得到地址的偏移
        self.0.wrapping_sub(rhs.0)
    }
}

impl From<PhysicalAddress> for usize {
    fn from(src: PhysicalAddress) -> usize {
        src.0
    }
}

/// 物理页号
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysicalPageNumber(usize);

impl PhysicalPageNumber {
    /// 将地址转换为页号，向下取整
    pub const fn floor(address: PhysicalAddress) -> Self {
        Self(address.0 / PAGE_SIZE)
    }
    /// 将地址转换为页号，向上取整
    pub const fn ceil(address: PhysicalAddress) -> Self {
        Self(address.0 / PAGE_SIZE + (address.0 % PAGE_SIZE != 0) as usize)
    }
    /// 从 satp 寄存器中取出根页表物理页号
    pub fn from_satp(satp: usize) -> Self {
        Self(satp.get_bits(0..44))
    }
    /// 得到物理页的起始地址
    pub fn start_address(&self) -> PhysicalAddress {
        PhysicalAddress(self.0 << 12)
    }
}

impl core::ops::Add<usize> for PhysicalPageNumber {
    type Output = PhysicalPageNumber;
    fn add(self, rhs: usize) -> Self::Output {
        PhysicalPageNumber(self.0 + rhs)
    }
}

impl core::ops::Sub<PhysicalPageNumber> for PhysicalPageNumber {
    type Output = usize;
    fn sub(self, rhs: PhysicalPageNumber) -> Self::Output {
        self.0.wrapping_sub(rhs.0) // todo
    }
}

impl From<PhysicalPageNumber> for usize {
    fn from(src: PhysicalPageNumber) -> usize {
        src.0
    }
}

impl From<usize> for PhysicalPageNumber {
    fn from(src: usize) -> PhysicalPageNumber {
        PhysicalPageNumber(src)
    }
}

/// 虚拟地址，usize恰好能表示虚拟地址
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtualAddress(pub usize);

impl VirtualAddress {
    /// 从虚拟地址取得某类型的 &mut 引用
    pub unsafe fn deref_virtual<T>(self) -> &'static mut T {
        &mut *(self.0 as *mut T)
    }
    // 线性映射下，得到物理地址对应的虚拟地址
    pub fn physical_address_linear(self) -> PhysicalAddress {
        let pa = self.0.wrapping_sub(KERNEL_MAP_OFFSET);
        PhysicalAddress(pa)
    }
}

impl core::ops::Add<usize> for VirtualAddress {
    type Output = VirtualAddress;
    fn add(self, rhs: usize) -> Self::Output {
        // 虚拟地址和偏移回环相加，得到虚拟地址
        VirtualAddress(self.0.wrapping_add(rhs))
    }
}

impl core::ops::AddAssign<usize> for VirtualAddress {
    fn add_assign(&mut self, rhs: usize) {
        self.0 = self.0.wrapping_add(rhs)
    }
}

impl core::ops::Sub<VirtualAddress> for VirtualAddress {
    type Output = usize;
    fn sub(self, rhs: VirtualAddress) -> Self::Output {
        // 虚拟地址回环相减，得到地址的偏移
        self.0.wrapping_sub(rhs.0)
    }
}

impl From<VirtualAddress> for usize {
    fn from(src: VirtualAddress) -> usize {
        src.0
    }
}

/// 虚拟页号
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtualPageNumber(usize);

impl VirtualPageNumber {
    /// 将地址转换为页号，向下取整
    pub const fn floor(address: VirtualAddress) -> Self {
        Self(address.0 / PAGE_SIZE)
    }
    /// 将地址转换为页号，向上取整
    pub const fn ceil(address: VirtualAddress) -> Self {
        Self(address.0 / PAGE_SIZE + (address.0 % PAGE_SIZE != 0) as usize)
    }
    /// 得到虚拟页的起始地址
    pub fn start_address(&self) -> VirtualAddress {
        VirtualAddress(self.0 << 12)
    }
    // 线性映射下，得到虚拟页号对应的物理页号
    pub fn physical_page_number_linear(self) -> PhysicalPageNumber {
        let va = self.start_address();
        let pa = va.physical_address_linear();
        PhysicalPageNumber(pa.0 / PAGE_SIZE)
    }
    /// 对于Sv39，得到一、二、三级页号
    pub fn levels(&self) -> [usize; 3] {
        [
            self.0.get_bits(18..27),
            self.0.get_bits(9..18),
            self.0.get_bits(0..9),
        ]
    }
}

impl core::ops::Add<usize> for VirtualPageNumber {
    type Output = VirtualPageNumber;
    fn add(self, rhs: usize) -> Self::Output {
        VirtualPageNumber(self.0 + rhs)
    }
}

impl core::ops::Sub<VirtualPageNumber> for VirtualPageNumber {
    type Output = usize;
    fn sub(self, rhs: VirtualPageNumber) -> Self::Output {
        self.0.wrapping_sub(rhs.0) // todo
    }
}

impl From<VirtualPageNumber> for usize {
    fn from(src: VirtualPageNumber) -> usize {
        src.0
    }
}

impl From<usize> for VirtualPageNumber {
    fn from(src: usize) -> VirtualPageNumber {
        VirtualPageNumber(src)
    }
}