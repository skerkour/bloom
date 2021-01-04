#![allow(non_camel_case_types, unstable_name_collisions)]
#![cfg_attr(not(feature = "std"), no_std)]

//! Standback backports a number of methods, structs, and macros that have been
//! stabilized in the Rust standard library since 1.31.0. This allows crate
//! authors to depend on Standback rather than forcing downstream users to
//! upgrade their compiler (or not use the new feature at all).
//!
//! Due to a variety of restrictions in the Rust, it is not possible to
//! implement everything that has been stabilized.
//!
//! # Usage
//!
//! If you are using methods on already-existing structs, you should use the
//! following:
//!
//! ```rust,no_run
//! use standback::prelude::*;
//! ```
//!
//! Additionally, if you are using newly stabilized structs, types, or anything
//! else that would normally have to be imported, use `standback` instead of
//! `std`:
//!
//! ```rust,no_run
//! use standback::mem::take;
//! ```
//!
//! It is _highly_ recommended to use `#![allow(unstable_name_collisions)]`, as
//! that's the whole point of this crate. Just be extra-careful to not do it for
//! anything that _can't_ be backported.
//!
//! # `#![no_std]` support
//!
//! By default, there standard library is used where necessary. If support for
//! `#![no_std]` is required, use `default-features = false`.
//!
//! An allocator is not required for any backported item. If any require an
//! allocator in the future, it will be gated under an `alloc` feature.
//!
//! # Methods on existing structs
//!
//! The following methods and constants are available via the prelude:
//!
//! ```rust,ignore
//! // 1.49
//! slice::select_nth_unstable
//! slice::select_nth_unstable_by
//! slice::select_nth_unstable_by_key
//!
//! // 1.48
//! slice::as_ptr_range
//! slice::as_mut_ptr_range
//!
//! // 1.47
//! Range::is_empty
//! Result::as_deref
//! Result::as_deref_mut
//! Vec::leak
//! f32::TAU
//! f64::TAU
//!
//! // 1.46
//! i8::leading_ones
//! i8::trailing_ones
//! i16::leading_ones
//! i16::trailing_ones
//! i32::leading_ones
//! i32::trailing_ones
//! i64::leading_ones
//! i64::trailing_ones
//! i128::leading_ones
//! i128::trailing_ones
//! isize::leading_ones
//! isize::trailing_ones
//! u8::leading_ones
//! u8::trailing_ones
//! u16::leading_ones
//! u16::trailing_ones
//! u32::leading_ones
//! u32::trailing_ones
//! u64::leading_ones
//! u64::trailing_ones
//! u128::leading_ones
//! u128::trailing_ones
//! usize::leading_ones
//! usize::trailing_ones
//! Option::zip
//!
//! // 1.45
//! i8::saturating_abs
//! i8::saturating_neg
//! i16::saturating_abs
//! i16::saturating_neg
//! i32::saturating_abs
//! i32::saturating_neg
//! i64::saturating_abs
//! i64::saturating_neg
//! i128::saturating_abs
//! i128::saturating_neg
//! isize::saturating_abs
//! isize::saturating_neg
//!
//! // 1.44
//! PathBuf::with_capacity
//! PathBuf::capacity
//! PathBuf::clear
//! PathBuf::reserve
//! PathBuf::reserve_exact
//! PathBuf::shrink_to_fit
//! Layout::align_to
//! Layout::pad_to_align
//! Layout::array
//! Layout::extend
//! f32::to_int_unchecked
//! f64::to_int_unchecked
//!
//! // 1.43
//! f32::RADIX
//! f32::MANTISSA_DIGITS
//! f32::DIGITS
//! f32::EPSILON
//! f32::MIN
//! f32::MIN_POSITIVE
//! f32::MAX
//! f32::MIN_EXP
//! f32::MAX_EXP
//! f32::MIN_10_EXP
//! f32::MAX_10_EXP
//! f32::NAN
//! f32::INFINITY
//! f32::NEG_INFINITY
//! f64::RADIX
//! f64::MANTISSA_DIGITS
//! f64::DIGITS
//! f64::EPSILON
//! f64::MIN
//! f64::MIN_POSITIVE
//! f64::MAX
//! f64::MIN_EXP
//! f64::MAX_EXP
//! f64::MIN_10_EXP
//! f64::MAX_10_EXP
//! f64::NAN
//! f64::INFINITY
//! f64::NEG_INFINITY
//! u8::MIN
//! u8::MAX
//! u16::MIN
//! u16::MAX
//! u32::MIN
//! u32::MAX
//! u64::MIN
//! u64::MAX
//! u128::MIN
//! u128::MAX
//! usize::MIN
//! usize::MAX
//! i8::MIN
//! i8::MAX
//! i16::MIN
//! i16::MAX
//! i32::MIN
//! i32::MAX
//! i64::MIN
//! i64::MAX
//! i128::MIN
//! i128::MAX
//! isize::MIN
//! isize::MAX
//!
//! // 1.42
//! CondVar::wait_while
//! CondVar::wait_timeout_while
//! ManuallyDrop::take
//!
//! // 1.41
//! Result::map_or
//! Result::map_or_else
//!
//! // 1.40
//! Option::as_deref
//! Option::as_deref_mut
//! f32::to_be_bytes
//! f32::to_le_bytes
//! f32::to_ne_bytes
//! f64::to_be_bytes
//! f64::to_le_bytes
//! f64::to_ne_bytes
//! f32::from_be_bytes
//! f32::from_le_bytes
//! f32::from_ne_bytes
//! f64::from_be_bytes
//! f64::from_le_bytes
//! f64::from_ne_bytes
//! slice::repeat
//!
//! // 1.39
//! // None :(
//!
//! // 1.38
//! <*const T>::cast
//! <*mut T>::cast
//! Duration::as_secs_f32
//! Duration::as_secs_f64
//! Duration::div_f32
//! Duration::div_f64
//! Duration::from_secs_f32
//! Duration::from_secs_f64
//! Duration::mul_f32
//! Duration::mul_f64
//! i8::rem_euclid
//! i8::checked_rem_euclid
//! i8::wrapping_rem_euclid
//! i8::overflowing_rem_euclid
//! i8::div_euclid
//! i8::checked_div_euclid
//! i8::wrapping_div_euclid
//! i8::overflowing_div_euclid
//! i16::rem_euclid
//! i16::checked_rem_euclid
//! i16::wrapping_rem_euclid
//! i16::overflowing_rem_euclid
//! i16::div_euclid
//! i16::checked_div_euclid
//! i16::wrapping_div_euclid
//! i16::overflowing_div_euclid
//! i32::rem_euclid
//! i32::checked_rem_euclid
//! i32::wrapping_rem_euclid
//! i32::overflowing_rem_euclid
//! i32::div_euclid
//! i32::checked_div_euclid
//! i32::wrapping_div_euclid
//! i32::overflowing_div_euclid
//! i64::rem_euclid
//! i64::checked_rem_euclid
//! i64::wrapping_rem_euclid
//! i64::overflowing_rem_euclid
//! i64::div_euclid
//! i64::checked_div_euclid
//! i64::wrapping_div_euclid
//! i64::overflowing_div_euclid
//! i128::rem_euclid
//! i128::checked_rem_euclid
//! i128::wrapping_rem_euclid
//! i128::overflowing_rem_euclid
//! i128::div_euclid
//! i128::checked_div_euclid
//! i128::wrapping_div_euclid
//! i128::overflowing_div_euclid
//! isize::rem_euclid
//! isize::checked_rem_euclid
//! isize::wrapping_rem_euclid
//! isize::overflowing_rem_euclid
//! isize::div_euclid
//! isize::checked_div_euclid
//! isize::wrapping_div_euclid
//! isize::overflowing_div_euclid
//! u8::rem_euclid
//! u8::checked_rem_euclid
//! u8::wrapping_rem_euclid
//! u8::overflowing_rem_euclid
//! u8::div_euclid
//! u8::checked_div_euclid
//! u8::wrapping_div_euclid
//! u8::overflowing_div_euclid
//! u16::rem_euclid
//! u16::checked_rem_euclid
//! u16::wrapping_rem_euclid
//! u16::overflowing_rem_euclid
//! u16::div_euclid
//! u16::checked_div_euclid
//! u16::wrapping_div_euclid
//! u16::overflowing_div_euclid
//! u32::rem_euclid
//! u32::checked_rem_euclid
//! u32::wrapping_rem_euclid
//! u32::overflowing_rem_euclid
//! u32::div_euclid
//! u32::checked_div_euclid
//! u32::wrapping_div_euclid
//! u32::overflowing_div_euclid
//! u64::rem_euclid
//! u64::checked_rem_euclid
//! u64::wrapping_rem_euclid
//! u64::overflowing_rem_euclid
//! u64::div_euclid
//! u64::checked_div_euclid
//! u64::wrapping_div_euclid
//! u64::overflowing_div_euclid
//! u128::rem_euclid
//! u128::checked_rem_euclid
//! u128::wrapping_rem_euclid
//! u128::overflowing_rem_euclid
//! u128::div_euclid
//! u128::checked_div_euclid
//! u128::wrapping_div_euclid
//! u128::overflowing_div_euclid
//! usize::rem_euclid
//! usize::checked_rem_euclid
//! usize::wrapping_rem_euclid
//! usize::overflowing_rem_euclid
//! usize::div_euclid
//! usize::checked_div_euclid
//! usize::wrapping_div_euclid
//! usize::overflowing_div_euclid
//! f32::rem_euclid
//! f32::div_euclid
//! f64::rem_euclid
//! f64::div_euclid
//!
//! // 1.37
//! Cell::from_mut
//! Cell<[T]>::as_slice_of_cells
//! DoubleEndedIterator::nth_back
//! Option::xor
//! slice::copy_within
//!
//! // 1.36
//! Iterator::copied
//! mem::MaybeUninit
//! task::Context
//! task::RawWaker
//! task::RawWakerVTable
//! task::Waker
//! task::Poll
//!
//! // 1.35
//! RefCell::replace_with
//! ptr::hash
//! Range::contains
//! RangeFrom::contains
//! RangeTo::contains
//! RangeInclusive::contains
//! RangeToInclusive::contains
//! Option::copied
//!
//! // 1.34
//! slice::sort_by_cached_key
//! i8::checked_pow
//! i8::saturating_pow
//! i8::wrapping_pow
//! i8::overflowing_pow
//! i16::checked_pow
//! i16::saturating_pow
//! i16::wrapping_pow
//! i16::overflowing_pow
//! i32::checked_pow
//! i32::saturating_pow
//! i32::wrapping_pow
//! i32::overflowing_pow
//! i64::checked_pow
//! i64::saturating_pow
//! i64::wrapping_pow
//! i64::overflowing_pow
//! i128::checked_pow
//! i128::saturating_pow
//! i128::wrapping_pow
//! i128::overflowing_pow
//! isize::checked_pow
//! isize::saturating_pow
//! isize::wrapping_pow
//! isize::overflowing_pow
//! u8::checked_pow
//! u8::saturating_pow
//! u8::wrapping_pow
//! u8::overflowing_pow
//! u16::checked_pow
//! u16::saturating_pow
//! u16::wrapping_pow
//! u16::overflowing_pow
//! u32::checked_pow
//! u32::saturating_pow
//! u32::wrapping_pow
//! u32::overflowing_pow
//! u64::checked_pow
//! u64::saturating_pow
//! u64::wrapping_pow
//! u64::overflowing_pow
//! u128::checked_pow
//! u128::saturating_pow
//! u128::wrapping_pow
//! u128::overflowing_pow
//! usize::checked_pow
//! usize::saturating_pow
//! usize::wrapping_pow
//! usize::overflowing_pow
//!
//! // 1.33
//! os::unix::fs::FileExt::read_exact_at
//! os::unix::fs::FileExt::write_all_at
//! Option::transpose
//! Result::transpose
//! VecDeque::resize_with
//! Duration::as_millis
//! Duration::as_micros
//! Duration::as_nanos
//!
//! // 1.32
//! i8::to_be_bytes
//! i8::to_le_bytes
//! i8::to_ne_bytes
//! i8::from_be_bytes
//! i8::from_le_bytes
//! i8::from_ne_bytes
//! i16::to_be_bytes
//! i16::to_le_bytes
//! i16::to_ne_bytes
//! i16::from_be_bytes
//! i16::from_le_bytes
//! i16::from_ne_bytes
//! i32::to_be_bytes
//! i32::to_le_bytes
//! i32::to_ne_bytes
//! i32::from_be_bytes
//! i32::from_le_bytes
//! i32::from_ne_bytes
//! i64::to_be_bytes
//! i64::to_le_bytes
//! i64::to_ne_bytes
//! i64::from_be_bytes
//! i64::from_le_bytes
//! i64::from_ne_bytes
//! i128::to_be_bytes
//! i128::to_le_bytes
//! i128::to_ne_bytes
//! i128::from_be_bytes
//! i128::from_le_bytes
//! i128::from_ne_bytes
//! isize::to_be_bytes
//! isize::to_le_bytes
//! isize::to_ne_bytes
//! isize::from_be_bytes
//! isize::from_le_bytes
//! isize::from_ne_bytes
//! u8::to_be_bytes
//! u8::to_le_bytes
//! u8::to_ne_bytes
//! u8::from_be_bytes
//! u8::from_le_bytes
//! u8::from_ne_bytes
//! u16::to_be_bytes
//! u16::to_le_bytes
//! u16::to_ne_bytes
//! u16::from_be_bytes
//! u16::from_le_bytes
//! u16::from_ne_bytes
//! u32::to_be_bytes
//! u32::to_le_bytes
//! u32::to_ne_bytes
//! u32::from_be_bytes
//! u32::from_le_bytes
//! u32::from_ne_bytes
//! u64::to_be_bytes
//! u64::to_le_bytes
//! u64::to_ne_bytes
//! u64::from_be_bytes
//! u64::from_le_bytes
//! u64::from_ne_bytes
//! u128::to_be_bytes
//! u128::to_le_bytes
//! u128::to_ne_bytes
//! u128::from_be_bytes
//! u128::from_le_bytes
//! u128::from_ne_bytes
//! usize::to_be_bytes
//! usize::to_le_bytes
//! usize::to_ne_bytes
//! usize::from_be_bytes
//! usize::from_le_bytes
//! usize::from_ne_bytes
//! ```
//!
//! # Other APIs implemented
//!
//! ```rust,ignore
//! future::pending // 1.48, requires rustc 1.36
//! future::ready // 1.48, requires rustc 1.36
//! char::UNICODE_VERSION // 1.45
//! f32::LOG10_2 // 1.43
//! f32::LOG2_10 // 1.43
//! f64::LOG10_2 // 1.43
//! f64::LOG2_10 // 1.43
//! iter::once_with // 1.43
//! mem::take // 1.40
//! iterator::Copied // 1.36
//! array::TryFromSliceError // 1.36
//! iter::from_fn // 1.34
//! iter::successors // 1.34
//! convert::TryFrom // 1.34
//! convert::TryInto // 1.34
//! num::TryFromIntError // 1.34
//! convert::identity // 1.33
//! pin::Pin // 1.33
//! marker::Unpin // 1.33
//! ```
//!
//! # Macros
//!
//! Macros should not be imported directly, but rather through the prelude.
//!
//! ```rust,ignore
//! todo! // 1.39
//! matches! // 1.42
//! ```

