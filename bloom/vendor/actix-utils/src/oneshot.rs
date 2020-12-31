//! A one-shot, futures-aware channel.
use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

pub use futures_channel::oneshot::Canceled;
use slab::Slab;

use crate::task::LocalWaker;

/// Creates a new futures-aware, one-shot channel.
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Rc::new(RefCell::new(Inner {
        value: None,
        rx_task: LocalWaker::new(),
    }));
    let tx = Sender {
        inner: inner.clone(),
    };
    let rx = Receiver { inner };
    (tx, rx)
}

/// Creates a new futures-aware, pool of one-shot's.
pub fn pool<T>() -> Pool<T> {
    Pool(Rc::new(RefCell::new(Slab::new())))
}

/// Represents the completion half of a oneshot through which the result of a
/// computation is signaled.
#[derive(Debug)]
pub struct Sender<T> {
    inner: Rc<RefCell<Inner<T>>>,
}

/// A future representing the completion of a computation happening elsewhere in
/// memory.
#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub struct Receiver<T> {
    inner: Rc<RefCell<Inner<T>>>,
}

// The channels do not ever project Pin to the inner T
impl<T> Unpin for Receiver<T> {}
impl<T> Unpin for Sender<T> {}

#[derive(Debug)]
struct Inner<T> {
    value: Option<T>,
    rx_task: LocalWaker,
}

impl<T> Sender<T> {
    /// Completes this oneshot with a successful result.
    ///
    /// This function will consume `self` and indicate to the other end, the
    /// `Receiver`, that the error provided is the result of the computation this
    /// represents.
    ///
    /// If the value is successfully enqueued for the remote end to receive,
    /// then `Ok(())` is returned. If the receiving end was dropped before
    /// this function was called, however, then `Err` is returned with the value
    /// provided.
    pub fn send(self, val: T) -> Result<(), T> {
        if Rc::strong_count(&self.inner) == 2 {
            let mut inner = self.inner.borrow_mut();
            inner.value = Some(val);
            inner.rx_task.wake();
            Ok(())
        } else {
            Err(val)
        }
    }

    /// Tests to see whether this `Sender`'s corresponding `Receiver`
    /// has gone away.
    pub fn is_canceled(&self) -> bool {
        Rc::strong_count(&self.inner) == 1
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        self.inner.borrow().rx_task.wake();
    }
}

impl<T> Future for Receiver<T> {
    type Output = Result<T, Canceled>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        // If we've got a value, then skip the logic below as we're done.
        if let Some(val) = this.inner.borrow_mut().value.take() {
            return Poll::Ready(Ok(val));
        }

        // Check if sender is dropped and return error if it is.
        if Rc::strong_count(&this.inner) == 1 {
            Poll::Ready(Err(Canceled))
        } else {
            this.inner.borrow().rx_task.register(cx.waker());
            Poll::Pending
        }
    }
}

/// Futures-aware, pool of one-shot's.
pub struct Pool<T>(Rc<RefCell<Slab<PoolInner<T>>>>);

bitflags::bitflags! {
    pub struct Flags: u8 {
        const SENDER = 0b0000_0001;
        const RECEIVER = 0b0000_0010;
    }
}

#[derive(Debug)]
struct PoolInner<T> {
    flags: Flags,
    value: Option<T>,
    waker: LocalWaker,
}

impl<T> Pool<T> {
    pub fn channel(&mut self) -> (PSender<T>, PReceiver<T>) {
        let token = self.0.borrow_mut().insert(PoolInner {
            flags: Flags::all(),
            value: None,
            waker: LocalWaker::default(),
        });

        (
            PSender {
                token,
                inner: self.0.clone(),
            },
            PReceiver {
                token,
                inner: self.0.clone(),
            },
        )
    }
}

impl<T> Clone for Pool<T> {
    fn clone(&self) -> Self {
        Pool(self.0.clone())
    }
}

/// Represents the completion half of a oneshot through which the result of a
/// computation is signaled.
#[derive(Debug)]
pub struct PSender<T> {
    token: usize,
    inner: Rc<RefCell<Slab<PoolInner<T>>>>,
}

/// A future representing the completion of a computation happening elsewhere in
/// memory.
#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub struct PReceiver<T> {
    token: usize,
    inner: Rc<RefCell<Slab<PoolInner<T>>>>,
}

// The one-shots do not ever project Pin to the inner T
impl<T> Unpin for PReceiver<T> {}
impl<T> Unpin for PSender<T> {}

