//! Async Mutex Implementation
//! ref: https://github.com/smol-rs/async-lock/blob/master/src/mutex.rs
use super::event::Event;
use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicUsize, Ordering},
};

/// 一个异步锁实现
///
/// # Examples
///
/// ```
/// use async_mutex::AsyncMutex;
///
/// let m = AsyncMutex::new(1);
///
/// let mut guard = m.lock().await;
/// *guard = 2;
///
/// assert!(m.try_lock().is_none());
/// drop(guard);
/// assert_eq!(*m.try_lock().unwrap(), 2);
/// ```
pub struct AsyncMutex<T: ?Sized> {
    /// 锁的当前状态
    ///
    /// 如果锁被锁住，最低有效位被置为 1
    /// 其他位保存锁请求操作的数量
    state: AtomicUsize,
    
    /// 等待锁被释放的监听行为
    lock_ops: Event,
    
    /// 锁的内部数据
    data: UnsafeCell<T>,
}

unsafe impl<T: Send + ?Sized> Send for AsyncMutex<T> {}
unsafe impl<T: Send + ?Sized> Sync for AsyncMutex<T> {}

impl<T> AsyncMutex<T> {
    /// 创建一个新的异步锁
    ///
    /// # Examples
    ///
    /// ```
    /// use async_mutex::AsyncMutex;
    ///
    /// let mutex = AsyncMutex::new(0);
    /// ```
    pub const fn new(data: T) -> AsyncMutex<T> {
        AsyncMutex {
            state: AtomicUsize::new(0),
            lock_ops: Event::new(),
            data: UnsafeCell::new(data),
        }
    }

    /// 消费锁的所有权，返回内部数据
    ///
    /// # Examples
    ///
    /// ```
    /// use async_mutex::AsyncMutex;
    ///
    /// let mutex = AsyncMutex::new(10);
    /// assert_eq!(mutex.into_inner(), 10);
    /// ```
    pub fn into_inner(self) -> T {
        self.data.into_inner()
    }
}

impl<T: ?Sized> AsyncMutex<T> {
    /// 异步方式获取锁
    ///
    /// 返回一个 `guard`，生命周期尽头的时候释放锁
    ///
    /// # Examples
    ///
    /// ```
    /// use async_mutex::AsyncMutex;
    ///
    /// let mutex = AsyncMutex::new(10);
    /// let guard = mutex.lock().await;
    /// assert_eq!(*guard, 10);
    /// ```
    #[inline]
    pub async fn lock(&self) -> AsyncMutexGuard<'_, T> {
        if let Some(guard) = self.try_lock() {
            return guard;
        }
        self.acquire_slow().await;
        AsyncMutexGuard(self)
    }

    #[cold]
    async fn acquire_slow(&self) {
        loop {
            // 开始监听事件
            let listener = self.lock_ops.listen();

            // 如果锁没被任何任务持有，则尝试获取锁
            match self
                .state
                .compare_exchange(0, 1, Ordering::Acquire, Ordering::Acquire)
                .unwrap_or_else(|x| x)
            {
                // 成功获取锁！
                0 => return,

                1 => {}

                _ => break,
            }

            // 等待锁被释放
            listener.await;

            match self
                .state
                .compare_exchange(0, 1, Ordering::Acquire, Ordering::Acquire)
                .unwrap_or_else(|x| x)
            {
                0 => return,

                1 => {}

                _ => {
                    self.lock_ops.notify(1);
                    break;
                }
            }
        }

        if self.state.fetch_add(2, Ordering::Release) > usize::MAX / 2 {
            panic!("In case of potential overflow, abort.");
        }

        let _call = CallOnDrop(|| {
            self.state.fetch_sub(2, Ordering::Release);
        });

        loop {
            let listener = self.lock_ops.listen();

            match self
                .state
                .compare_exchange(2, 2 | 1, Ordering::Acquire, Ordering::Acquire)
                .unwrap_or_else(|x| x)
            {
                2 => return,

                s if s % 2 == 1 => {}

                _ => {
                    self.lock_ops.notify(1);
                }
            }

            listener.await;

            if self.state.fetch_or(1, Ordering::Acquire) % 2 == 0 {
                return;
            }
        }
    }

    /// 尝试获取锁
    ///
    /// 如果获取锁失败，返回 [`None`]，如果获取成功，返回 [`Some(guard)`]
    ///
    /// # Examples
    ///
    /// ```
    /// use async_mutex::AsyncMutex;
    ///
    /// let mutex = AsyncMutex::new(10);
    /// if let Some(guard) = mutex.try_lock() {
    ///     assert_eq!(*guard, 10);
    /// }
    /// ```
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

    /// 返回内部数据的可变引用
    ///
    /// Since this call borrows the mutex mutably, no actual locking takes place -- the mutable
    /// borrow statically guarantees the mutex is not already acquired.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_mutex::AsyncMutex;
    ///
    /// let mut mutex = AsyncMutex::new(0);
    /// *mutex.get_mut() = 10;
    /// assert_eq!(*mutex.lock().await, 10);
    /// ```
    pub fn get_mut(&mut self) -> &mut T {
        unsafe { &mut *self.data.get() }
    }
}

