//! 用于内核初始化
// unused
use {
    alloc::{boxed::Box, collections::vec_deque::VecDeque, sync::Arc, vec::Vec},
    core::{
        future::Future,
        pin::Pin,
        task::{Context, Poll},
        usize,
    },
    lazy_static::*,
    spin::Mutex,
    woke::{waker_ref, Woke},
};

/// Task is our unit of execution and holds a future are waiting on
struct Task<T> {
    pub future: Mutex<Pin<Box<dyn Future<Output = T> + Send + 'static>>>,
}

impl<T> Woke for Task<T> {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        todo!()
    }
}

// Block on task
pub fn block_on<T>(future: impl Future<Output = T> + 'static + Send) -> T
// where
//     T: Send + 'static
{
    let task = Arc::new(Task {
        future: Mutex::new(Box::pin(future)),
    });
    let mut future = task.future.lock();
    // create a waker for the task
    let waker = waker_ref(&task);
    let context = &mut Context::from_waker(&*waker);
    loop {
        if let Poll::Ready(val) = future.as_mut().poll(context) {
            return val;
        }
    }
}
