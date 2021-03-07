//! 和处理核相关的函数
use core::ops::Add;

use alloc::boxed::Box;

use crate::memory::AddressSpaceId;

/// 写一个指针到上下文指针
pub unsafe fn write_tp(value: usize) {
    asm!("mv tp, {}", in(reg) value);
}

pub fn read_tp() -> usize {
    match () {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        () => {
            let value: usize;
            // asm!("", lateout("tp") value); // bug: rust-lang/rust#82753
            unsafe { llvm_asm!("":"={tp}"(value)) };
            value
        },
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        () => unimplemented!(),
    }
}

// 用户层将定义自己的tp寄存器意义
// 在内核层中，tp指向一个结构体，说明当前的硬件线程编号，以及已经分配的地址空间
pub struct KernelHartInfo {
    hart_id: usize,
    address_space_id: AddressSpaceId,
}

impl KernelHartInfo {
    /// 准备一个新的核，以供调度器使用
    pub unsafe fn load_hart(hart_id: usize) {
        let hart_info = Box::new(KernelHartInfo {
            hart_id,
            address_space_id: AddressSpaceId::kernel(),
        });
        let tp = Box::into_raw(hart_info) as usize; // todo: 这里有内存泄漏，要在drop里处理
        write_tp(tp)
    }

    /// 热加载/热卸载处理核，释放这个核占用的内存资源
    pub unsafe fn unload_hart() {
        let addr = read_tp();
        let bx: Box<KernelHartInfo> = Box::from_raw(addr as *mut _);
        drop(bx);
    }

    /// 得到当前硬件线程的编号，必须在load_hart之后使用
    pub fn hart_id() -> usize {
        let addr = read_tp();
        let bx: Box<KernelHartInfo> = unsafe { Box::from_raw(addr as *mut _) };
        let ans = bx.hart_id;
        drop(Box::into_raw(bx));
        ans
    }  

    pub unsafe fn load_address_space_id(asid: AddressSpaceId) {
        let addr = read_tp();
        let mut bx: Box<KernelHartInfo> = Box::from_raw(addr as *mut _);
        bx.address_space_id = asid;
        drop(Box::into_raw(bx)); // 防止Box指向的内存被释放
    }

    /// 得到当前的地址空间编号
    pub fn current_address_space_id() -> AddressSpaceId {
        let addr = read_tp();
        let bx: Box<KernelHartInfo> = unsafe { Box::from_raw(addr as *mut _) };
        let ans = bx.address_space_id;
        drop(Box::into_raw(bx));
        ans
    }
}