#![deny(rust_2018_idioms, unused_qualifications)]

// A few traits to make sealing other traits simpler.
mod traits {
    pub trait Sealed<T: ?Sized> {}
    impl<T: ?Sized> Sealed<T> for T {}

    macro_rules! impl_trait_for_all {
        ($trait:ident => $($type:ty)+) => {$(
            impl $trait for $type {}
        )+};
    }

    pub trait Integer: Sized {}
    impl_trait_for_all!(Integer => i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

    pub trait SignedInteger {}
    impl_trait_for_all!(SignedInteger => i8 i16 i32 i64 i128 isize);

    pub trait UnsignedInteger {}
    impl_trait_for_all!(UnsignedInteger => u8 u16 u32 u64 u128 usize);

    pub trait Float {}
    impl_trait_for_all!(Float => f32 f64);
}

#[cfg(__standback_before_1_32)]
mod v1_32;
#[cfg(__standback_before_1_33)]
mod v1_33;
#[cfg(__standback_before_1_34)]
mod v1_34;
#[cfg(__standback_before_1_35)]
mod v1_35;
#[cfg(__standback_before_1_36)]
mod v1_36;
#[cfg(__standback_before_1_37)]
mod v1_37;
#[cfg(__standback_before_1_38)]
mod v1_38;
#[cfg(__standback_before_1_40)]
mod v1_40;
#[cfg(__standback_before_1_41)]
mod v1_41;
#[cfg(__standback_before_1_42)]
mod v1_42;
#[cfg(__standback_before_1_43)]
mod v1_43;
#[cfg(__standback_before_1_44)]
mod v1_44;
#[cfg(__standback_before_1_45)]
mod v1_45;
#[cfg(__standback_before_1_46)]
mod v1_46;
#[cfg(__standback_before_1_47)]
mod v1_47;
#[cfg(__standback_before_1_48)]
mod v1_48;
#[cfg(__standback_before_1_49)]
mod v1_49;

pub mod prelude {
    #[cfg(__standback_before_1_42)]
    pub use crate::matches;
    #[cfg(__standback_before_1_32)]
    pub use crate::v1_32::{
        i128_v1_32, i16_v1_32, i32_v1_32, i64_v1_32, i8_v1_32, isize_v1_32, u128_v1_32, u16_v1_32,
        u32_v1_32, u64_v1_32, u8_v1_32, usize_v1_32,
    };
    #[cfg(all(feature = "std", __standback_before_1_33, target_family = "unix"))]
    pub use crate::v1_33::UnixFileExt_v1_33;
    #[cfg(all(feature = "std", __standback_before_1_33))]
    pub use crate::v1_33::VecDeque_v1_33;
    #[cfg(__standback_before_1_33)]
    pub use crate::v1_33::{Duration_v1_33, Option_v1_33, Result_v1_33};
    #[cfg(__standback_before_1_34)]
    pub use crate::v1_34::{Pow_v1_34, Slice_v1_34};
    #[cfg(__standback_before_1_35)]
    pub use crate::v1_35::{Option_v1_35, RangeBounds_v1_35, RefCell_v1_35};
    #[cfg(__standback_before_1_36)]
    pub use crate::v1_36::{str_v1_36, Iterator_v1_36};
    #[cfg(__standback_before_1_37)]
    pub use crate::v1_37::{
        Cell_v1_37, Cell_v1_37_, DoubleEndedIterator_v1_37, Option_v1_37, Slice_v1_37,
    };
    #[cfg(__standback_before_1_38)]
    pub use crate::v1_38::{
        ConstPtr_v1_38, Duration_v1_38, EuclidFloat_v1_38, Euclid_v1_38, MutPtr_v1_38,
    };
    #[cfg(all(feature = "std", __standback_before_1_40))]
    pub use crate::v1_40::slice_v1_40;
    #[cfg(__standback_before_1_40)]
    pub use crate::v1_40::{f32_v1_40, f64_v1_40, Option_v1_40, Option_v1_40_};
    #[cfg(__standback_before_1_41)]
    pub use crate::v1_41::Result_v1_41;
    #[cfg(all(__standback_before_1_42, feature = "std"))]
    pub use crate::v1_42::Condvar_v1_42;
    #[cfg(__standback_before_1_42)]
    pub use crate::v1_42::ManuallyDrop_v1_42;
    #[cfg(__standback_before_1_43)]
    pub use crate::v1_43::{float_v1_43, int_v1_43};
    #[cfg(__standback_before_1_44)]
    pub use crate::v1_44::Layout_v1_44;
    #[cfg(all(__standback_before_1_44, feature = "std"))]
    pub use crate::v1_44::PathBuf_v1_44;
    #[cfg(__standback_before_1_45)]
    pub use crate::v1_45::int_v1_45;
    #[cfg(__standback_before_1_46)]
    pub use crate::v1_46::{int_v1_46, Option_v1_46};
    #[cfg(all(feature = "std", __standback_before_1_47))]
    pub use crate::v1_47::Vec_v1_47;
    #[cfg(__standback_before_1_47)]
    pub use crate::v1_47::{Range_v1_47, Result_v1_47};
    #[cfg(__standback_before_1_48)]
    pub use crate::v1_48::Slice_v1_48;
    #[cfg(__standback_before_1_49)]
    pub use crate::v1_49::Slice_v1_49;
    #[cfg(__standback_before_1_39)]
    pub use core::unimplemented as todo;
}

pub mod mem {
    #[cfg(__standback_before_1_40)]
    pub use crate::v1_40::take;
    #[cfg(__standback_since_1_40)]
    pub use core::mem::take;

