use super::config::PAGE_SIZE;

/// Actually can be larger than usize
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysicalAddress(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysicalPageNumber(usize);

impl PhysicalPageNumber {
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
        self.0 - rhs.0
    }
}

impl PhysicalPageNumber {
    /// 将地址转换为页号，向下取整
    pub const fn floor(address: PhysicalAddress) -> Self {
        Self(address.0 / PAGE_SIZE)
    }
    /// 将地址转换为页号，向上取整
    pub const fn ceil(address: PhysicalAddress) -> Self {
        Self(address.0 / PAGE_SIZE + (address.0 % PAGE_SIZE != 0) as usize)
    }
}
