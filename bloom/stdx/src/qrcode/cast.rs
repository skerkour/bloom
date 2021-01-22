use std::convert::TryFrom;
use std::fmt::Display;

pub trait Truncate {
    fn truncate_as_u8(self) -> u8;
}

impl Truncate for u16 {
    #[allow(clippy::cast_possible_truncation)]
    fn truncate_as_u8(self) -> u8 {
        (self & 0xff) as u8
    }
}

pub trait As {
    fn as_u16(self) -> u16;
    fn as_i16(self) -> i16;
    fn as_u32(self) -> u32;
    fn as_usize(self) -> usize;
    fn as_isize(self) -> isize;
}

trait ExpectOrOverflow {
    type Output;
    fn expect_or_overflow<D: Display>(self, value: D, ty: &str) -> Self::Output;
}

impl<T> ExpectOrOverflow for Option<T> {
    type Output = T;
    fn expect_or_overflow<D: Display>(self, value: D, ty: &str) -> Self::Output {
        match self {
            Some(v) => v,
            None => panic!("{} overflows {}", value, ty),
        }
    }
}

macro_rules! impl_as {
    ($ty:ty) => {
        #[cfg(debug_assertions)]
        impl As for $ty {
            fn as_u16(self) -> u16 {
                u16::try_from(self).ok().expect_or_overflow(self, "u16")
            }

            fn as_i16(self) -> i16 {
                i16::try_from(self).ok().expect_or_overflow(self, "i16")
            }

            fn as_u32(self) -> u32 {
                u32::try_from(self).ok().expect_or_overflow(self, "u32")
            }

            fn as_usize(self) -> usize {
                usize::try_from(self).ok().expect_or_overflow(self, "usize")
            }

            fn as_isize(self) -> isize {
                isize::try_from(self).ok().expect_or_overflow(self, "isize")
            }
        }

        #[cfg(not(debug_assertions))]
        impl As for $ty {
            fn as_u16(self) -> u16 {
                self as u16
            }
            fn as_i16(self) -> i16 {
                self as i16
            }
            fn as_u32(self) -> u32 {
                self as u32
            }
            fn as_usize(self) -> usize {
                self as usize
            }
            fn as_isize(self) -> isize {
                self as isize
            }
        }
    };
}

impl_as!(i16);
impl_as!(u32);
impl_as!(usize);
impl_as!(isize);
