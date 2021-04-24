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
    SHARED_PAYLOAD_BASE,
    shared,
    task,
    exit,
};

#[no_mangle]
fn main() -> ! {
    let mut test_v = vec![1, 2, 3, 4, 5];
    test_v.iter_mut().for_each(|x| *x += 1);
    assert_eq!(test_v, vec![2, 3, 4, 5, 6]);

    let shared_payload = unsafe { shared::SharedPayload::new(SHARED_PAYLOAD_BASE) };
    let task = task::UserTask::new(FibonacciFuture::new(6));
    unsafe {
        /* todo: hart_id, asid */
        shared_payload.add_task(0, tornado_user::shared::AddressSpaceId::from_raw(tornado_user::ADDRESS_SPACE_ID), task.task_repr());
    }
    let ret = shared::run_until_ready(
        || unsafe { shared_payload.peek_task(shared::user_should_switch) },
        |task_repr| unsafe { shared_payload.delete_task(task_repr) }
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