impl<T> PSender<T> {
    /// Completes this oneshot with a successful result.
    ///
    /// This function will consume `self` and indicate to the other end, the
    /// `Receiver`, that the error provided is the result of the computation this
    /// represents.
    ///
    /// If the value is successfully enqueued for the remote end to receive,
    /// then `Ok(())` is returned. If the receiving end was dropped before
    /// this function was called, however, then `Err` is returned with the value
    /// provided.
    pub fn send(self, val: T) -> Result<(), T> {
        let mut inner = self.inner.borrow_mut();
        let inner = unsafe { inner.get_unchecked_mut(self.token) };

        if inner.flags.contains(Flags::RECEIVER) {
            inner.value = Some(val);
            inner.waker.wake();
            Ok(())
        } else {
            Err(val)
        }
    }

    /// Tests to see whether this `Sender`'s corresponding `Receiver`
    /// has gone away.
    pub fn is_canceled(&self) -> bool {
        !unsafe { self.inner.borrow().get_unchecked(self.token) }
            .flags
            .contains(Flags::RECEIVER)
    }
}

impl<T> Drop for PSender<T> {
    fn drop(&mut self) {
        let mut inner = self.inner.borrow_mut();
        let inner_token = unsafe { inner.get_unchecked_mut(self.token) };
        if inner_token.flags.contains(Flags::RECEIVER) {
            inner_token.waker.wake();
            inner_token.flags.remove(Flags::SENDER);
        } else {
            inner.remove(self.token);
        }
    }
}

impl<T> Drop for PReceiver<T> {
    fn drop(&mut self) {
        let mut inner = self.inner.borrow_mut();
        let inner_token = unsafe { inner.get_unchecked_mut(self.token) };
        if inner_token.flags.contains(Flags::SENDER) {
            inner_token.flags.remove(Flags::RECEIVER);
        } else {
            inner.remove(self.token);
        }
    }
}

impl<T> Future for PReceiver<T> {
    type Output = Result<T, Canceled>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let mut inner = this.inner.borrow_mut();
        let inner = unsafe { inner.get_unchecked_mut(this.token) };

        // If we've got a value, then skip the logic below as we're done.
        if let Some(val) = inner.value.take() {
            return Poll::Ready(Ok(val));
        }

        // Check if sender is dropped and return error if it is.
        if !inner.flags.contains(Flags::SENDER) {
            Poll::Ready(Err(Canceled))
        } else {
            inner.waker.register(cx.waker());
            Poll::Pending
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::future::lazy;

    #[actix_rt::test]
    async fn test_oneshot() {
        let (tx, rx) = channel();
        tx.send("test").unwrap();
        assert_eq!(rx.await.unwrap(), "test");

        let (tx, rx) = channel();
        assert!(!tx.is_canceled());
        drop(rx);
        assert!(tx.is_canceled());
        assert!(tx.send("test").is_err());

        let (tx, rx) = channel::<&'static str>();
        drop(tx);
        assert!(rx.await.is_err());

        let (tx, mut rx) = channel::<&'static str>();
        assert_eq!(lazy(|cx| Pin::new(&mut rx).poll(cx)).await, Poll::Pending);
        tx.send("test").unwrap();
        assert_eq!(rx.await.unwrap(), "test");

        let (tx, mut rx) = channel::<&'static str>();
        assert_eq!(lazy(|cx| Pin::new(&mut rx).poll(cx)).await, Poll::Pending);
        drop(tx);
        assert!(rx.await.is_err());
    }

    #[actix_rt::test]
    async fn test_pool() {
        let (tx, rx) = pool().channel();
        tx.send("test").unwrap();
        assert_eq!(rx.await.unwrap(), "test");

        let (tx, rx) = pool().channel();
        assert!(!tx.is_canceled());
        drop(rx);
        assert!(tx.is_canceled());
        assert!(tx.send("test").is_err());

        let (tx, rx) = pool::<&'static str>().channel();
        drop(tx);
        assert!(rx.await.is_err());

        let (tx, mut rx) = pool::<&'static str>().channel();
        assert_eq!(lazy(|cx| Pin::new(&mut rx).poll(cx)).await, Poll::Pending);
        tx.send("test").unwrap();
        assert_eq!(rx.await.unwrap(), "test");

        let (tx, mut rx) = pool::<&'static str>().channel();
        assert_eq!(lazy(|cx| Pin::new(&mut rx).poll(cx)).await, Poll::Pending);
        drop(tx);
        assert!(rx.await.is_err());
    }
}
