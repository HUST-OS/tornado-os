#![no_std]
#![no_main]
#![feature(asm)]
#![feature(llvm_asm)]

extern crate alloc;
#[macro_use]
extern crate tornado_user;
use core::future::Future;
use core::task::{Context, Poll};
use core::pin::Pin;

async fn async_main() -> i32 {
    // todo: 唤醒逻辑
    tornado_user::spawn(async {
        let ans = FibonacciFuture::new(5).await;
        println!("[User] Fibonacci[5] = {}", ans);
    });
    let ans = FibonacciFuture::new(6).await;
    println!("[User] Fibonacci[6] = {}", ans);
    0
}

// 异步main函数，由entry调用execute_async_main
#[no_mangle]
fn main() -> i32 {
    tornado_user::execute_async_main(async_main())
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
            println!("Fibonacci {} result: {}", self.cnt, self.a);
            Poll::Ready(self.a)
        } else {
            let t = self.a;
            self.a += self.b;
            self.b = t;
            self.i += 1;
            println!("Fibonacci {}: i = {}, a = {}, b = {}", self.cnt, self.i, self.a, self.b);
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
