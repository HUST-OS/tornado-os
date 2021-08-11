//! 从文件系统中加载用户程序到内存

use super::space::USER_SPACE;
use crate::{
    fs::FS,
    hart::KernelHartInfo,
    memory::{AddressSpaceId, MemorySet, PhysicalAddress, PhysicalPageNumber},
};
use alloc::string::String;
use core::ptr::copy;

/// 从文件系统中加载一个用户程序到内存，并返回包含映射关系的[`MemorySet`]结构
///
/// note: 调用这个函数之前文件系统必须已经初始化
///
/// # Example:
///
/// ```Rust
/// async {
///     let mm_set = load_user("alloc-test.bin").await;    
/// }
/// ```
pub async fn load_user<S: Into<String>>(user: S) -> MemorySet {
    // 获取一个新的地址空间编号
    let asid = KernelHartInfo::alloc_address_space_id().expect("alloc address space id");
    println!("[kernel] new asid: {:?}", asid);
    let binary = {
        let fs = FS.lock().await;
        let fs = unsafe { fs.assume_init_ref() };
        fs.load_binary(user).await
    };
    let base = {
        let mut s = USER_SPACE.lock().await;
        s.alloc(300, asid)
            .expect("alloc physical space for user binary")
    };
    let base = base.start_address();
    println!("[kernel] asid {:?} user base: {:x?}", asid, base);
    let base_va = base.virtual_address_linear();
    let dst = base_va.0 as *const () as *mut u8;
    let src = binary.as_ptr();
    // 加载用户二进制程序到
    unsafe {
        copy(src, dst, binary.len());
    }
    MemorySet::new_bin(base.0, 400, asid).expect("create user memory set")
}
