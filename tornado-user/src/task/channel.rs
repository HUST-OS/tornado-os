//! 任务间通信通道(channel)实现
//!
//! 目前只考虑一对一的场景
use alloc::sync::Arc;
use async_mutex::AsyncMutex;
use core::mem::MaybeUninit;
use core::ptr;
use event::Event;

/// 缓冲区
struct ChannelBuf<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    head: usize,
    tail: usize,
}

impl<T, const N: usize> ChannelBuf<T, N> {
    pub const fn new() -> Self {
        Self {
            data: MaybeUninit::uninit_array(),
            head: 0,
            tail: 0,
        }
    }
    pub const fn len(&self) -> usize {
        self.tail.wrapping_sub(self.head) % N
    }
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.tail == self.head
    }
    #[inline]
    fn is_full(&self) -> bool {
        self.len() == N - 1
    }
    pub fn push_back(&mut self, val: T) -> Option<T> {
        if self.is_full() {
            return Some(val);
        }
        unsafe { *self.data[self.tail].as_mut_ptr() = val };
        self.tail = self.tail.wrapping_add(1);
        if self.tail >= N || self.tail == 0 {
            self.tail = self.tail.wrapping_sub(N);
        }
        None
    }
    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let value = unsafe { ptr::read(self.data[self.head].as_ptr()) };
        self.head = self.head.wrapping_add(1); // assured non empty
        if self.head >= N || self.head == 0 {
            self.head = self.head.wrapping_sub(N);
        }
        Some(value)
    }
}

/// 接收者
pub struct Receiver<T, const N: usize> {
    buf: Arc<AsyncMutex<ChannelBuf<T, N>>>,
    rx_event: Arc<Event>,
    tx_event: Arc<Event>,
}

impl<T, const N: usize> Receiver<T, N> {
    pub async fn receive(&self) -> T {
        let rx_listener = self.rx_event.listen();
        let should_yield;
        {
            let s = self.buf.lock().await;
            should_yield = s.is_empty();
        }
        if should_yield {
            // 如果缓冲区为空，这里先主动让出
            rx_listener.await;
        }
        // 该任务被唤醒，在一对一的场景下缓冲区必不为空
        let mut s = self.buf.lock().await;
        let val = s.pop_front().unwrap();
        // 通知写端
        // 如果没有写端在监听，这个消息被丢失
        self.tx_event.notify(1);
        val
    }
}

/// 发送者
pub struct Sender<T, const N: usize> {
    buf: Arc<AsyncMutex<ChannelBuf<T, N>>>,
    rx_event: Arc<Event>,
    tx_event: Arc<Event>,
}

impl<T, const N: usize> Sender<T, N> {
    pub async fn send(&self, t: T) {
        let tx_listener = self.tx_event.listen();
        let should_yield;
        {
            let s = self.buf.lock().await;
            should_yield = s.is_full();
        }
        if should_yield {
            // 如果缓冲区已满，这里主动让出
            tx_listener.await;
        }
        // 该任务被唤醒，在一对一的情况下缓冲区必没满
        let mut s = self.buf.lock().await;
        assert!(s.push_back(t).is_none());
        // 通知读端
        // 如果没有读端在监听，这个消失被丢失
        self.rx_event.notify(1);
    }
}

pub fn bounded<T, const N: usize>() -> (Sender<T, N>, Receiver<T, N>) {
    let buf = Arc::new(AsyncMutex::new(ChannelBuf::new()));
    let tx_event = Arc::new(Event::new());
    let rx_event = Arc::new(Event::new());
    let sender = Sender {
        buf: Arc::clone(&buf),
        rx_event: Arc::clone(&rx_event),
        tx_event: Arc::clone(&tx_event),
    };
    let receiver = Receiver {
        buf: Arc::clone(&buf),
        rx_event: Arc::clone(&rx_event),
        tx_event: Arc::clone(&tx_event),
    };
    (sender, receiver)
}
