use crate::traits::Sealed;
#[cfg(__standback_before_1_32)]
use crate::v1_32::{u32_v1_32, u64_v1_32};
use core::ops::DerefMut;

#[cfg(feature = "std")]
use core::ptr;

pub trait Option_v1_40<T: DerefMut>: Sealed<Option<T>> {
    fn as_deref_mut(&mut self) -> Option<&mut T::Target>;
    fn as_deref(&self) -> Option<&T::Target>;
}

impl<T: DerefMut> Option_v1_40<T> for Option<T> {
    fn as_deref_mut(&mut self) -> Option<&mut T::Target> {
        self.as_mut().map(|t| t.deref_mut())
    }

    fn as_deref(&self) -> Option<&T::Target> {
        self.as_ref().map(|t| t.deref())
    }
}

pub trait Option_v1_40_<T>: Sealed<Option<Option<T>>> {
    fn flatten(self) -> Option<T>;
}

impl<T> Option_v1_40_<T> for Option<Option<T>> {
    fn flatten(self) -> Option<T> {
        self.and_then(crate::convert::identity)
    }
}

pub trait f32_v1_40: Sealed<f32> {
    fn to_be_bytes(self) -> [u8; 4];
    fn to_le_bytes(self) -> [u8; 4];
    fn to_ne_bytes(self) -> [u8; 4];
    fn from_be_bytes(bytes: [u8; 4]) -> Self;
    fn from_le_bytes(bytes: [u8; 4]) -> Self;
    fn from_ne_bytes(bytes: [u8; 4]) -> Self;
}

impl f32_v1_40 for f32 {
    #[inline]
    fn to_be_bytes(self) -> [u8; 4] {
        self.to_bits().to_be_bytes()
    }

    #[inline]
    fn to_le_bytes(self) -> [u8; 4] {
        self.to_bits().to_le_bytes()
    }

    #[inline]
    fn to_ne_bytes(self) -> [u8; 4] {
        self.to_bits().to_ne_bytes()
    }

    #[inline]
    fn from_be_bytes(bytes: [u8; 4]) -> Self {
        Self::from_bits(u32::from_be_bytes(bytes))
    }

    #[inline]
    fn from_le_bytes(bytes: [u8; 4]) -> Self {
        Self::from_bits(u32::from_le_bytes(bytes))
    }

    #[inline]
    fn from_ne_bytes(bytes: [u8; 4]) -> Self {
        Self::from_bits(u32::from_ne_bytes(bytes))
    }
}

pub trait f64_v1_40: Sealed<f64> {
    fn to_be_bytes(self) -> [u8; 8];
    fn to_le_bytes(self) -> [u8; 8];
    fn to_ne_bytes(self) -> [u8; 8];
    fn from_be_bytes(bytes: [u8; 8]) -> Self;
    fn from_le_bytes(bytes: [u8; 8]) -> Self;
    fn from_ne_bytes(bytes: [u8; 8]) -> Self;
}

impl f64_v1_40 for f64 {
    #[inline]
    fn to_be_bytes(self) -> [u8; 8] {
        self.to_bits().to_be_bytes()
    }

    #[inline]
    fn to_le_bytes(self) -> [u8; 8] {
        self.to_bits().to_le_bytes()
    }

    #[inline]
    fn to_ne_bytes(self) -> [u8; 8] {
        self.to_bits().to_ne_bytes()
    }

    #[inline]
    fn from_be_bytes(bytes: [u8; 8]) -> Self {
        Self::from_bits(u64::from_be_bytes(bytes))
    }

    #[inline]
    fn from_le_bytes(bytes: [u8; 8]) -> Self {
        Self::from_bits(u64::from_le_bytes(bytes))
    }

    #[inline]
    fn from_ne_bytes(bytes: [u8; 8]) -> Self {
        Self::from_bits(u64::from_ne_bytes(bytes))
    }
}

pub fn take<T: Default>(dest: &mut T) -> T {
    core::mem::replace(dest, T::default())
}

#[cfg(feature = "std")]
pub trait slice_v1_40<T>: Sealed<[T]> {
    fn repeat(&self, n: usize) -> Vec<T>
    where
        T: Copy;
}

#[cfg(feature = "std")]
impl<T: Copy> slice_v1_40<T> for [T] {
    fn repeat(&self, n: usize) -> Vec<T> {
        if n == 0 {
            return Vec::new();
        }

        // If `n` is larger than zero, it can be split as
        // `n = 2^expn + rem (2^expn > rem, expn >= 0, rem >= 0)`.
        // `2^expn` is the number represented by the leftmost '1' bit of `n`,
        // and `rem` is the remaining part of `n`.

        // Using `Vec` to access `set_len()`.
        let mut buf = Vec::with_capacity(self.len().checked_mul(n).expect("capacity overflow"));

        // `2^expn` repetition is done by doubling `buf` `expn`-times.
        buf.extend(self);
        {
            let mut m = n >> 1;
            // If `m > 0`, there are remaining bits up to the leftmost '1'.
            while m > 0 {
                // `buf.extend(buf)`:
                unsafe {
                    ptr::copy_nonoverlapping(
                        buf.as_ptr(),
                        (buf.as_mut_ptr() as *mut T).add(buf.len()),
                        buf.len(),
                    );
                    // `buf` has capacity of `self.len() * n`.
                    let buf_len = buf.len();
                    buf.set_len(buf_len * 2);
                }

                m >>= 1;
            }
        }

        // `rem` (`= n - 2^expn`) repetition is done by copying
        // first `rem` repetitions from `buf` itself.
        let rem_len = self.len() * n - buf.len(); // `self.len() * rem`
        if rem_len > 0 {
            // `buf.extend(buf[0 .. rem_len])`:
            unsafe {
                // This is non-overlapping since `2^expn > rem`.
                ptr::copy_nonoverlapping(
                    buf.as_ptr(),
                    (buf.as_mut_ptr() as *mut T).add(buf.len()),
                    rem_len,
                );
                // `buf.len() + rem_len` equals to `buf.capacity()` (`= self.len() * n`).
                let buf_cap = buf.capacity();
                buf.set_len(buf_cap);
            }
        }
        buf
    }
}
