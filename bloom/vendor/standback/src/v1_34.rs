mod try_from;

pub use self::try_from::{TryFrom, TryFromIntError, TryInto};
pub use crate::array::TryFromSliceError;
use crate::traits::{Integer, Sealed};
#[cfg(feature = "std")]
use core::mem;
use core::{
    fmt,
    hash::{Hash, Hasher},
    iter::FusedIterator,
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Infallible {}

impl fmt::Debug for Infallible {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {}
    }
}

impl fmt::Display for Infallible {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {}
    }
}

impl Hash for Infallible {
    fn hash<H: Hasher>(&self, _: &mut H) {
        match *self {}
    }
}

#[inline]
pub fn from_fn<T, F>(f: F) -> FromFn<F>
where
    F: FnMut() -> Option<T>,
{
    FromFn(f)
}

#[derive(Clone)]
pub struct FromFn<F>(F);

impl<T, F> Iterator for FromFn<F>
where
    F: FnMut() -> Option<T>,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        (self.0)()
    }
}

impl<F> fmt::Debug for FromFn<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FromFn").finish()
    }
}

pub fn successors<T, F>(first: Option<T>, succ: F) -> Successors<T, F>
where
    F: FnMut(&T) -> Option<T>,
{
    Successors { next: first, succ }
}

#[derive(Clone)]
pub struct Successors<T, F> {
    next: Option<T>,
    succ: F,
}

impl<T, F> Iterator for Successors<T, F>
where
    F: FnMut(&T) -> Option<T>,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.next.take()?;
        self.next = (self.succ)(&item);
        Some(item)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.next.is_some() {
            (1, None)
        } else {
            (0, Some(0))
        }
    }
}

impl<T, F> FusedIterator for Successors<T, F> where F: FnMut(&T) -> Option<T> {}

impl<T: fmt::Debug, F> fmt::Debug for Successors<T, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Successors")
            .field("next", &self.next)
            .finish()
    }
}