    #[cfg(__standback_before_1_36)]
    pub use crate::v1_36::MaybeUninit;
    #[cfg(__standback_since_1_36)]
    pub use core::mem::MaybeUninit;
}
pub mod convert {
    #[cfg(__standback_before_1_33)]
    pub use crate::v1_33::identity;
    #[cfg(__standback_since_1_33)]
    pub use core::convert::identity;

    #[cfg(__standback_before_1_34)]
    pub use crate::v1_34::Infallible;
    #[cfg(__standback_since_1_34)]
    pub use core::convert::Infallible;

    #[cfg(__standback_before_1_34)]
    pub use crate::v1_34::{TryFrom, TryInto};
    #[cfg(__standback_since_1_34)]
    pub use core::convert::{TryFrom, TryInto};
}
pub mod num {
    #[cfg(__standback_before_1_34)]
    pub use crate::v1_34::TryFromIntError;
    #[cfg(__standback_since_1_34)]
    pub use core::num::TryFromIntError;
}
pub mod iter {
    #[cfg(__standback_before_1_36)]
    pub use crate::v1_36::Copied;
    #[cfg(__standback_since_1_36)]
    pub use core::iter::Copied;

    #[cfg(__standback_before_1_34)]
    pub use crate::v1_34::{from_fn, successors};
    #[cfg(__standback_since_1_34)]
    pub use core::iter::{from_fn, successors};

