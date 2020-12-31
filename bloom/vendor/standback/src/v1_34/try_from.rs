use crate::{array::TryFromSliceError, convert::Infallible};
use core::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TryFromIntError(pub(crate) ());

impl TryFromIntError {
    #[doc(hidden)]
    pub fn __description(&self) -> &str {
        "out of range integral type conversion attempted"
    }
}

impl fmt::Display for TryFromIntError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.__description().fmt(fmt)
    }
}

impl From<Infallible> for TryFromIntError {
    fn from(x: Infallible) -> TryFromIntError {
        match x {}
    }
}

pub trait TryFrom<T>: Sized {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}

pub trait TryInto<T>: Sized {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}

impl<T, U> TryInto<U> for T
where
    U: TryFrom<T>,
{
    type Error = U::Error;

    fn try_into(self) -> Result<U, U::Error> {
        U::try_from(self)
    }
}

macro_rules! try_from_unbounded {
    ($source:ty, $($target:ty),*) => {$(
        impl TryFrom<$source> for $target {
            type Error = TryFromIntError;

            #[inline]
            fn try_from(value: $source) -> Result<Self, Self::Error> {
                Ok(value as $target)
            }
        }
    )*}
}

macro_rules! try_from_lower_bounded {
    ($source:ty, $($target:ty),*) => {$(
        impl TryFrom<$source> for $target {
            type Error = TryFromIntError;

            #[inline]
            fn try_from(u: $source) -> Result<$target, TryFromIntError> {
                if u >= 0 {
                    Ok(u as $target)
                } else {
                    Err(TryFromIntError(()))
                }
            }
        }
    )*}
}

macro_rules! try_from_upper_bounded {
    ($source:ty, $($target:ty),*) => {$(
        impl TryFrom<$source> for $target {
            type Error = TryFromIntError;

            #[inline]
            fn try_from(u: $source) -> Result<$target, TryFromIntError> {
                if u > (<$target>::max_value() as $source) {
                    Err(TryFromIntError(()))
                } else {
                    Ok(u as $target)
                }
            }
        }
    )*}
}

macro_rules! try_from_both_bounded {
    ($source:ty, $($target:ty),*) => {$(
        impl TryFrom<$source> for $target {
            type Error = TryFromIntError;

            #[inline]
            fn try_from(u: $source) -> Result<$target, TryFromIntError> {
                let min = <$target>::min_value() as $source;
                let max = <$target>::max_value() as $source;
                if u < min || u > max {
                    Err(TryFromIntError(()))
                } else {
                    Ok(u as $target)
                }
            }
        }
    )*}
}

macro_rules! rev {
    ($mac:ident, $source:ty, $($target:ty),*) => {$(
        $mac!($target, $source);
    )*}
}

try_from_upper_bounded!(u16, u8);
try_from_upper_bounded!(u32, u16, u8);
try_from_upper_bounded!(u64, u32, u16, u8);
try_from_upper_bounded!(u128, u64, u32, u16, u8);

try_from_both_bounded!(i16, i8);
try_from_both_bounded!(i32, i16, i8);
try_from_both_bounded!(i64, i32, i16, i8);
try_from_both_bounded!(i128, i64, i32, i16, i8);

try_from_upper_bounded!(u8, i8);
try_from_upper_bounded!(u16, i8, i16);
try_from_upper_bounded!(u32, i8, i16, i32);
try_from_upper_bounded!(u64, i8, i16, i32, i64);
try_from_upper_bounded!(u128, i8, i16, i32, i64, i128);

try_from_lower_bounded!(i8, u8, u16, u32, u64, u128);
try_from_lower_bounded!(i16, u16, u32, u64, u128);
try_from_lower_bounded!(i32, u32, u64, u128);
try_from_lower_bounded!(i64, u64, u128);
try_from_lower_bounded!(i128, u128);
try_from_both_bounded!(i16, u8);
try_from_both_bounded!(i32, u16, u8);
try_from_both_bounded!(i64, u32, u16, u8);
try_from_both_bounded!(i128, u64, u32, u16, u8);

try_from_upper_bounded!(usize, isize);
try_from_lower_bounded!(isize, usize);

#[cfg(target_pointer_width = "16")]
mod ptr_try_from_impls {
    use super::{TryFrom, TryFromIntError};

    try_from_upper_bounded!(usize, u8);
    try_from_unbounded!(usize, u16, u32, u64, u128);
    try_from_upper_bounded!(usize, i8, i16);
    try_from_unbounded!(usize, i32, i64, i128);

