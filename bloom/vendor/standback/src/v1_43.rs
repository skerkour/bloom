use crate::traits::{Float, Integer};
use core::iter::FusedIterator;

pub mod f32 {
    pub const LOG10_2: f32 = 0.301029995663981195213738894724493027_f32;
    pub const LOG2_10: f32 = 3.32192809488736234787031942948939018_f32;
}

pub mod f64 {
    pub const LOG10_2: f64 = 0.301029995663981195213738894724493027_f64;
    pub const LOG2_10: f64 = 3.32192809488736234787031942948939018_f64;
}

#[inline]
pub fn once_with<A, F: FnOnce() -> A>(gen: F) -> OnceWith<F> {
    OnceWith { gen: Some(gen) }
}

#[derive(Copy, Clone, Debug)]
pub struct OnceWith<F> {
    gen: Option<F>,
}

impl<A, F: FnOnce() -> A> Iterator for OnceWith<F> {
    type Item = A;

    #[inline]
    fn next(&mut self) -> Option<A> {
        let f = self.gen.take()?;
        Some(f())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.gen.iter().size_hint()
    }
}

impl<A, F: FnOnce() -> A> DoubleEndedIterator for OnceWith<F> {
    fn next_back(&mut self) -> Option<A> {
        self.next()
    }
}

impl<A, F: FnOnce() -> A> ExactSizeIterator for OnceWith<F> {
    fn len(&self) -> usize {
        self.gen.iter().len()
    }
}

impl<A, F: FnOnce() -> A> FusedIterator for OnceWith<F> {}

pub trait float_v1_43: Float {
    const RADIX: u32;
    const MANTISSA_DIGITS: u32;
    const DIGITS: u32;
    const EPSILON: Self;
    const MIN: Self;
    const MIN_POSITIVE: Self;
    const MAX: Self;
    const MIN_EXP: i32;
    const MAX_EXP: i32;
    const MIN_10_EXP: i32;
    const MAX_10_EXP: i32;
    const NAN: Self;
    const INFINITY: Self;
    const NEG_INFINITY: Self;
}

impl float_v1_43 for f32 {
    const DIGITS: u32 = 6;
    const EPSILON: f32 = 1.19209290e-07_f32;
    const INFINITY: f32 = 1.0_f32 / 0.0_f32;
    const MANTISSA_DIGITS: u32 = 24;
    const MAX: f32 = 3.40282347e+38_f32;
    const MAX_10_EXP: i32 = 38;
    const MAX_EXP: i32 = 128;
    const MIN: f32 = -3.40282347e+38_f32;
    const MIN_10_EXP: i32 = -37;
    const MIN_EXP: i32 = -125;
    const MIN_POSITIVE: f32 = 1.17549435e-38_f32;
    const NAN: f32 = 0.0_f32 / 0.0_f32;
    const NEG_INFINITY: f32 = -1.0_f32 / 0.0_f32;
    const RADIX: u32 = 2;
}

impl float_v1_43 for f64 {
    const DIGITS: u32 = 15;
    const EPSILON: f64 = 2.2204460492503131e-16_f64;
    const INFINITY: f64 = 1.0_f64 / 0.0_f64;
    const MANTISSA_DIGITS: u32 = 53;
    const MAX: f64 = 1.7976931348623157e+308_f64;
    const MAX_10_EXP: i32 = 308;
    const MAX_EXP: i32 = 1024;
    const MIN: f64 = -1.7976931348623157e+308_f64;
    const MIN_10_EXP: i32 = -307;
    const MIN_EXP: i32 = -1021;
    const MIN_POSITIVE: f64 = 2.2250738585072014e-308_f64;
    const NAN: f64 = 0.0_f64 / 0.0_f64;
    const NEG_INFINITY: f64 = -1.0_f64 / 0.0_f64;
    const RADIX: u32 = 2;
}

pub trait int_v1_43: Integer {
    const MIN: Self;
    const MAX: Self;
}

macro_rules! impl_int_v1_43 {
    ($($signed_type:ty, $unsigned_type:ty),*) => {$(
        impl int_v1_43 for $signed_type {
            const MIN: Self = !0 ^ ((!0 as $unsigned_type) >> 1) as Self;
            const MAX: Self = !Self::MIN;
        }

        impl int_v1_43 for $unsigned_type {
            const MIN: Self = 0;
            const MAX: Self = !0;
        }
    )*}
}

impl_int_v1_43![i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize];