    #[cfg(__standback_before_1_43)]
    pub use crate::v1_43::{once_with, OnceWith};
    #[cfg(__standback_since_1_43)]
    pub use core::iter::{once_with, OnceWith};
}
pub mod marker {
    #[cfg(__standback_before_1_33)]
    pub use crate::v1_33::Unpin;
    #[cfg(__standback_since_1_33)]
    pub use core::marker::Unpin;
}
pub mod pin {
    #[cfg(__standback_before_1_33)]
    pub use crate::v1_33::Pin;
    #[cfg(__standback_since_1_33)]
    pub use core::pin::Pin;
}
pub mod task {
    #[cfg(__standback_before_1_36)]
    pub use crate::v1_36::{Context, Poll, RawWaker, RawWakerVTable, Waker};
    #[cfg(__standback_since_1_36)]
    pub use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
}
pub mod ptr {
    #[cfg(__standback_before_1_35)]
    pub use crate::v1_35::hash;
    #[cfg(__standback_since_1_35)]
    pub use core::ptr::hash;
}
pub mod array {
    #[cfg(__standback_before_1_36)]
    pub use crate::v1_36::TryFromSliceError;
    #[cfg(__standback_since_1_36)]
    pub use core::array::TryFromSliceError;
}
pub mod f32 {
    pub mod consts {
        #[cfg(__standback_before_1_43)]
        pub use crate::v1_43::f32::{LOG10_2, LOG2_10};
        #[cfg(__standback_since_1_43)]
        pub use core::f32::consts::{LOG10_2, LOG2_10};