    try_from_both_bounded!(isize, u8);
    try_from_lower_bounded!(isize, u16, u32, u64, u128);
    try_from_both_bounded!(isize, i8);
    try_from_unbounded!(isize, i16, i32, i64, i128);

    rev!(try_from_upper_bounded, usize, u32, u64, u128);
    rev!(try_from_lower_bounded, usize, i8, i16);
    rev!(try_from_both_bounded, usize, i32, i64, i128);

    rev!(try_from_upper_bounded, isize, u16, u32, u64, u128);
    rev!(try_from_both_bounded, isize, i32, i64, i128);
}

#[cfg(target_pointer_width = "32")]
mod ptr_try_from_impls {
    use super::{TryFrom, TryFromIntError};

    try_from_upper_bounded!(usize, u8, u16);
    try_from_unbounded!(usize, u32, u64, u128);
    try_from_upper_bounded!(usize, i8, i16, i32);
    try_from_unbounded!(usize, i64, i128);

    try_from_both_bounded!(isize, u8, u16);
    try_from_lower_bounded!(isize, u32, u64, u128);
    try_from_both_bounded!(isize, i8, i16);
    try_from_unbounded!(isize, i32, i64, i128);

    rev!(try_from_unbounded, usize, u32);
    rev!(try_from_upper_bounded, usize, u64, u128);
    rev!(try_from_lower_bounded, usize, i8, i16, i32);
    rev!(try_from_both_bounded, usize, i64, i128);

    rev!(try_from_unbounded, isize, u16);
    rev!(try_from_upper_bounded, isize, u32, u64, u128);
    rev!(try_from_unbounded, isize, i32);
    rev!(try_from_both_bounded, isize, i64, i128);
}

#[cfg(target_pointer_width = "64")]
mod ptr_try_from_impls {
    use super::{TryFrom, TryFromIntError};

    try_from_upper_bounded!(usize, u8, u16, u32);
    try_from_unbounded!(usize, u64, u128);
    try_from_upper_bounded!(usize, i8, i16, i32, i64);
    try_from_unbounded!(usize, i128);

    try_from_both_bounded!(isize, u8, u16, u32);
    try_from_lower_bounded!(isize, u64, u128);
    try_from_both_bounded!(isize, i8, i16, i32);
    try_from_unbounded!(isize, i64, i128);

    rev!(try_from_unbounded, usize, u32, u64);
    rev!(try_from_upper_bounded, usize, u128);
    rev!(try_from_lower_bounded, usize, i8, i16, i32, i64);
    rev!(try_from_both_bounded, usize, i128);

    rev!(try_from_unbounded, isize, u16, u32);
    rev!(try_from_upper_bounded, isize, u64, u128);
    rev!(try_from_unbounded, isize, i32, i64);
    rev!(try_from_both_bounded, isize, i128);
}

macro_rules! impl_length_at_most_32 {
    ($($n:expr),+) => {$(
        impl<T> TryFrom<&[T]> for [T; $n]
        where
            T: Copy,
        {
            type Error = TryFromSliceError;

            fn try_from(slice: &[T]) -> Result<[T; $n], TryFromSliceError> {
                <&Self>::try_from(slice).map(|r| *r)
            }
        }

        impl<'a, T> TryFrom<&'a [T]> for &'a [T; $n] {
            type Error = TryFromSliceError;

            fn try_from(slice: &[T]) -> Result<&[T; $n], TryFromSliceError> {
                if slice.len() == $n {
                    let ptr = slice.as_ptr() as *const [T; $n];
                    unsafe { Ok(&*ptr) }
                } else {
                    Err(TryFromSliceError(()))
                }
            }
        }

        impl<'a, T> TryFrom<&'a mut [T]> for &'a mut [T; $n] {
            type Error = TryFromSliceError;

            fn try_from(slice: &mut [T]) -> Result<&mut [T; $n], TryFromSliceError> {
                if slice.len() == $n {
                    let ptr = slice.as_mut_ptr() as *mut [T; $n];
                    unsafe { Ok(&mut *ptr) }
                } else {
                    Err(TryFromSliceError(()))
                }
            }
        }
    )+}
}

impl_length_at_most_32![
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32
];

// Although we aren't able to `impl<T, U: Into<T>> TryFrom<U> for T`, we are
// able to implement it for any given type.
macro_rules! impl_identity {
    ($($type:ty),*) => {$(
        impl TryFrom<$type> for $type {
            type Error = Infallible;

            fn try_from(value: $type) -> Result<Self, Self::Error> {
                Ok(value)
            }
        }
    )*}
}

// Implement for some primitives. Other types can be trivially added upon
// request.
impl_identity![
    (),
    bool,
    char,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    f32,
    f64
];
