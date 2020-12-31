use crate::traits::{Float, Integer, Sealed};
use core::time::Duration;

pub trait ConstPtr_v1_38<T>: Sealed<*const T> {
    fn cast<U>(self) -> *const U;
}

impl<T> ConstPtr_v1_38<T> for *const T {
    #[inline]
    fn cast<U>(self) -> *const U {
        self as _
    }
}

pub trait MutPtr_v1_38<T>: Sealed<*mut T> {
    fn cast<U>(self) -> *mut U;
}

impl<T> MutPtr_v1_38<T> for *mut T {
    #[inline]
    fn cast<U>(self) -> *mut U {
        self as _
    }
}

pub trait Duration_v1_38: Sealed<Duration> {
    fn as_secs_f32(&self) -> f32;
    fn as_secs_f64(&self) -> f64;
    fn div_f32(&self, rhs: f32) -> Self;
    fn div_f64(&self, rhs: f64) -> Self;
    fn from_secs_f32(secs: f32) -> Self;
    fn from_secs_f64(secs: f64) -> Self;
    fn mul_f32(&self, rhs: f32) -> Self;
    fn mul_f64(&self, rhs: f64) -> Self;
}

impl Duration_v1_38 for Duration {
    #[inline]
    fn as_secs_f32(&self) -> f32 {
        (self.as_secs() as f32) + (self.subsec_nanos() as f32) / 1_000_000_000.
    }

    #[inline]
    fn as_secs_f64(&self) -> f64 {
        (self.as_secs() as f64) + (self.subsec_nanos() as f64) / 1_000_000_000.
    }

    #[inline]
    fn div_f32(&self, rhs: f32) -> Self {
        Self::from_secs_f32(self.as_secs_f32() / rhs)
    }

    #[inline]
    fn div_f64(&self, rhs: f64) -> Self {
        Self::from_secs_f64(self.as_secs_f64() / rhs)
    }

    #[inline]
    fn from_secs_f32(secs: f32) -> Self {
        const MAX_NANOS_F32: f32 = ((u64::max_value() as u128 + 1) * 1_000_000_000) as f32;
        let nanos = secs * 1_000_000_000.;
        if !nanos.is_finite() {
            panic!("got non-finite value when converting float to duration");
        }
        if nanos >= MAX_NANOS_F32 {
            panic!("overflow when converting float to duration");
        }
        if nanos < 0.0 {
            panic!("underflow when converting float to duration");
        }
        let nanos = nanos as u128;
        Self::new(
            (nanos / 1_000_000_000) as u64,
            (nanos % 1_000_000_000) as u32,
        )
    }

    #[inline]
    fn from_secs_f64(secs: f64) -> Self {
        const MAX_NANOS_F64: f64 = ((u64::max_value() as u128 + 1) * 1_000_000_000) as f64;
        let nanos = secs * 1_000_000_000.;
        if !nanos.is_finite() {
            panic!("got non-finite value when converting float to duration");
        }
        if nanos >= MAX_NANOS_F64 {
            panic!("overflow when converting float to duration");
        }
        if nanos < 0.0 {
            panic!("underflow when converting float to duration");
        }
        let nanos = nanos as u128;
        Self::new(
            (nanos / 1_000_000_000) as u64,
            (nanos % 1_000_000_000) as u32,
        )
    }

    #[inline]
    fn mul_f32(&self, rhs: f32) -> Self {
        Self::from_secs_f32(rhs * self.as_secs_f32())
    }

    #[inline]
    fn mul_f64(&self, rhs: f64) -> Self {
        Self::from_secs_f64(rhs * self.as_secs_f64())
    }
}

pub trait Euclid_v1_38: Integer {
    fn rem_euclid(self, rhs: Self) -> Self;
    fn checked_rem_euclid(self, rhs: Self) -> Option<Self>;
    fn wrapping_rem_euclid(self, rhs: Self) -> Self;
    fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool);
    fn div_euclid(self, rhs: Self) -> Self;
    fn checked_div_euclid(self, rhs: Self) -> Option<Self>;
    fn wrapping_div_euclid(self, rhs: Self) -> Self;
    fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool);
}