        #[cfg(__standback_before_1_47)]
        pub use crate::v1_47::f32::TAU;
        #[cfg(__standback_since_1_47)]
        pub use core::f32::consts::TAU;
    }
}
pub mod f64 {
    pub mod consts {
        #[cfg(__standback_before_1_43)]
        pub use crate::v1_43::f64::{LOG10_2, LOG2_10};
        #[cfg(__standback_since_1_43)]
        pub use core::f64::consts::{LOG10_2, LOG2_10};

        #[cfg(__standback_before_1_47)]
        pub use crate::v1_47::f64::TAU;
        #[cfg(__standback_since_1_47)]
        pub use core::f64::consts::TAU;
    }
}
pub mod char {
    #[cfg(__standback_before_1_38)]
    pub const UNICODE_VERSION: (u8, u8, u8) = (11, 0, 0);
    #[cfg(all(__standback_since_1_38, __standback_before_1_44))]
    pub const UNICODE_VERSION: (u8, u8, u8) = (12, 1, 0);
    #[cfg(all(__standback_since_1_44, __standback_before_1_45))]
    pub const UNICODE_VERSION: (u8, u8, u8) = (13, 0, 0);
    #[cfg(__standback_since_1_45)]
    pub use core::char::UNICODE_VERSION;
}

#[cfg(__standback_since_1_36)]
pub mod future {
    #[cfg(__standback_before_1_48)]
    pub use crate::v1_48::future::{pending, ready, Pending, Ready};
    #[cfg(__standback_since_1_48)]
    pub use core::future::{pending, ready, Pending, Ready};
}
