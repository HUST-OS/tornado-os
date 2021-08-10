//! ref: https://github.com/smol-rs/event-listener/blob/master/src/lib.rs
//! 事件监听器
//! 通过这个可以将同步的数据结构转换为异步数据结构，比如异步锁
//!
//! # Examples
//! ```
//! ```
#![no_std]
#![feature(llvm_asm)]

mod log;
mod sbi;

extern crate alloc;
use alloc::boxed::Box;
use alloc::sync::Arc;
use core::cell::{Cell, UnsafeCell};
use core::fmt;
use core::future::Future;
use core::mem::{self, ManuallyDrop};
use core::ops::{Deref, DerefMut};
use core::pin::Pin;
use core::ptr::{self, NonNull};
use core::sync::atomic::{self, AtomicPtr, AtomicUsize, Ordering};
use core::task::{Context, Poll, Waker};
use core::usize;
#[cfg(feature = "kernel")]
use rv_lock::{Lock, LockGuard};
#[cfg(any(not(feature = "kernel")))]
use spin::{Mutex, MutexGuard};

/// [`Event`] 的内部数据
struct Inner {
    /// 已被通知的 [`Entry`] （条目）数
    /// 如果所有条目都被通知了，该值被设为 `usize::MAX`
    ///
    /// 如果没有条目，该值被设为 `usize::MAX`
    notified: AtomicUsize,
    /// 保存注册的监听者的链表
    #[cfg(any(not(feature = "kernel")))]
    list: Mutex<List>,
    #[cfg(feature = "kernel")]
    list: Lock<List>,
    /// 链表的单缓冲条目（用于算法优化）
    cache: UnsafeCell<Entry>,
}

impl Inner {
    /// 把 [`List`] 锁住
    fn lock(&self) -> ListGuard<'_> {
        ListGuard {
            inner: self,
            guard: self.list.lock(),
        }
    }

    /// 返回链表单缓冲条目的指针
    #[inline(always)]
    fn cache_ptr(&self) -> NonNull<Entry> {
        unsafe { NonNull::new_unchecked(self.cache.get()) }
    }
}

/// 通知异步任务的同步原语
///
/// 监听者可以通过 [`Event::listen()`] 来进行注册。有两种方法来通知监听者：
/// 1. [`Event::notify()`] 通知一定数量的监听者
/// 2. [`Event::notify_additional()`] 通知一定数量的之前没有被通知的监听者
///
/// 如果一个消息发出去后，当前没有监听者，这个消息将会被丢失。
///
/// 监听者可以使用 `.await` 来异步等待一个消息
///
/// 如果一个监听者在没有收到任何消息的情况下走到生命周期的尽头，`dropping` 会通知
/// 其他监听了同一个 [`Event`] 的监听者。
///
/// 监听者们的注册和被通知操作遵循`先进先出`规律，保证公平性。
pub struct Event {
    /// A pointer to heap-allocated inner state.
    ///
    /// This pointer is initially null and gets lazily initialized on first use. Semantically, it
    /// is an `Arc<Inner>` so it's important to keep in mind that it contributes to the [`Arc`]'s
    /// reference count.
    inner: AtomicPtr<Inner>,
}

unsafe impl Send for Event {}
unsafe impl Sync for Event {}

impl Event {
    /// 创建一个新的 [`Event`]
    ///
    /// # Examples
    ///
    /// ```
    /// use event::Event;
    ///
    /// let event = Event::new();
    /// ```
    #[inline]
    pub const fn new() -> Event {
        Event {
            inner: AtomicPtr::new(ptr::null_mut()),
        }
    }

    /// 返回一个监听者
    ///
    /// This method emits a `SeqCst` fence after registering a listener.
    ///
    /// # Examples
    ///
    /// ```
    /// use event::Event;
    ///
    /// let event = Event::new();
    /// let listener = event.listen();
    /// ```
    #[cold]
    pub fn listen(&self) -> EventListener {
        let inner = self.inner();
        let listener = EventListener {
            inner: unsafe { Arc::clone(&ManuallyDrop::new(Arc::from_raw(inner))) },
            entry: Some(inner.lock().insert(inner.cache_ptr())),
        };

        // Make sure the listener is registered before whatever happens next.
        full_fence();
        listener
    }

