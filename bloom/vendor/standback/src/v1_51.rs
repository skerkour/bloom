mod slice;

use core::iter::Peekable;
#[cfg(feature = "std")]
use core::mem;
#[cfg(feature = "std")]
use std::io::{Seek, SeekFrom};
#[cfg(feature = "std")]
use std::sync::Arc;

use crate::task::Poll;
#[cfg(__standback_before_1_40)]
use crate::v1_40::Option_v1_40_;
#[cfg(__standback_before_1_50)]
use crate::v1_50::Bool_v1_50;

#[cfg(feature = "std")]
pub trait Arc_v1_51<T> {
    unsafe fn decrement_strong_count(ptr: *const T);
    unsafe fn increment_strong_count(ptr: *const T);
}

#[cfg(feature = "std")]
impl<T> Arc_v1_51<T> for Arc<T> {
    #[inline]
    unsafe fn decrement_strong_count(ptr: *const T) {
        drop(Arc::from_raw(ptr));
    }

    #[inline]
    unsafe fn increment_strong_count(ptr: *const T) {
        let arc = mem::ManuallyDrop::new(Arc::<T>::from_raw(ptr));
        let _arc_clone: mem::ManuallyDrop<_> = arc.clone();
    }
}

pub trait Peekable_v1_51<I: Iterator> {
    fn next_if(&mut self, func: impl FnOnce(&I::Item) -> bool) -> Option<I::Item>;
    fn next_if_eq<T>(&mut self, expected: &T) -> Option<I::Item>
    where
        T: ?Sized,
        I::Item: PartialEq<T>;
}

impl<I: Iterator> Peekable_v1_51<I> for Peekable<I> {
    fn next_if(&mut self, func: impl FnOnce(&I::Item) -> bool) -> Option<I::Item> {
        func(self.peek()?).then(|| self.next()).flatten()
    }

    fn next_if_eq<T>(&mut self, expected: &T) -> Option<I::Item>
    where
        T: ?Sized,
        I::Item: PartialEq<T>,
    {
        self.next_if(|next| next == expected)
    }
}

#[cfg(feature = "std")]
pub trait Seek_v1_51 {
    fn stream_position(&mut self) -> std::io::Result<u64>;
}

#[cfg(feature = "std")]
impl<T: Seek> Seek_v1_51 for T {
    fn stream_position(&mut self) -> std::io::Result<u64> {
        self.seek(SeekFrom::Current(0))
    }
}

pub trait Slice_v1_51<T> {
    fn fill_with<F>(&mut self, f: F)
    where
        F: FnMut() -> T;
    fn split_inclusive_mut<F>(&mut self, pred: F) -> slice::SplitInclusiveMut<'_, T, F>
    where
        F: FnMut(&T) -> bool;
    fn split_inclusive<F>(&self, pred: F) -> slice::SplitInclusive<'_, T, F>
    where
        F: FnMut(&T) -> bool;
    fn strip_prefix(&self, prefix: &[T]) -> Option<&[T]>
    where
        T: PartialEq;
    fn strip_suffix(&self, suffix: &[T]) -> Option<&[T]>
    where
        T: PartialEq;
}

impl<T> Slice_v1_51<T> for [T] {
    fn fill_with<F>(&mut self, mut f: F)
    where
        F: FnMut() -> T,
    {
        for el in self {
            *el = f();
        }
    }

    #[inline]
    fn split_inclusive_mut<F>(&mut self, pred: F) -> slice::SplitInclusiveMut<'_, T, F>
    where
        F: FnMut(&T) -> bool,
    {
        slice::SplitInclusiveMut::new(self, pred)
    }

    #[inline]
    fn split_inclusive<F>(&self, pred: F) -> slice::SplitInclusive<'_, T, F>
    where
        F: FnMut(&T) -> bool,
    {
        slice::SplitInclusive::new(self, pred)
    }

    #[must_use = "returns the subslice without modifying the original"]
    fn strip_prefix(&self, prefix: &[T]) -> Option<&[T]>
    where
        T: PartialEq,
    {
        let n = prefix.len();
        if n <= self.len() {
            let (head, tail) = self.split_at(n);
            if head == prefix {
                return Some(tail);
            }
        }
        None
    }

    #[must_use = "returns the subslice without modifying the original"]
    fn strip_suffix(&self, suffix: &[T]) -> Option<&[T]>
    where
        T: PartialEq,
    {
        let (len, n) = (self.len(), suffix.len());
        if n <= len {
            let (head, tail) = self.split_at(len - n);
            if tail == suffix {
                return Some(head);
            }
        }
        None
    }
}

#[cfg(all(__standback_since_1_33, feature = "std"))]
pub trait Wake {
    fn wake(self: Arc<Self>);
    #[cfg(__standback_since_v1_41)]
    fn wake_by_ref(self: &Arc<Self>) {
        self.clone().wake();
    }
}

pub trait Integer_v1_51 {
    type __StandbackUnsigned;
    fn unsigned_abs(self) -> Self::__StandbackUnsigned;
}

macro_rules! impl_integer {
    ($($int:ty => $uint:ty)*) => {$(
        impl Integer_v1_51 for $int {
            type __StandbackUnsigned = $uint;
            #[inline]
            fn unsigned_abs(self) -> Self::__StandbackUnsigned {
                 self.wrapping_abs() as $uint
            }
        }
    )*};
}

impl_integer! {
    i8 => u8
    i16 => u16
    i32 => u32
    i64 => u64
    i128 => u128
}

pub trait Poll_v1_51<T, E> {
    fn map_ok<U, F>(self, f: F) -> Poll<Result<U, E>>
    where
        F: FnOnce(T) -> U;
    fn map_err<U, F>(self, f: F) -> Poll<Result<T, U>>
    where
        F: FnOnce(E) -> U;
}

impl<T, E> Poll_v1_51<T, E> for Poll<Result<T, E>> {
    fn map_ok<U, F>(self, f: F) -> Poll<Result<U, E>>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Poll::Ready(Ok(t)) => Poll::Ready(Ok(f(t))),
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            Poll::Pending => Poll::Pending,
        }
    }

    fn map_err<U, F>(self, f: F) -> Poll<Result<T, U>>
    where
        F: FnOnce(E) -> U,
    {
        match self {
            Poll::Ready(Ok(t)) => Poll::Ready(Ok(t)),
            Poll::Ready(Err(e)) => Poll::Ready(Err(f(e))),
            Poll::Pending => Poll::Pending,
        }
    }
}