pub trait Slice_v1_34<T>: Sealed<[T]> {
    fn sort_by_cached_key<K, F>(&mut self, f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord;
}

#[cfg(feature = "std")]
impl<T> Slice_v1_34<T> for [T] {
    #[inline]
    fn sort_by_cached_key<K, F>(&mut self, f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        macro_rules! sort_by_key {
            ($t:ty, $slice:ident, $f:ident) => {{
                let mut indices: Vec<_> = $slice
                    .iter()
                    .map($f)
                    .enumerate()
                    .map(|(i, k)| (k, i as $t))
                    .collect();
                indices.sort_unstable();
                for i in 0..$slice.len() {
                    let mut index = indices[i].1;
                    while (index as usize) < i {
                        index = indices[index as usize].1;
                    }
                    indices[i].1 = index;
                    $slice.swap(i, index as usize);
                }
            }};
        }

        let sz_u8 = mem::size_of::<(K, u8)>();
        let sz_u16 = mem::size_of::<(K, u16)>();
        let sz_u32 = mem::size_of::<(K, u32)>();
        let sz_usize = mem::size_of::<(K, usize)>();

        let len = self.len();
        if len < 2 {
            return;
        }
        if sz_u8 < sz_u16 && len <= (u8::max_value() as usize) {
            return sort_by_key!(u8, self, f);
        }
        if sz_u16 < sz_u32 && len <= (u16::max_value() as usize) {
            return sort_by_key!(u16, self, f);
        }
        if sz_u32 < sz_usize && len <= (u32::max_value() as usize) {
            return sort_by_key!(u32, self, f);
        }
        sort_by_key!(usize, self, f)
    }
}

pub trait Pow_v1_34: Integer {
    fn checked_pow(self, exp: u32) -> Option<Self>;
    fn saturating_pow(self, exp: u32) -> Self;
    fn wrapping_pow(self, exp: u32) -> Self;
    fn overflowing_pow(self, exp: u32) -> (Self, bool);
}

macro_rules! impl_pow_for_signed {
    ($($type:ty)+) => {$(
        impl Pow_v1_34 for $type {
            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            fn checked_pow(self, mut exp: u32) -> Option<Self> {
                let mut base = self;
                let mut acc: Self = 1;

                while exp > 1 {
                    if (exp & 1) == 1 {
                        acc = acc.checked_mul(base)?;
                    }
                    exp /= 2;
                    base = base.checked_mul(base)?;
                }

                if exp == 1 {
                    acc = acc.checked_mul(base)?;
                }

                Some(acc)
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            fn saturating_pow(self, exp: u32) -> Self {
                match self.checked_pow(exp) {
                    Some(x) => x,
                    None if self < 0 && exp % 2 == 1 => Self::min_value(),
                    None => Self::max_value(),
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            fn wrapping_pow(self, mut exp: u32) -> Self {
                let mut base = self;
                let mut acc: Self = 1;

                while exp > 1 {
                    if (exp & 1) == 1 {
                        acc = acc.wrapping_mul(base);
                    }
                    exp /= 2;
                    base = base.wrapping_mul(base);
                }

                if exp == 1 {
                    acc = acc.wrapping_mul(base);
                }

                acc
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            fn overflowing_pow(self, mut exp: u32) -> (Self, bool) {
                let mut base = self;
                let mut acc: Self = 1;
                let mut overflown = false;
                let mut r;

                while exp > 1 {
                    if (exp & 1) == 1 {
                        r = acc.overflowing_mul(base);
                        acc = r.0;
                        overflown |= r.1;
                    }
                    exp /= 2;
                    r = base.overflowing_mul(base);
                    base = r.0;
                    overflown |= r.1;
                }

                if exp == 1 {
                    r = acc.overflowing_mul(base);
                    acc = r.0;
                    overflown |= r.1;
                }

                (acc, overflown)
            }
        }
    )+};
}

impl_pow_for_signed![i8 i16 i32 i64 i128 isize];

macro_rules! impl_pow_for_unsigned {
    ($($type:ty)+) => {$(
        impl Pow_v1_34 for $type {
            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            fn checked_pow(self, mut exp: u32) -> Option<Self> {
                let mut base = self;
                let mut acc: Self = 1;

                while exp > 1 {
                    if (exp & 1) == 1 {
                        acc = acc.checked_mul(base)?;
                    }
                    exp /= 2;
                    base = base.checked_mul(base)?;
                }

                if exp == 1 {
                    acc = acc.checked_mul(base)?;
                }

                Some(acc)
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            fn saturating_pow(self, exp: u32) -> Self {
                match self.checked_pow(exp) {
                    Some(x) => x,
                    None => Self::max_value(),
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            fn wrapping_pow(self, mut exp: u32) -> Self {
                let mut base = self;
                let mut acc: Self = 1;

                while exp > 1 {
                    if (exp & 1) == 1 {
                        acc = acc.wrapping_mul(base);
                    }
                    exp /= 2;
                    base = base.wrapping_mul(base);
                }

                if exp == 1 {
                    acc = acc.wrapping_mul(base);
                }

                acc
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            fn overflowing_pow(self, mut exp: u32) -> (Self, bool) {
                let mut base = self;
                let mut acc: Self = 1;
                let mut overflown = false;
                let mut r;

                while exp > 1 {
                    if (exp & 1) == 1 {
                        r = acc.overflowing_mul(base);
                        acc = r.0;
                        overflown |= r.1;
                    }
                    exp /= 2;
                    r = base.overflowing_mul(base);
                    base = r.0;
                    overflown |= r.1;
                }

                if exp == 1 {
                    r = acc.overflowing_mul(base);
                    acc = r.0;
                    overflown |= r.1;
                }

                (acc, overflown)
            }
        }
    )+};
}

impl_pow_for_unsigned![u8 u16 u32 u64 u128 usize];