    /// 通知一定数量的监听者
    ///
    /// 数量可以是零或者超过监听者的数量
    ///
    /// This method emits a `SeqCst` fence before notifying listeners.
    ///
    /// # Examples
    ///
    /// ```
    /// use event::Event;
    ///
    /// let event = Event::new();
    ///
    /// // This notification gets lost because there are no listeners.
    /// event.notify(1);
    ///
    /// let listener1 = event.listen();
    /// let listener2 = event.listen();
    /// let listener3 = event.listen();
    ///
    /// // Notifies two listeners.
    /// //
    /// // Listener queueing is fair, which means `listener1` and `listener2`
    /// // get notified here since they start listening before `listener3`.
    /// event.notify(2);
    /// ```
    #[inline]
    pub fn notify(&self, n: usize) {
        full_fence();
        if let Some(inner) = self.try_inner() {
            if inner.notified.load(Ordering::Acquire) < n {
                inner.lock().notify(n);
            }
        }
    }

    /// 通知一定数量的监听者但是不 `emit a SeqCst fence`
    ///
    /// 数量可以是零或者超过监听者的数量
    ///
    /// Unlike [`Event::notify()`], this method does not emit a `SeqCst` fence.
    ///
    /// # Examples
    ///
    /// ```
    /// use event::Event;
    /// use core::sync::atomic::{self, Ordering};
    ///
    /// let event = Event::new();
    ///
    /// // This notification gets lost because there are no listeners.
    /// event.notify(1);
    ///
    /// let listener1 = event.listen();
    /// let listener2 = event.listen();
    /// let listener3 = event.listen();
    ///
    /// // We should emit a fence manually when using relaxed notifications.
    /// atomic::fence(Ordering::SeqCst);
    ///
    /// // Notifies two listeners.
    /// //
    /// // Listener queueing is fair, which means `listener1` and `listener2`
    /// // get notified here since they start listening before `listener3`.
    /// event.notify(2);
    /// ```
    #[inline]
    pub fn notify_relaxed(&self, n: usize) {
        if let Some(inner) = self.try_inner() {
            // Notify if there is at least one unnotified listener and the number of notified
            // listeners is less than `n`.
            if inner.notified.load(Ordering::Acquire) < n {
                inner.lock().notify(n);
            }
        }
    }

    /// 通知一定数量没有被通知过的监听者
    ///
    /// 数量可以是零或者超过监听者的数量
    ///
    /// This method emits a `SeqCst` fence before notifying listeners.
    ///
    /// # Examples
    ///
    /// ```
    /// use event::Event;
    ///
    /// let event = Event::new();
    ///
    /// // This notification gets lost because there are no listeners.
    /// event.notify(1);
    ///
    /// let listener1 = event.listen();
    /// let listener2 = event.listen();
    /// let listener3 = event.listen();
    ///
    /// // Notifies two listeners.
    /// //
    /// // Listener queueing is fair, which means `listener1` and `listener2`
    /// // get notified here since they start listening before `listener3`.
    /// event.notify_additional(1);
    /// event.notify_additional(1);
    /// ```
    #[inline]
    pub fn notify_additional(&self, n: usize) {
        full_fence();

        if let Some(inner) = self.try_inner() {
            if inner.notified.load(Ordering::Acquire) < usize::MAX {
                inner.lock().notify_additional(n);
            }
        }
    }

    /// 通知一定数量没有被通知过的监听者但不 `emitting a SeqCst fence`
    ///
    /// 数量可以是零或者超过监听者的数量
    ///
    /// Unlike [`Event::notify_additional()`], this method does not emit a `SeqCst` fence.
    ///
    /// # Examples
    ///
    /// ```
    /// use event::Event;
    /// use core::sync::atomic::{self, Ordering};
    ///
    /// let event = Event::new();
    ///
    /// // This notification gets lost because there are no listeners.
    /// event.notify(1);
    ///
    /// let listener1 = event.listen();
    /// let listener2 = event.listen();
    /// let listener3 = event.listen();
    ///
    /// // We should emit a fence manually when using relaxed notifications.
    /// atomic::fence(Ordering::SeqCst);
    ///
    /// // Notifies two listeners.
    /// //
    /// // Listener queueing is fair, which means `listener1` and `listener2`
    /// // get notified here since they start listening before `listener3`.
    /// event.notify_additional_relaxed(1);
    /// event.notify_additional_relaxed(1);
    /// ```
    #[inline]
    pub fn notify_additional_relaxed(&self, n: usize) {
        if let Some(inner) = self.try_inner() {
            if inner.notified.load(Ordering::Acquire) < usize::MAX {
                inner.lock().notify_additional(n);
            }
        }
    }

