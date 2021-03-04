//! 和处理核相关的函数


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
