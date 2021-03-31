use crate::traits::{Integer, Sealed};

pub trait int_v1_46: Integer {
    fn leading_ones(self) -> u32;
    fn trailing_ones(self) -> u32;
}

macro_rules! impl_int_v1_46 {
    ($($signed_type:ty, $unsigned_type:ty),*) => {$(
        impl int_v1_46 for $signed_type {
            #[inline]
            fn leading_ones(self) -> u32 {
                (self as $unsigned_type).leading_ones()
            }

            #[inline]
            fn trailing_ones(self) -> u32 {
                (self as $unsigned_type).trailing_ones()
            }
        }

        impl int_v1_46 for $unsigned_type {
            #[inline]
            fn leading_ones(self) -> u32 {
                (!self).leading_zeros()
            }

            #[inline]
            fn trailing_ones(self) -> u32 {
                (!self).trailing_zeros()
            }
        }
    )*};
}

impl_int_v1_46![
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize
];

pub trait Option_v1_46<T>: Sealed<Option<T>> {
    fn zip<U>(self, other: Option<U>) -> Option<(T, U)>;
}

impl<T> Option_v1_46<T> for Option<T> {
    fn zip<U>(self, other: Option<U>) -> Option<(T, U)> {
        match (self, other) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        }
    }
}
