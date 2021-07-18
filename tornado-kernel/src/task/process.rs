use core::ops::Range;
use alloc::vec::Vec;
use alloc::sync::Arc;
use lazy_static::lazy_static;
use spin::Mutex;
use crate::mm::{AddressSpaceId, FrameBox, VirtPageNum};

/// 进程代表一组使用相同资源、共同承担错误的任务
#[derive(Debug)]
pub struct Process {
    /// 编号，仅为方便管理
    pid: ProcessId,
    /// 相关联的地址空间区域。关闭进程时，进程管理器根据这些信息，删除地址映射关系
    related_address_spaces: Vec<(AddressSpaceId, Range<VirtPageNum>)>,
    /// 关联的帧。关闭进程时，释放这些帧
    frames: Vec<FrameBox>,
}

impl Process {
    /// 创建一个新的进程
    pub fn new(related_address_spaces: Vec<(AddressSpaceId, Range<VirtPageNum>)>, frames: Vec<FrameBox>) -> Arc<Self> {
        let pid = next_process_id();
        Arc::new(Process {
            pid,
            related_address_spaces,
            frames,
        })
    }
}

/// 进程的编号
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProcessId(usize);

lazy_static! {
    pub static ref PROCESS_COUNTER: Mutex<usize> = Mutex::new(1);
}

fn next_process_id() -> ProcessId {
    let mut pid = PROCESS_COUNTER.lock();
    let ans = *pid;
    *pid += 1;
    ProcessId(ans)
}
