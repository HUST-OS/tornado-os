//! 管理分配给用户程序的内存空间
use crate::memory::{AddressSpaceId, PAGE_SIZE, PhysicalAddress, PhysicalPageNumber};
use alloc::boxed::Box;
use lazy_static::lazy_static;
use async_mutex::AsyncMutex;

const BASE: usize = 0x8700_0000;

lazy_static!(
    pub static ref USER_SPACE: AsyncMutex<UserSpaceManager<20, BASE>> = AsyncMutex::new(UserSpaceManager::new());
);

/// 用户内存管理器
///
/// N: 最大 N 页内存
/// B: 用户空间起始地址
pub struct UserSpaceManager<const N: usize, const B: usize> {
    /// 已用链表
    used: ListNode<AddressSpaceId>, // 头结点是哑结点
    /// 空闲链表
    free: ListNode<AddressSpaceId>, // 头结点是哑结点
    len: usize
}

impl<const N: usize, const B: usize> UserSpaceManager<N, B> {
    pub fn new() -> Self {
        let used = ListNode {
            id: 0,
            val: unsafe { AddressSpaceId::from_raw(0) },
            next: None
        };
        let mut free = used.clone();
        for i in 0..N {
            let prev = free.next.take();
            let node = ListNode {
                id: N - 1 - i,
                val: unsafe { AddressSpaceId::from_raw(0) },
                next: prev
            };
            free.next = Some(Box::new(node));
        }
        Self {used, free, len: 0}

    }
    
    /// 分配一个空间，需要字节大小为 `size`
    ///
    /// 分配的空间对齐为 [`PAGE_SIZE`]
    ///
    /// 分配成功返回起始物理页号和页数
    pub fn alloc(&mut self, size: usize, asid: AddressSpaceId) -> Option<(PhysicalPageNumber, usize)> {
        assert!(PAGE_SIZE % 2 == 0);
        let num = (size + PAGE_SIZE - 1) / PAGE_SIZE; // 需要的页数
        if num > N - self.len {
            None
        } else {
            let base = self.free.next.as_ref().unwrap().id * PAGE_SIZE + B;
            let base = PhysicalPageNumber::floor(PhysicalAddress(base));
            // 更新链表
            for _ in 0..num {
                let mut node = self.free.next.take().unwrap();
                self.free.next =  node.next.take();
                let prev = self.used.next.take();
                node.next = prev;
                self.used.next = Some(node);
            }
            self.len += num;
            Some((base, num))
        }

    }

    // todo: 测试这个函数
    pub fn dealloc(&mut self, asid: AddressSpaceId) -> Option<(PhysicalPageNumber, usize)> {
        let mut prev = &mut self.used;
        loop {
            if prev.next.is_none() { break; }
            if prev.next.as_ref().unwrap().val == asid {
                let mut num = 0;
                let base = prev.next.as_ref().unwrap().id * PAGE_SIZE + B;
                let base = PhysicalPageNumber::floor(PhysicalAddress(base));
                // 更新链表
                while prev.next.as_ref().is_some() && prev.next.as_ref().unwrap().val == asid {
                    let mut node = prev.next.take().unwrap();
                    prev.next = node.next.take();
                    let temp = self.free.next.take();
                    node.next = temp;
                    self.free.next = Some(node);
                    num += 1;
                }
                self.len -= num;
                return Some((base, num))
            } else {
                prev = prev.next.as_mut().unwrap();
            }
        }
        None
    }

}


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T> {
    pub id: usize,
    pub val: T,
    pub next: Option<Box<ListNode<T>>>
}