//! 从文件系统中加载用户程序到内存

use super::space::USER_SPACE;
use crate::fs::FS;
use crate::hart::KernelHartInfo;
use crate::memory::{AddressSpaceId, MemorySet, PhysicalPageNumber};
use alloc::string::String;
use core::ptr::copy;

pub async fn load_user<S: Into<String>>(user: S) -> MemorySet {
    // 获取一个新的地址空间编号
    let asid = KernelHartInfo::alloc_address_space_id().expect("alloc address space id");
    let binary = {
        let fs = FS.lock().await;
        let fs = unsafe { fs.assume_init_ref() };
        fs.load_binary(user).await
    };
    let (base, pages) = {
        let mut s = USER_SPACE.lock().await;
        s.alloc(binary.len(), asid)
            .expect("alloc physical space for user binary")
    };

    let base = base.start_address();
    let base_va = base.virtual_address_linear();
    let dst = base_va.0 as *const () as *mut u8;
    let src = binary.as_ptr();
    // 加载用户二进制程序到
    unsafe {
        copy(src, dst, binary.len());
    }
    MemorySet::new_bin(base.0, pages + 100, asid).expect("create user memory set")
}
