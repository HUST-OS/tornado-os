// Ref: crate Executor

use alloc::collections::VecDeque;
use alloc::sync::Arc;
use alloc::boxed::Box;
use core::future::Future;
use core::pin::Pin;
use core::task::{Poll, Context};

use spin::Mutex;
use woke::{waker_ref, Woke};

pub struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>,
    state: Mutex<bool>,
}

impl Task {
    fn mark_ready(&self) {
        let mut value = self.state.lock();
        *value = true;
    }

    pub fn is_sleeping(&self) -> bool {
        let value = self.state.lock();
        !(*value)
    }

    pub fn mark_sleep(&self) {
        let mut value = self.state.lock();
        *value = false;
    }
}

impl Woke for Task {
    fn wake_by_ref(task: &Arc<Self>) {
        task.mark_ready();
    }
}

#[derive(Default)]
pub struct Executor {
    tasks: Mutex<VecDeque<Arc<Task>>>,
}

impl Executor {
    fn add_task(&self, future: Pin<Box<dyn Future<Output = ()> + Send + 'static>>) {
        let task = Arc::new(Task {
            future: Mutex::new(future), // todo: rewrite using UnsafeCell
            state: Mutex::new(true),
        });
        self.tasks.lock().push_back(task);
    }

    pub fn push_task(&self, task: Arc<Task>) {
        self.tasks.lock().push_back(task);
    }

    pub fn pop_task(&self) -> Option<Arc<Task>> {
        let mut tasks = self.tasks.lock();
        for _ in 0..tasks.len() {
            let task = tasks.pop_front().unwrap();
            if !task.is_sleeping() {
                return Some(task);
            }
            tasks.push_back(task);
        }
        None
    }

    // Give future to be polled and executed
    pub fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        self.add_task(Box::pin(future));
    }

    /// Run futures until there is no runnable task.
    pub fn run_until_idle(&self) {
        while let Some(task) = self.pop_task() {
            task.mark_sleep();
            // make a waker for our task
            let waker = waker_ref(&task);
            // poll our future and give it a waker
            let mut context = Context::from_waker(&*waker);
            let ret = task.future.lock().as_mut().poll(&mut context);
            if let Poll::Pending = ret {
                self.push_task(task);
            }
        }
    }
}
