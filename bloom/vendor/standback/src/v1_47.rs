use core::ops::{DerefMut, Range};

use crate::traits::Sealed;

pub trait Range_v1_47<Idx>: Sealed<Range<Idx>> {
    fn is_empty(&self) -> bool;
}

impl<Idx: PartialOrd<Idx>> Range_v1_47<Idx> for Range<Idx> {
    fn is_empty(&self) -> bool {
        !(self.start < self.end)
    }
}

pub trait Result_v1_47<T: DerefMut, E>: Sealed<Result<T, E>> {
    fn as_deref(&self) -> Result<&T::Target, &E>;
    fn as_deref_mut(&mut self) -> Result<&mut T::Target, &mut E>;
}

impl<T: DerefMut, E> Result_v1_47<T, E> for Result<T, E> {
    fn as_deref(&self) -> Result<&T::Target, &E> {
        self.as_ref().map(|t| t.deref())
    }

    fn as_deref_mut(&mut self) -> Result<&mut T::Target, &mut E> {
        self.as_mut().map(|t| t.deref_mut())
    }
}

#[cfg(feature = "std")]
pub trait Vec_v1_47<T>: Sealed<Vec<T>> {
    fn leak<'a>(self) -> &'a mut [T]
    where
        T: 'a;
}

#[cfg(feature = "std")]
impl<T> Vec_v1_47<T> for Vec<T> {
    #[inline]
    fn leak<'a>(self) -> &'a mut [T]
    where
        T: 'a,
    {
        Box::leak(self.into_boxed_slice())
    }
}

pub mod f32 {
    pub const TAU: f32 = 6.28318530717958647692528676655900577_f32;
}

pub mod f64 {
    pub const TAU: f64 = 6.28318530717958647692528676655900577_f64;
}
