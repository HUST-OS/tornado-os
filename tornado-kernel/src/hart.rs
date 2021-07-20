//! 和处理核相关的函数
use crate::mm;
use crate::task::Process;
use alloc::boxed::Box;
use alloc::collections::LinkedList;
use alloc::sync::Arc;
use alloc::vec::Vec;
use hashbrown::HashMap;

/// 写一个指针到上下文指针
#[inline]
pub unsafe fn write_tp(tp: usize) {
    asm!("mv tp, {}", in(reg) tp, options(nostack));
}
//
/// 从tp寄存器读上下文指针
#[inline]
pub fn read_tp() -> usize {
    let tp: usize;
    unsafe {
        asm!("mv {}, tp", out(reg) tp, options(nomem, nostack));
    }; // rust-lang/rust#82753 Thank you @Amanieu :)
    tp
}

// 用户层将定义自己的tp寄存器意义
// 在内核层中，tp指向一个结构体，说明当前的硬件线程编号，以及已经分配的地址空间
pub struct KernelHartInfo {
    hart_id: usize,
    current_address_space_id: mm::AddressSpaceId,
    current_process: Option<Arc<Process>>,
    asid_alloc: mm::StackAsidAllocator,
    addr_spaces: HashMap<mm::AddressSpaceId, Arc<spin::Mutex<mm::PagedAddrSpace<mm::Sv39>>>>,
    frame_boxes: HashMap<mm::AddressSpaceId, Vec<mm::FrameBox>>,
}

impl KernelHartInfo {
    /// 准备一个新的核，以供调度器使用
    pub unsafe fn load_hart(hart_id: usize) {
        let hart_info = Box::new(KernelHartInfo {
            hart_id,
            current_address_space_id: mm::DEFAULT_ASID,
            current_process: None,
            asid_alloc: mm::StackAsidAllocator::new(mm::max_asid()),
            addr_spaces: HashMap::new(),
            frame_boxes: HashMap::new(),
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
        use_tp_box(|b| b.hart_id)
    }

    pub unsafe fn load_address_space_id(asid: mm::AddressSpaceId) {
        use_tp_box(|b| b.current_address_space_id = asid);
    }

    /// 得到当前的地址空间编号
    pub fn current_address_space_id() -> mm::AddressSpaceId {
        use_tp_box(|b| b.current_address_space_id)
    }

    pub unsafe fn load_process(process: Arc<Process>) {
        use_tp_box(|b| b.current_process = Some(process.clone()));
    }

    pub fn current_process() -> Option<Arc<Process>> {
        use_tp_box(|b| b.current_process.clone())
    }

    /// 分配一个地址空间编号
    pub fn alloc_address_space_id() -> Result<mm::AddressSpaceId, mm::AsidAllocError> {
        use_tp_box(|b| {
            b.asid_alloc.allocate_asid()
        })
    }

    /// 释放地址空间编号
    pub fn free_address_space_id(asid: mm::AddressSpaceId) {
        use_tp_box(|b| {
            b.asid_alloc.deallocate_asid(asid)
        });
    }

    /// 添加地址空间编号和 satp 寄存器的对应关系
    pub fn create_address_space(asid: mm::AddressSpaceId, addr_space: mm::PagedAddrSpace<mm::Sv39>) {
        // todo: 需要判断是否地址空间编号已经存在
        use_tp_box(move |b| {
            b.addr_spaces.insert(asid, Arc::new(spin::Mutex::new(addr_space)));
        })
    }

    pub fn insert_frame_box(asid: mm::AddressSpaceId, frames: &mut Vec<mm::FrameBox>) {
        use_tp_box(move |b| {
            b.frame_boxes.entry(asid).or_default()
                .append(frames)
        })
    }

    pub fn delete_frame_box(asid: mm::AddressSpaceId) {
        use_tp_box(|b| {
            b.frame_boxes.remove(&asid);
        })
    }

    /// 根据地址空间编号获得 satp 寄存器
    pub unsafe fn get_address_space(asid: mm::AddressSpaceId) -> Option<Arc<spin::Mutex<mm::PagedAddrSpace<mm::Sv39>>>> {
        use_tp_box(|b| {
            let v = &mut b.addr_spaces;
            if let Some(x) = v.get(&asid) {
                return Some(Arc::clone(x));
            }
            return None;
        })
    }
}

#[inline]
fn use_tp_box<F: FnOnce(&mut Box<KernelHartInfo>) -> T, T>(f: F) -> T {
    let addr = read_tp();
    let mut bx: Box<KernelHartInfo> = unsafe { Box::from_raw(addr as *mut _) };
    let ans = f(&mut bx);
    drop(Box::into_raw(bx)); // 防止Box指向的空间被释放
    ans
}
