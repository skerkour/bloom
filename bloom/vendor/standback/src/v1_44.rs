use core::alloc::{Layout, LayoutErr};
use core::cmp;
use core::mem::{self, transmute};
#[cfg(feature = "std")]
use std::ffi::OsString;
#[cfg(feature = "std")]
use std::path::PathBuf;

use crate::traits::{Float, Sealed};

#[cfg(feature = "std")]
pub trait PathBuf_v1_44: Sealed<PathBuf> {
    fn with_capacity(capacity: usize) -> PathBuf;
    fn capacity(&self) -> usize;
    fn clear(&mut self);
    fn reserve(&mut self, additional: usize);
    fn reserve_exact(&mut self, additional: usize);
    fn shrink_to_fit(&mut self);
}

#[cfg(feature = "std")]
impl PathBuf_v1_44 for PathBuf {
    fn with_capacity(capacity: usize) -> PathBuf {
        OsString::with_capacity(capacity).into()
    }

    fn capacity(&self) -> usize {
        unsafe { transmute::<_, &OsString>(self) }.capacity()
    }

    fn clear(&mut self) {
        unsafe { transmute::<_, &mut OsString>(self) }.clear()
    }

    fn reserve(&mut self, additional: usize) {
        unsafe { transmute::<_, &mut OsString>(self) }.reserve(additional)
    }

    fn reserve_exact(&mut self, additional: usize) {
        unsafe { transmute::<_, &mut OsString>(self) }.reserve_exact(additional)
    }

    fn shrink_to_fit(&mut self) {
        unsafe { transmute::<_, &mut OsString>(self) }.shrink_to_fit()
    }
}

pub trait Layout_v1_44: Sealed<Layout> {
    fn align_to(&self, align: usize) -> Result<Layout, LayoutErr>;
    fn pad_to_align(&self) -> Layout;
    fn array<T>(n: usize) -> Result<Layout, LayoutErr>;
    fn extend(&self, next: Layout) -> Result<(Layout, usize), LayoutErr>;
}

impl Layout_v1_44 for Layout {
    #[inline]
    fn align_to(&self, align: usize) -> Result<Self, LayoutErr> {
        Layout::from_size_align(self.size(), cmp::max(self.align(), align))
    }

    #[inline]
    fn pad_to_align(&self) -> Layout {
        let pad = padding_needed_for(self, self.align());
        let new_size = self.size() + pad;
        Layout::from_size_align(new_size, self.align()).unwrap()
    }

    #[inline]
    fn array<T>(n: usize) -> Result<Self, LayoutErr> {
        repeat(&Layout::new::<T>(), n).map(|(k, offs)| {
            debug_assert!(offs == mem::size_of::<T>());
            k
        })
    }

    #[inline]
    fn extend(&self, next: Self) -> Result<(Self, usize), LayoutErr> {
        let new_align = cmp::max(self.align(), next.align());
        let pad = padding_needed_for(self, next.align());

        let offset = self.size().checked_add(pad).ok_or(layout_err())?;
        let new_size = offset.checked_add(next.size()).ok_or(layout_err())?;

        let layout = Layout::from_size_align(new_size, new_align)?;
        Ok((layout, offset))
    }
}

fn padding_needed_for(zelf: &Layout, align: usize) -> usize {
    let len = zelf.size();
    let len_rounded_up = len.wrapping_add(align).wrapping_sub(1) & !align.wrapping_sub(1);
    len_rounded_up.wrapping_sub(len)
}

#[inline]
fn repeat(zelf: &Layout, n: usize) -> Result<(Layout, usize), LayoutErr> {
    let padded_size = zelf.size() + padding_needed_for(zelf, zelf.align());
    let alloc_size = padded_size.checked_mul(n).ok_or(layout_err())?;

    unsafe {
        Ok((
            Layout::from_size_align_unchecked(alloc_size, zelf.align()),
            padded_size,
        ))
    }
}

#[inline(always)]
fn layout_err() -> LayoutErr {
    unsafe { transmute(()) }
}

mod sealed {
    pub trait FloatToInt<Int> {
        unsafe fn to_int_unchecked(self) -> Int;
    }

    macro_rules! impl_float_to_int {
        ($float:ident => $($int:ident)+) => {$(
            impl FloatToInt<$int> for $float {
                #[inline]
                unsafe fn to_int_unchecked(self) -> $int {
                    self as $int
                }
            }
        )+}
    }

    impl_float_to_int!(f32 => u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
    impl_float_to_int!(f64 => u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
}

pub trait float_v1_44: Float {
    unsafe fn to_int_unchecked<Int>(self) -> Int
    where
        Self: sealed::FloatToInt<Int>;
}

impl float_v1_44 for f32 {
    unsafe fn to_int_unchecked<Int>(self) -> Int
    where
        f32: sealed::FloatToInt<Int>,
    {
        sealed::FloatToInt::to_int_unchecked(self)
    }
}

impl float_v1_44 for f64 {
    unsafe fn to_int_unchecked<Int>(self) -> Int
    where
        f64: sealed::FloatToInt<Int>,
    {
        sealed::FloatToInt::to_int_unchecked(self)
    }
}
