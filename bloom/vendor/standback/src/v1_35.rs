use core::cell::RefCell;
use core::hash::{Hash, Hasher};
use core::mem;
use core::ops::{Bound, RangeBounds};

use crate::traits::Sealed;

pub trait RefCell_v1_35<T>: Sealed<RefCell<T>> {
    fn replace_with<F: FnOnce(&mut T) -> T>(&self, f: F) -> T;
}

impl<T> RefCell_v1_35<T> for RefCell<T> {
    #[inline]
    fn replace_with<F: FnOnce(&mut T) -> T>(&self, f: F) -> T {
        let mut_borrow = &mut *self.borrow_mut();
        let replacement = f(mut_borrow);
        mem::replace(mut_borrow, replacement)
    }
}

pub trait Option_v1_35<'a, T: Copy + 'a>: Sealed<Option<&'a T>> {
    fn copied(self) -> Option<T>;
}

impl<'a, T: Copy + 'a> Option_v1_35<'a, T> for Option<&'a T> {
    fn copied(self) -> Option<T> {
        self.map(|&t| t)
    }
}

pub fn hash<T: ?Sized, S: Hasher>(hashee: *const T, into: &mut S) {
    hashee.hash(into);
}

pub trait RangeBounds_v1_35<T>: RangeBounds<T> {
    fn contains<U>(&self, item: &U) -> bool
    where
        T: PartialOrd<U>,
        U: ?Sized + PartialOrd<T>;
}

impl<T: PartialOrd<T>, RB: RangeBounds<T>> RangeBounds_v1_35<T> for RB {
    fn contains<U>(&self, item: &U) -> bool
    where
        T: PartialOrd<U>,
        U: ?Sized + PartialOrd<T>,
    {
        contains(self, item)
    }
}

fn contains<T, U>(range: &impl RangeBounds<T>, item: &U) -> bool
where
    T: ?Sized + PartialOrd<U>,
    U: ?Sized + PartialOrd<T>,
{
    (match range.start_bound() {
        Bound::Included(ref start) => *start <= item,
        Bound::Excluded(ref start) => *start < item,
        Bound::Unbounded => true,
    }) && (match range.end_bound() {
        Bound::Included(ref end) => item <= *end,
        Bound::Excluded(ref end) => item < *end,
        Bound::Unbounded => true,
    })
}
