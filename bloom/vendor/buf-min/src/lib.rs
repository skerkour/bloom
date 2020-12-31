// Adapted from [`bytes`](https://github.com/tokio-rs/bytes)

#[cfg(feature = "bytes-buf")]
use bytes::{Bytes, BytesMut};

/// Minimal Buffer trait
pub trait Buffer {
    /// Into immutable type
    type Freeze;

    /// Returns new `Buffer` with capacity
    fn with_capacity(capacity: usize) -> Self
    where
        Self: Sized;

    /// Returns true if the `Buffer` has a length of 0.
    fn is_empty(&self) -> bool;

    /// Appends given bytes to this `Buffer`.
    ///
    /// # Panics
    /// Can panic if current length plus `src` length overflows usize
    fn extend_from_slice(&mut self, src: &[u8]);

    /// Reserves capacity for at least `additional` more bytes to be inserted
    /// into the given `Buffer`.
    ///
    /// # Panics
    /// Can panic if current capacity plus `additional` overflows usize
    fn reserve(&mut self, additional: usize);

    /// Converts `self` into a Freeze type
    fn freeze(self) -> Self::Freeze;

    /// Advance the internal cursor of the `Buffer`
    ///
    /// # Safety
    /// Can't advance more than capacity of the `Buffer`
    ///
    /// # Panics
    /// Can panic if length plus `cnt` is bigger than capacity
    unsafe fn advance(&mut self, cnt: usize);

    /// Return unsafe ptr to current `Buffer` position
    ///
    /// # Safety
    /// If buffer is full, can return invalid pointer
    unsafe fn buf_ptr(&mut self) -> *mut u8;
}

impl Buffer for Vec<u8> {
    type Freeze = Vec<u8>;

    #[inline]
    fn with_capacity(capacity: usize) -> Self
    where
        Self: Sized,
    {
        Vec::with_capacity(capacity)
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    #[inline]
    fn extend_from_slice(&mut self, src: &[u8]) {
        Buffer::reserve(self, src.len());
        unsafe {
            debug_assert!(self.capacity() - self.len() >= src.len());
            std::ptr::copy_nonoverlapping(src.as_ptr(), self.buf_ptr(), src.len());
            Buffer::advance(self, src.len())
        }
    }

    #[inline]
    fn reserve(&mut self, additional: usize) {
        debug_assert!(self.len() <= self.capacity());
        if self.capacity().wrapping_sub(self.len()) < additional {
            self.reserve(additional);
        }
    }

    #[inline]
    fn freeze(mut self) -> Self::Freeze {
        self.shrink_to_fit();
        self
    }

    #[inline]
    unsafe fn advance(&mut self, cnt: usize) {
        self.set_len(self.len() + cnt);
    }

    #[inline]
    unsafe fn buf_ptr(&mut self) -> *mut u8 {
        self.as_mut_ptr().add(self.len())
    }
}

#[cfg(feature = "bytes-buf")]
impl Buffer for BytesMut {
    type Freeze = Bytes;

    #[inline]
    fn with_capacity(capacity: usize) -> Self
    where
        Self: Sized,
    {
        BytesMut::with_capacity(capacity)
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    #[inline]
    fn extend_from_slice(&mut self, src: &[u8]) {
        self.reserve(src.len());
        unsafe {
            debug_assert!(self.capacity() - self.len() >= src.len());
            std::ptr::copy_nonoverlapping(src.as_ptr(), self.buf_ptr(), src.len());
            self.advance(src.len())
        }
    }

    #[inline(always)]
    fn reserve(&mut self, additional: usize) {
        self.reserve(additional);
    }

    #[inline(always)]
    fn freeze(self) -> Self::Freeze {
        self.freeze()
    }

    #[inline]
    unsafe fn advance(&mut self, cnt: usize) {
        let new_len = self.len() + cnt;
        debug_assert!(
            new_len <= self.capacity(),
            "new_len = {}; capacity = {}",
            new_len,
            self.capacity()
        );
        self.set_len(new_len);
    }

    #[inline]
    unsafe fn buf_ptr(&mut self) -> *mut u8 {
        self.as_mut_ptr().add(self.len())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let e = b"Hello world!";
        let mut buf: Vec<u8> = Buffer::with_capacity(0);
        Buffer::extend_from_slice(&mut buf, e);
        assert_eq!(e, &Buffer::freeze(buf)[..]);

        let mut buf: Vec<u8> = Buffer::with_capacity(124);
        Buffer::extend_from_slice(&mut buf, e);
        assert_eq!(e, &Buffer::freeze(buf)[..]);

        let mut buf: Vec<u8> = Buffer::with_capacity(14);
        Buffer::extend_from_slice(&mut buf, e);
    }
}

#[cfg(all(test, feature = "bytes-buf"))]
mod test_bytes {
    use super::*;

    use bytes::BytesMut;

    #[test]
    fn test() {
        let e = b"Hello world!";
        let mut buf: BytesMut = Buffer::with_capacity(0);
        Buffer::extend_from_slice(&mut buf, e);
        assert_eq!(e, &Buffer::freeze(buf)[..]);

        let mut buf: BytesMut = Buffer::with_capacity(124);
        Buffer::extend_from_slice(&mut buf, e);
        assert_eq!(e, &Buffer::freeze(buf)[..]);
    }
}
