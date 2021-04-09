#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(asm)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
extern crate alloc;

mod excutor;

use buddy_system_allocator::LockedHeap;
use alloc::vec;
use core::future::Future;
use core::task::{Context, Poll};
use core::pin::Pin;

const USER_HEAP_SIZE: usize = 32768;

static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

#[cfg_attr(not(test), panic_handler)]
pub fn panic_handler(_panic_info: &core::panic::PanicInfo) -> ! {
    // todo: 直接传给系统调用
    unsafe { llvm_asm!("ebreak"); }
    unreachable!()
}

#[cfg_attr(not(test), alloc_error_handler)]
pub fn handle_alloc_error(_layout: core::alloc::Layout) -> ! {
    // todo: 直接传给系统调用
    unsafe { llvm_asm!("ebreak"); }
    unreachable!()
}

#[no_mangle]
#[link_section = ".text.entry"]
#[export_name = "_start"]
fn main() -> ! {
    unsafe {
        asm!(
            "lw t0, (gp)"
        );
    }
    unsafe {
        HEAP.lock().init(HEAP_SPACE.as_ptr() as usize, USER_HEAP_SIZE);
    }
    
    let mut test_v = vec![1, 2, 3, 4, 5];
    test_v.iter_mut().for_each(|x| *x += 1);
    assert_eq!(test_v, vec![2, 3, 4, 5, 6]);

    let fib = FibonacciFuture::new(6);
    excutor::spawn(fib);
    let ret = excutor::try_join();
    assert_eq!(ret, Some(8));
    // todo: 退出进程的系统调用
    unsafe { llvm_asm!("ecall"); }
    unreachable!()
}

struct FibonacciFuture {
    a: usize,
    b: usize,
    i: usize,
    cnt: usize
}

impl FibonacciFuture {
    fn new(cnt: usize) -> FibonacciFuture {
        FibonacciFuture {
            a: 0,
            b: 1,
            i: 0,
            cnt
        }
    }
}

impl Future for FibonacciFuture {
    type Output = usize;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.i == self.cnt {
            Poll::Ready(self.a)
        } else {
            let t = self.a;
            self.a += self.b;
            self.b = t;
            self.i += 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
