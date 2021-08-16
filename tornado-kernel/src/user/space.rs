//! 管理分配给用户程序的内存空间
use crate::memory::{AddressSpaceId, PhysicalAddress, PhysicalPageNumber, PAGE_SIZE};
use alloc::boxed::Box;
use async_mutex::AsyncMutex;
use lazy_static::lazy_static;

/// 分配给用户程序的物理空间的起始地址
#[cfg(feature = "qemu")]
const BASE: usize = 0x8400_0000;
#[cfg(feature = "k210")]
const BASE: usize = 0x8050_0000;

lazy_static! {
    pub static ref USER_SPACE: AsyncMutex<UserSpaceManager<2000, BASE>> =
        AsyncMutex::new(UserSpaceManager::new());
}

/// 用户内存管理器
///
/// 常量泛型参数N: 最大N页内存
/// 常量泛型参数B: 用户空间起始地址
pub struct UserSpaceManager<const N: usize, const B: usize> {
    /// 已用链表
    used: ListNode<AddressSpaceId>, // 头结点是哑结点
    /// 空闲链表
    free: ListNode<AddressSpaceId>, // 头结点是哑结点
    /// 长度
    len: usize,
}

impl<const N: usize, const B: usize> UserSpaceManager<N, B> {
    /// 创建一个空的[`UserSpaceManager`]
    pub fn new() -> Self {
        let used = ListNode {
            id: 0,
            val: unsafe { AddressSpaceId::from_raw(0) },
            next: None,
        };
        let mut free = used.clone();
        for i in 0..N {
            let prev = free.next.take();
            let node = ListNode {
                id: N - 1 - i,
                val: unsafe { AddressSpaceId::from_raw(0) },
                next: prev,
            };
            free.next = Some(Box::new(node));
        }
        Self { used, free, len: 0 }
    }

    /// 分配一个空间，需要物理页的数量为 `pages`
    ///
    /// 分配成功返回起始物理页号
    ///
    /// # Example:
    ///
    /// ```
    /// let mut user_space = UserSpaceManager::new();
    ///
    /// // 分配100帧物理内存给地址空间1
    /// let asid = unsafe { AddressSpaceId::from_raw(1) };
    /// let ppn = user_space.alloc(100, asid).expect("alloc user space");
    ///
    /// println!("alloc ppn {:?}", ppn);
    /// ```
    pub fn alloc(&mut self, pages: usize, _asid: AddressSpaceId) -> Option<PhysicalPageNumber> {
        assert!(PAGE_SIZE % 2 == 0);
        if pages > N - self.len {
            None
        } else {
            let base = self.free.next.as_ref().unwrap().id * PAGE_SIZE + B;
            let base = PhysicalPageNumber::floor(PhysicalAddress(base));
            // 更新链表
            for _ in 0..pages {
                let mut node = self.free.next.take().unwrap();
                self.free.next = node.next.take();
                let prev = self.used.next.take();
                node.next = prev;
                self.used.next = Some(node);
            }
            self.len += pages;
            Some(base)
        }
    }

    /// 释放一个地址空间占用的物理内存
    ///
    /// note: 还没测试
    #[allow(unused)]
    pub fn dealloc(&mut self, asid: AddressSpaceId) -> Option<(PhysicalPageNumber, usize)> {
        let mut prev = &mut self.used;
        loop {
            if prev.next.is_none() {
                break;
            }
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
                return Some((base, num));
            } else {
                prev = prev.next.as_mut().unwrap();
            }
        }
        None
    }
}

/// 链表结点
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T> {
    pub id: usize,
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}
