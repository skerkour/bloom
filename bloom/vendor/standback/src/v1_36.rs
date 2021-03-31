mod iterator_copied;
mod maybe_uninit;
mod poll;
mod waker;

use core::fmt;

pub use self::iterator_copied::{Copied, Iterator_v1_36};
pub use self::maybe_uninit::MaybeUninit;
pub use self::poll::Poll;
pub use self::waker::{Context, RawWaker, RawWakerVTable, Waker};
use crate::traits::Sealed;

pub trait str_v1_36: Sealed<str> {
    fn as_mut_ptr(&mut self) -> *mut u8;
}

impl str_v1_36 for str {
    #[inline]
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self as *mut str as *mut u8
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TryFromSliceError(pub(crate) ());

impl fmt::Display for TryFromSliceError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.__description(), f)
    }
}

impl TryFromSliceError {
    #[inline]
    #[doc(hidden)]
    pub fn __description(&self) -> &str {
        "could not convert slice to array"
    }
}
