//! 和处理核相关的函数
use core::marker::PhantomData;

/// 上下文指针的借用结构
pub struct ThreadPointer<'a> {
    address: usize,
    _borrowed: PhantomData<&'a ()>,
}

impl<'a> ThreadPointer<'a> {
    /// 写一个指针到上下文指针
    pub unsafe fn write(address: usize) {
        asm!("mv tp, {}", in(reg) address);
    }

    /// 得到借用
    pub unsafe fn as_ref<T>() -> Option<&'a T> {
        (Self::read().address as *const T).as_ref()
    }

    /// 得到可变借用
    pub unsafe fn as_mut<T>() -> Option<&'a mut T> {
        (Self::read().address as *mut T).as_mut()
    }

    #[inline(always)]
    unsafe fn read() -> Self {
        match () {
            #[cfg(riscv)] () => {
                let tp: usize;
                asm!("", lateout("tp") tp);
                ThreadPointer { address: tp, _borrowed: PhantomData }
            },
            #[cfg(not(riscv))] () => unimplemented!(),
        }
        
    }
}