    /// 如果 `inner` 的状态被初始化了，然后它的引用
    #[inline]
    fn try_inner(&self) -> Option<&Inner> {
        let inner = self.inner.load(Ordering::Acquire);
        unsafe { inner.as_ref() }
    }

    /// 返回 `inner` 的引用，如果有必要可以初始化
    fn inner(&self) -> &Inner {
        let mut inner = self.inner.load(Ordering::Acquire);

        if inner.is_null() {
            let new = Arc::new(Inner {
                notified: AtomicUsize::new(usize::MAX),
                #[cfg(any(not(feature = "kernel")))]
                list: Mutex::new(List {
                    head: None,
                    tail: None,
                    start: None,
                    len: 0,
                    notified: 0,
                    cache_used: false,
                }),
                #[cfg(feature = "kernel")]
                list: Lock::new(List {
                    head: None,
                    tail: None,
                    start: None,
                    len: 0,
                    notified: 0,
                    cache_used: false,
                }),
                cache: UnsafeCell::new(Entry {
                    state: Cell::new(State::Created),
                    prev: Cell::new(None),
                    next: Cell::new(None),
                }),
            });
            let new = Arc::into_raw(new) as *mut Inner;

            inner = self
                .inner
                .compare_exchange(inner, new, Ordering::AcqRel, Ordering::Acquire)
                .unwrap_or_else(|x| x);

            if inner.is_null() {
                inner = new;
            } else {
                unsafe {
                    drop(Arc::from_raw(new));
                }
            }
        }

        unsafe { &*inner }
    }
}

impl Drop for Event {
    #[inline]
    fn drop(&mut self) {
        let inner: *mut Inner = *self.inner.get_mut();

        // If the state pointer has been initialized, deallocate it.
        if !inner.is_null() {
            unsafe {
                drop(Arc::from_raw(inner));
            }
        }
    }
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("Event { .. }")
    }
}

impl Default for Event {
    fn default() -> Event {
        Event::new()
    }
}

/// 监听者
pub struct EventListener {
    inner: Arc<Inner>,
    entry: Option<NonNull<Entry>>,
}

unsafe impl Send for EventListener {}
unsafe impl Sync for EventListener {}

impl fmt::Debug for EventListener {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("EventListener { .. }")
    }
}

impl Future for EventListener {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut list = self.inner.lock();

        let entry = match self.entry {
            None => unreachable!("cannot poll a completed `EventListener` future"),
            Some(entry) => entry,
        };
        let state = unsafe { &entry.as_ref().state };

        match state.replace(State::Notified(false)) {
            State::Notified(_) => {
                list.remove(entry, self.inner.cache_ptr());
                drop(list);
                self.entry = None;
                return Poll::Ready(());
            }
            State::Created => {
                state.set(State::Polling(cx.waker().clone()));
            }
            State::Polling(w) => {
                if w.will_wake(cx.waker()) {
                    state.set(State::Polling(w));
                } else {
                    state.set(State::Polling(cx.waker().clone()));
                }
            }
        }

        Poll::Pending
    }
}

impl Drop for EventListener {
    fn drop(&mut self) {
        if let Some(entry) = self.entry.take() {
            let mut list = self.inner.lock();

            if let State::Notified(additional) = list.remove(entry, self.inner.cache_ptr()) {
                if additional {
                    list.notify_additional(1);
                } else {
                    list.notify(1);
                }
            }
        }
    }
}

/// A guard holding the linked list locked.
struct ListGuard<'a> {
    /// [`Event`] 的 `inner` 的引用
    inner: &'a Inner,
    /// The actual guard that acquired the linked list.
    #[cfg(any(not(feature = "kernel")))]
    guard: MutexGuard<'a, List>,
    #[cfg(feature = "kernel")]
    guard: LockGuard<'a, List>,
}

impl Drop for ListGuard<'_> {
    #[inline]
    fn drop(&mut self) {
        let list = &mut **self;

        // Update the atomic `notified` counter.
        let notified = if list.notified < list.len {
            list.notified
        } else {
            usize::MAX
        };
        self.inner.notified.store(notified, Ordering::Release);
    }
}

