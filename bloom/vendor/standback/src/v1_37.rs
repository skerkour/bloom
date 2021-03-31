use core::cell::Cell;
use core::ops::{Bound, RangeBounds};
use core::ptr;

use crate::traits::Sealed;

pub trait Cell_v1_37<T>: Sealed<Cell<T>> {
    fn from_mut(t: &mut T) -> &Cell<T>;
}

impl<T> Cell_v1_37<T> for Cell<T> {
    #[inline]
    fn from_mut(t: &mut T) -> &Cell<T> {
        unsafe { &*(t as *mut T as *const Cell<T>) }
    }
}

pub trait Cell_v1_37_<T>: Sealed<Cell<[T]>> {
    fn as_slice_of_cells(&self) -> &[Cell<T>];
}

impl<T> Cell_v1_37_<T> for Cell<[T]> {
    fn as_slice_of_cells(&self) -> &[Cell<T>] {
        unsafe { &*(self as *const Cell<[T]> as *const [Cell<T>]) }
    }
}

pub trait Option_v1_37<T>: Sealed<Option<T>> {
    fn xor(self, optb: Option<T>) -> Option<T>;
}

impl<T> Option_v1_37<T> for Option<T> {
    #[inline]
    fn xor(self, optb: Option<T>) -> Option<T> {
        match (self, optb) {
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            _ => None,
        }
    }
}

pub trait Slice_v1_37<T>: Sealed<[T]> {
    fn copy_within<R: RangeBounds<usize>>(&mut self, src: R, dest: usize)
    where
        T: Copy;
}

impl<T> Slice_v1_37<T> for [T] {
    fn copy_within<R: RangeBounds<usize>>(&mut self, src: R, dest: usize)
    where
        T: Copy,
    {
        let src_start = match src.start_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n
                .checked_add(1)
                .unwrap_or_else(|| slice_index_overflow_fail()),
            Bound::Unbounded => 0,
        };
        let src_end = match src.end_bound() {
            Bound::Included(&n) => n
                .checked_add(1)
                .unwrap_or_else(|| slice_index_overflow_fail()),
            Bound::Excluded(&n) => n,
            Bound::Unbounded => self.len(),
        };
        assert!(src_start <= src_end, "src end is before src start");
        assert!(src_end <= self.len(), "src is out of bounds");
        let count = src_end - src_start;
        assert!(dest <= self.len() - count, "dest is out of bounds");
        unsafe {
            ptr::copy(
                self.as_ptr().add(src_start),
                self.as_mut_ptr().add(dest),
                count,
            );
        }
    }
}

#[inline(never)]
#[cold]
fn slice_index_overflow_fail() -> ! {
    panic!("attempted to index slice up to maximum usize");
}

pub trait DoubleEndedIterator_v1_37: DoubleEndedIterator {
    fn nth_back(&mut self, n: usize) -> Option<Self::Item>;
}

impl<Iter: DoubleEndedIterator> DoubleEndedIterator_v1_37 for Iter {
    #[inline]
    fn nth_back(&mut self, mut n: usize) -> Option<Self::Item> {
        for x in self.rev() {
            if n == 0 {
                return Some(x);
            }
            n -= 1;
        }
        None
    }
}
