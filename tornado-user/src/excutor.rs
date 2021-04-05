#![no_std]
extern crate alloc;
use lazy_static::*;
use {
    alloc::{boxed::Box, collections::vec_deque::VecDeque, sync::Arc},
    core::{
        future::Future,
        pin::Pin,
        task::{Context, Poll},
    },
    spin::Mutex,
    woke::{waker_ref, Woke},
};

#[derive(Default)]
pub struct Executor<T> {
    tasks: Mutex<VecDeque<Arc<Task<T>>>>,
}

pub struct Task<T> {
    pub future: Mutex<Pin<Box<dyn Future<Output = T> + Send + 'static>>>,
    sleeping: Mutex<bool>,
}

impl<T> Woke for Task<T> {
    fn wake_by_ref(task: &Arc<Self>) {
        task.mark_ready();
    }
}

impl<T> Task<T> {
    fn mark_ready(&self) {
        let mut is_sleeping = self.sleeping.lock();
        *is_sleeping = false;
    }

    pub fn mark_sleep(&self) {
        let mut is_sleeping = self.sleeping.lock();
        *is_sleeping = true;
    }

    pub fn is_sleeping(&self) -> bool {
        let is_sleeping = self.sleeping.lock();
        *is_sleeping
    }
}

impl<T> Executor<T> {
    fn append_task(&self, future: Pin<Box<dyn Future<Output = T> + 'static + Send>>) {
        let task = Arc::new(Task {
            future: Mutex::new(future),
            sleeping: Mutex::new(false),
        });
        self.tasks.lock().push_back(task);
    }

    pub fn push_task(&self, task: Arc<Task<T>>) {
        self.tasks.lock().push_back(task);
    }

    pub fn pop_task(&self) -> Option<Arc<Task<T>>> {
        let mut task_v = self.tasks.lock();
        for _ in 0..task_v.len() {
            let task = task_v.pop_front().unwrap();
            if !task.is_sleeping() {
                return Some(task);
            }
            task_v.push_back(task);
        }
        None
    }

    pub fn spawn(&self, future: impl Future<Output = T> + 'static + Send) {
        self.append_task(Box::pin(future));
    }

    pub fn run_until_idle(&self) {
        while let Some(task) = self.pop_task() {
            task.mark_sleep();
            let waker = waker_ref(&task);
            let mut context = Context::from_waker(&*waker);
            let ret = task.future.lock().as_mut().poll(&mut context);
            if let Poll::Pending = ret {
                self.push_task(task);
            }
        }
    }

    pub fn try_get(&self) -> Option<T> {
        while let Some(task) = self.pop_task() {
            task.mark_sleep();
            let waker = waker_ref(&task);
            let mut context = Context::from_waker(&*waker);
            let ret = task.future.lock().as_mut().poll(&mut context);
            if let Poll::Ready(ret) = ret {
                return Some(ret);
            } else {
                self.push_task(task);
            }
        }
        None
    }
}

lazy_static! {
    static ref EXECUTOR: Executor<usize> = Executor::default();
}

/// Give future to global executor to be polled and executed.
pub fn spawn(future: impl Future<Output = usize> + 'static + Send) {
    EXECUTOR.spawn(future);
}

/// Run futures in global executor until there is no runnable task.
pub fn run_until_idle() {
    EXECUTOR.run_until_idle();
}

pub fn try_join() -> Option<usize> {
    EXECUTOR.try_get()
}