impl Deref for ListGuard<'_> {
    type Target = List;

    #[inline]
    fn deref(&self) -> &List {
        &*self.guard
    }
}

impl DerefMut for ListGuard<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut List {
        &mut *self.guard
    }
}

/// 监听者的状态
enum State {
    /// 刚刚被创建
    Created,
    /// 已经接收到一个通知
    ///
    /// 如果这是个 `additional` 通知，`bool` 将为 `true`
    Notified(bool),
    /// 正在被一个异步任务 `poll`，保存了任务的 `waker`
    Polling(Waker),
}

impl State {
    /// 如果已经被通知了，返回 `true`
    #[inline]
    fn is_notified(&self) -> bool {
        match self {
            State::Notified(_) => true,
            State::Created | State::Polling(_) => false,
        }
    }
}

/// 一个条目，代表一个监听者
struct Entry {
    /// 监听者的状态
    state: Cell<State>,
    /// 链表的前一个条目
    prev: Cell<Option<NonNull<Entry>>>,
    /// 链表的后一个条目
    next: Cell<Option<NonNull<Entry>>>,
}

/// 链表
struct List {
    #[allow(unused)]
    head: Option<NonNull<Entry>>,
    tail: Option<NonNull<Entry>>,
    start: Option<NonNull<Entry>>,
    len: usize,
    notified: usize,
    cache_used: bool,
}

impl List {
    /// 插入一个新的条目到链表
    fn insert(&mut self, cache: NonNull<Entry>) -> NonNull<Entry> {
        unsafe {
            let entry = Entry {
                state: Cell::new(State::Created),
                prev: Cell::new(self.tail),
                next: Cell::new(None),
            };

            let entry = if self.cache_used {
                NonNull::new_unchecked(Box::into_raw(Box::new(entry)))
            } else {
                self.cache_used = true;
                cache.as_ptr().write(entry);
                cache
            };

            match mem::replace(&mut self.tail, Some(entry)) {
                None => self.head = Some(entry),
                Some(t) => t.as_ref().next.set(Some(entry)),
            }

            if self.start.is_none() {
                self.start = self.tail;
            }

            self.len += 1;

            entry
        }
    }

    /// 从链表移除一个条目并且返回其的状态
    fn remove(&mut self, entry: NonNull<Entry>, cache: NonNull<Entry>) -> State {
        unsafe {
            let prev = entry.as_ref().prev.get();
            let next = entry.as_ref().next.get();

            match prev {
                None => self.head = next,
                Some(p) => p.as_ref().next.set(next),
            }

            match next {
                None => self.tail = prev,
                Some(n) => n.as_ref().prev.set(prev),
            }

            if self.start == Some(entry) {
                self.start = next;
            }

            let state = if ptr::eq(entry.as_ptr(), cache.as_ptr()) {
                self.cache_used = false;
                entry.as_ref().state.replace(State::Created)
            } else {
                Box::from_raw(entry.as_ptr()).state.into_inner()
            };

            if state.is_notified() {
                self.notified -= 1;
            }
            self.len -= 1;

            state
        }
    }

    /// 通知一定数量的条目
    #[cold]
    fn notify(&mut self, mut n: usize) {
        if n <= self.notified {
            return;
        }
        n -= self.notified;
        while n > 0 {
            n -= 1;

            match self.start {
                None => break,
                Some(e) => {
                    let e = unsafe { e.as_ref() };
                    self.start = e.next.get();

                    match e.state.replace(State::Notified(false)) {
                        State::Notified(_) => {}
                        State::Created => {}
                        State::Polling(w) => w.wake_by_ref(),
                    }

                    self.notified += 1;
                }
            }
        }
    }

    /// 通知一定数量的 `additional` 条目
    #[cold]
    fn notify_additional(&mut self, mut n: usize) {
        while n > 0 {
            n -= 1;

            match self.start {
                None => break,
                Some(e) => {
                    let e = unsafe { e.as_ref() };
                    self.start = e.next.get();

                    match e.state.replace(State::Notified(true)) {
                        State::Notified(_) => {}
                        State::Created => {}
                        State::Polling(w) => w.wake_by_ref(),
                    }
                    self.notified += 1;
                }
            }
        }
    }
}

/// 内存排序相关？
#[inline]
fn full_fence() {
    atomic::fence(Ordering::SeqCst);
}
