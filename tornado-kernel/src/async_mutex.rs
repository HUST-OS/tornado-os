//! RISC-V ISA Mutex Implementation

use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicUsize, Ordering},
    task::Poll,
};

pub struct AsyncMutex<T: ?Sized> {
    state: AtomicUsize,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send + ?Sized> Send for AsyncMutex<T> {}
unsafe impl<T: Send + ?Sized> Sync for AsyncMutex<T> {}

impl<T> AsyncMutex<T> {
    pub const fn new(data: T) -> AsyncMutex<T> {
        AsyncMutex {
            state: AtomicUsize::new(0),
            data: UnsafeCell::new(data),
        }
    }
    pub fn into_inner(self) -> T {
        self.data.into_inner()
    }
}

impl<T: ?Sized> AsyncMutex<T> {
    #[inline]
    pub async fn lock(&self) -> AsyncMutexGuard<'_, T> {
        if let Some(guard) = self.try_lock() {
            return guard;
        }
        todo!()
    }

    fn acquire(&self) -> Poll<()> {
        match self
            .state
            .compare_exchange(0, 1, Ordering::Acquire, Ordering::Acquire)
            .unwrap_or_else(|x| x)
        {
            0 => return Poll::Ready(()), // 成功获得锁
            _ => return Poll::Pending,   // 没获得锁，返回 pending
        }
    }

    #[inline]
    pub fn try_lock(&self) -> Option<AsyncMutexGuard<'_, T>> {
        if self
            .state
            .compare_exchange(0, 1, Ordering::Acquire, Ordering::Acquire)
            .is_ok()
        {
            Some(AsyncMutexGuard(self))
        } else {
            None
        }
    }
}

pub struct AsyncMutexGuard<'a, T: ?Sized>(&'a AsyncMutex<T>);

unsafe impl<T: Send + ?Sized> Send for AsyncMutexGuard<'_, T> {}
unsafe impl<T: Sync + ?Sized> Sync for AsyncMutexGuard<'_, T> {}

impl<'a, T: ?Sized> AsyncMutexGuard<'a, T> {
    pub fn source(guard: &AsyncMutexGuard<'a, T>) -> &'a AsyncMutex<T> {
        guard.0
    }
}