impl<T: core::fmt::Debug + ?Sized> core::fmt::Debug for AsyncMutex<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        struct Locked;
        impl core::fmt::Debug for Locked {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("<locked>")
            }
        }

        match self.try_lock() {
            None => f.debug_struct("AsyncMutex").field("data", &Locked).finish(),
            Some(guard) => f
                .debug_struct("AsyncMutex")
                .field("data", &&*guard)
                .finish(),
        }
    }
}

impl<T> From<T> for AsyncMutex<T> {
    fn from(val: T) -> AsyncMutex<T> {
        AsyncMutex::new(val)
    }
}

impl<T: Default + ?Sized> Default for AsyncMutex<T> {
    fn default() -> AsyncMutex<T> {
        AsyncMutex::new(Default::default())
    }
}

pub struct AsyncMutexGuard<'a, T: ?Sized>(&'a AsyncMutex<T>);

unsafe impl<T: Send + ?Sized> Send for AsyncMutexGuard<'_, T> {}
unsafe impl<T: Sync + ?Sized> Sync for AsyncMutexGuard<'_, T> {}

impl<'a, T: ?Sized> AsyncMutexGuard<'a, T> {
    /// 返回内部锁的引用
    ///
    /// # Examples
    ///
    /// ```
    /// use async_mutex::{AsyncMutex, AsyncMutexGuard};
    ///
    /// let mutex = AsyncMutex::new(10i32);
    /// let guard = mutex.lock().await;
    /// dbg!(AsyncMutexGuard::source(&guard));
    /// ```
    pub fn source(guard: &AsyncMutexGuard<'a, T>) -> &'a AsyncMutex<T> {
        guard.0
    }
}

impl<T: ?Sized> Drop for AsyncMutexGuard<'_, T> {
    fn drop(&mut self) {
        self.0.state.fetch_sub(1, Ordering::Release);
        self.0.lock_ops.notify(1);
    }
}

impl<T: core::fmt::Debug + ?Sized> core::fmt::Debug for AsyncMutexGuard<'_, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&**self, f)
    }
}

impl<T: core::fmt::Display + ?Sized> core::fmt::Display for AsyncMutexGuard<'_, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        (**self).fmt(f)
    }
}

impl<T: ?Sized> Deref for AsyncMutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.0.data.get() }
    }
}

impl<T: ?Sized> DerefMut for AsyncMutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.0.data.get() }
    }
}

/// Calls a function when dropped.
struct CallOnDrop<F: Fn()>(F);

impl<F: Fn()> Drop for CallOnDrop<F> {
    fn drop(&mut self) {
        (self.0)();
    }
}

// Simple test for async mutex
use alloc::sync::Arc;
pub async fn async_mutex_test0<T>(mutex: Arc<AsyncMutex<T>>, event: Arc<Event>) {
    let listener = event.listen();
    println!("[async_mutex_test0]: try acquire mutex!");
    let _s = mutex.lock().await;
    println!("[async_mutex_test0]: acquire mutex!");
    listener.await;
    println!("[async_mutex_test0]: release the mutex!");
}

pub async fn async_mutex_test1<T>(mutex: Arc<AsyncMutex<T>>, event: Arc<Event>) {
    event.notify(1);
    println!("[async_mutex_test1]: try acquire mutex!");
    let _s = mutex.lock().await;
    println!("[async_mutex_test1]: acquire mutex!");
    println!("[async_mutex_test1]: release the mutex!");
}
