//! 从文件系统中加载用户程序到内存

use core::ptr::copy;
use alloc::string::String;
use super::space::USER_SPACE;
use crate::fs::FS;
use crate::memory::{
    MemorySet,
    AddressSpaceId,
    PhysicalPageNumber
};

pub async fn load_user<S: Into<String>>(user: S, asid: AddressSpaceId) -> MemorySet {
    let fs = FS.lock().await;
    let fs = unsafe { fs.assume_init_ref() };
    let binary = fs.load_binary(user).await;
    let mut s = USER_SPACE.lock().await;
    let (base, pages) = s.alloc(binary.len(), asid).expect("alloc physical space for user binary");
    let base = base.start_address();
    let base_va = base.virtual_address_linear();
    let dst = base_va.0 as *const () as *mut u8;
    let src = binary.as_ptr();
    // 加载用户二进制程序到
    println!("src: {:x?}, dst: {:x?}, len: {}, base: {:x?}, pages: {}", src, dst, binary.len(), base, pages);
    unsafe {
        copy(src, dst, binary.len());
    }
    MemorySet::new_bin(base.0, pages + 100).expect("create user memory set")
}