macro_rules! impl_euclid_for_signed {
    ($($type:ty)+) => {$(
        impl Euclid_v1_38 for $type {
            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            fn rem_euclid(self, rhs: Self) -> Self {
                let r = self % rhs;
                if r < 0 {
                    if rhs < 0 {
                        r - rhs
                    } else {
                        r + rhs
                    }
                } else {
                    r
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
                if rhs == 0 || (self == Self::min_value() && rhs == -1) {
                    None
                } else {
                    Some(self.rem_euclid(rhs))
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            fn wrapping_rem_euclid(self, rhs: Self) -> Self {
                self.overflowing_rem_euclid(rhs).0
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
                if self == Self::min_value() && rhs == -1 {
                    (0, true)
                } else {
                    (self.rem_euclid(rhs), false)
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            fn div_euclid(self, rhs: Self) -> Self {
                let q = self / rhs;
                if self % rhs < 0 {
                    return if rhs > 0 { q - 1 } else { q + 1 };
                }
                q
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
                if rhs == 0 || (self == Self::min_value() && rhs == -1) {
                    None
                } else {
                    Some(self.div_euclid(rhs))
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            fn wrapping_div_euclid(self, rhs: Self) -> Self {
                self.overflowing_div_euclid(rhs).0
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
                if self == Self::min_value() && rhs == -1 {
                    (self, true)
                } else {
                    (self.div_euclid(rhs), false)
                }
            }
        }
    )+};
}

impl_euclid_for_signed![i8 i16 i32 i64 i128 isize];

macro_rules! impl_euclid_for_unsigned {
    ($($type:ty)+) => {$(
        impl Euclid_v1_38 for $type {
            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            fn rem_euclid(self, rhs: Self) -> Self {
                self % rhs
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
                if rhs == 0 {
                    None
                } else {
                    Some(self.rem_euclid(rhs))
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            fn wrapping_rem_euclid(self, rhs: Self) -> Self {
                self % rhs
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
                (self % rhs, false)
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            fn div_euclid(self, rhs: Self) -> Self {
                self / rhs
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
                if rhs == 0 {
                    None
                } else {
                    Some(self.div_euclid(rhs))
                }
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            fn wrapping_div_euclid(self, rhs: Self) -> Self {
                self / rhs
            }

            #[must_use = "this returns the result of the operation, without modifying the original"]
            #[inline]
            fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
                (self / rhs, false)
            }
        }
    )+};
}

impl_euclid_for_unsigned![u8 u16 u32 u64 u128 usize];

pub trait EuclidFloat_v1_38: Float {
    fn rem_euclid(self, rhs: Self) -> Self;
    fn div_euclid(self, rhs: Self) -> Self;
}

#[cfg(feature = "std")]
impl EuclidFloat_v1_38 for f32 {
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn rem_euclid(self, rhs: f32) -> f32 {
        let r = self % rhs;
        if r < 0.0 {
            r + rhs.abs()
        } else {
            r
        }
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn div_euclid(self, rhs: f32) -> f32 {
        let q = (self / rhs).trunc();
        if self % rhs < 0.0 {
            return if rhs > 0.0 { q - 1.0 } else { q + 1.0 };
        }
        q
    }
}

#[cfg(feature = "std")]
impl EuclidFloat_v1_38 for f64 {
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn rem_euclid(self, rhs: f64) -> f64 {
        let r = self % rhs;
        if r < 0.0 {
            r + rhs.abs()
        } else {
            r
        }
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline]
    fn div_euclid(self, rhs: f64) -> f64 {
        let q = (self / rhs).trunc();
        if self % rhs < 0.0 {
            return if rhs > 0.0 { q - 1.0 } else { q + 1.0 };
        }
        q
    }
}
