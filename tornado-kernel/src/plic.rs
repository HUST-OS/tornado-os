//! RISC-V 平台级中断控制器逻辑
//!
//! [关于plic](https://github.com/riscv/riscv-plic-spec)
//!
//! [关于context](https://github.com/riscv/riscv-plic-spec/pull/18)
use crate::hart;
use crate::memory::{PLIC_BASE, VIRTIO0_IRQ};
use core::convert::TryFrom;
use plic::{Nr, Plic, Priority};

#[derive(Copy, Clone, Debug)]
#[repr(u16)]
pub enum ExternInterrupt {
    VIRTIO0 = VIRTIO0_IRQ as u16,
}

impl From<ExternInterrupt> for Nr {
    fn from(src: ExternInterrupt) -> Nr {
        Nr::try_from(src as u16).unwrap()
    }
}

pub type PLIC = Plic<PLIC_BASE, 3>;

/// 通过 plic 库来初始化 PLIC，目前有问题，原因未知
pub unsafe fn init() {
    // set desired IRQ priorities non-zero (otherwise disabled).
    PLIC::set_priority(ExternInterrupt::VIRTIO0, Priority::lowest());
    let hart = hart::KernelHartInfo::hart_id();
    // set virtio's enable bit for this hart's S-mode.
    PLIC::unmask(hart * 2 + 1, ExternInterrupt::VIRTIO0);
    // set this hart's S-mode priority threshold to 0.
    PLIC::set_threshold(hart * 2 + 1, Priority::never());
}

pub const PLIC_PENDING: usize = PLIC_BASE.wrapping_add(0x1000);

pub const fn plic_senable(hart: usize) -> usize {
    PLIC_BASE
        .wrapping_add(0x2080)
        .wrapping_add((hart).wrapping_mul(0x100))
}

pub const fn plic_spriority(hart: usize) -> usize {
    PLIC_BASE
        .wrapping_add(0x201000)
        .wrapping_add((hart).wrapping_mul(0x2000))
}

pub const fn plic_sclaim(hart: usize) -> usize {
    PLIC_BASE
        .wrapping_add(0x201004)
        .wrapping_add((hart).wrapping_mul(0x2000))
}

/// xv6 中初始化 PLIC 的方式，暂时先使用这种方法
pub unsafe fn xv6_plic_init() {
    *((PLIC_BASE + VIRTIO0_IRQ * 4) as *mut u32) = 1;
    let hart = hart::KernelHartInfo::hart_id();
    *(plic_senable(hart) as *mut u32) = (1 << VIRTIO0_IRQ) as u32;
    *(plic_spriority(hart) as *mut u32) = 0;
}

/// ask the PLIC what interrupt we should serve.
pub unsafe fn plic_claim() -> u32 {
    let hart = hart::KernelHartInfo::hart_id();
    let irq: u32 = *(plic_sclaim(hart) as *mut u32);
    irq
}

/// tell the PLIC we've served this IRQ.
pub unsafe fn plic_complete(irq: u32) {
    let hart = hart::KernelHartInfo::hart_id();
    *(plic_sclaim(hart) as *mut u32) = irq;
}
