use crate::memory::{PhysicalPageNumber, VirtualPageNumber};

use super::{page_table::PageTable, page_table_entry::PageTableEntry};
use riscv::register::satp::Mode;
use bit_field::BitField;

/// Satp 寄存器，用于用户陷入内核态的时候提供地址映射帮助
pub struct Satp(usize);

impl Satp {
    pub fn new(satp: usize) -> Self {
        Self(satp)
    }
    pub fn mode(&self) -> Mode {
        match self.0.get_bits(60..64) {
            0 => Mode::Bare,
            8 => Mode::Sv39,
            9 => Mode::Sv48,
            10 => Mode::Sv57,
            11 => Mode::Sv64,
            _ => unreachable!(),
        }
    }
    pub fn asid(&self) -> usize {
        self.0.get_bits(44..60)
    }
    pub fn ppn(&self) -> usize {
        self.0.get_bits(0..44)
    }
    pub fn find_pte(&self, vpn: VirtualPageNumber) -> Option<&mut PageTableEntry> {
        let root_ppn = PhysicalPageNumber::from_satp(self.0);
        let root_table_pa = root_ppn.start_address();
        let root_table: &mut PageTable = unsafe { root_table_pa.deref_linear_static() };
        let mut entry = &mut root_table.entries[vpn.levels()[0]];
        for vpn_i in &vpn.levels()[1..] {
            // 没有页表项或页表项无效
            if entry.is_empty() || !entry.is_valid() {
                return None;
            }
            // 进入下一级页表
            let next_table_pa = entry.start_address();
            let next_table: &mut PageTable = unsafe { next_table_pa.deref_linear_static() };
            entry = &mut next_table.entries[*vpn_i];
        }
        Some(entry)
    }
    pub fn translate(&self, vpn: VirtualPageNumber) -> Option<PhysicalPageNumber> {
        self.find_pte(vpn).map(
            |pte| {pte.page_number()}
        )
    }
    pub fn inner(&self) -> usize {
        self.0
    }
}
