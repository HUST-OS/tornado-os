#![no_std]
#![no_main]
#![feature(asm)]
#![feature(llvm_asm)]

extern crate alloc;
extern crate tornado_user;
use alloc::vec;
use core::future::Future;
use core::task::{Context, Poll};
use core::pin::Pin;
use tornado_user::{
    excutor,
    shared,
    task,
    SHARED_RAW_TABLE,
    exit,
};

#[no_mangle]
fn main() -> ! {
    let mut test_v = vec![1, 2, 3, 4, 5];
    test_v.iter_mut().for_each(|x| *x += 1);
    assert_eq!(test_v, vec![2, 3, 4, 5, 6]);

    let fib = FibonacciFuture::new(6);
    excutor::spawn(fib);
    let ret = excutor::try_join();
    assert_eq!(ret, Some(8));

    // 获取共享运行时的函数表
    let raw_table: extern "C" fn(a0: usize) -> usize = unsafe {
        core::mem::transmute(SHARED_RAW_TABLE)
    };
    let shared_scheduler_ptr = raw_table(0);
    let shared_add_task_ptr = raw_table(1);
    let shared_pop_task_ptr = raw_table(2);
    let shared_scheduler: fn()  -> core::ptr::NonNull<()> = unsafe {
        core::mem::transmute(shared_scheduler_ptr)
    };
    let shared_add_task: unsafe fn(
        shared_scheduler: core::ptr::NonNull<()>, handle: shared::SharedTaskHandle
    ) -> Option<shared::SharedTaskHandle> = unsafe {
        core::mem::transmute(shared_add_task_ptr)
    };
    let shared_pop_task: unsafe fn(
        shared_scheduler: core::ptr::NonNull<()>,
        should_switch: fn(&shared::SharedTaskHandle) -> bool
    ) -> task::TaskResult = unsafe {
        core::mem::transmute(shared_pop_task_ptr)
    };
    let shared_scheduler = shared_scheduler();
    let task = task::UserTask::new(FibonacciFuture::new(6));
    unsafe {
        shared_add_task(shared_scheduler, task.shared_task_handle());
    }
    let ret = shared::run_until_ready(
        || unsafe { shared_pop_task(shared_scheduler, shared::SharedTaskHandle::should_switch) },
        |handle| unsafe { shared_add_task(shared_scheduler, handle) }
    );
    assert_eq!(ret, Some(8));
    // 用户态退出的系统调用
    exit(0);
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