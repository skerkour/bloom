use core::mem::{size_of, transmute};

use crate::traits::Sealed;

macro_rules! impl_int_v1_32 {
    ($(($trait:ident, $type:ty)),+) => {$(
        pub trait $trait: Sealed<$type> {
            fn to_be_bytes(self) -> [u8; size_of::<$type>()];
            fn to_le_bytes(self) -> [u8; size_of::<$type>()];
            fn to_ne_bytes(self) -> [u8; size_of::<$type>()];
            fn from_be_bytes(bytes: [u8; size_of::<$type>()]) -> Self;
            fn from_le_bytes(bytes: [u8; size_of::<$type>()]) -> Self;
            fn from_ne_bytes(bytes: [u8; size_of::<$type>()]) -> Self;
        }

        impl $trait for $type {
            #[inline]
            fn to_be_bytes(self) -> [u8; size_of::<Self>()] {
                self.to_be().to_ne_bytes()
            }

            #[inline]
            fn to_le_bytes(self) -> [u8; size_of::<Self>()] {
                self.to_le().to_ne_bytes()
            }

            #[inline]
            fn to_ne_bytes(self) -> [u8; size_of::<Self>()] {
                unsafe { transmute(self) }
            }

            #[inline]
            fn from_be_bytes(bytes: [u8; size_of::<Self>()]) -> Self {
                Self::from_be(Self::from_ne_bytes(bytes))
            }

            #[inline]
            fn from_le_bytes(bytes: [u8; size_of::<Self>()]) -> Self {
                Self::from_le(Self::from_ne_bytes(bytes))
            }

            #[inline]
            fn from_ne_bytes(bytes: [u8; size_of::<Self>()]) -> Self {
                unsafe { transmute(bytes) }
            }
        }
    )+};
}

impl_int_v1_32![
    (u8_v1_32, u8),
    (u16_v1_32, u16),
    (u32_v1_32, u32),
    (u64_v1_32, u64),
    (u128_v1_32, u128),
    (usize_v1_32, usize),
    (i8_v1_32, i8),
    (i16_v1_32, i16),
    (i32_v1_32, i32),
    (i64_v1_32, i64),
    (i128_v1_32, i128),
    (isize_v1_32, isize)
];
