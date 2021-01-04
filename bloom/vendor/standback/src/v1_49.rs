mod sort;

use crate::traits::Sealed;
use core::cmp::Ordering;

pub trait Slice_v1_49<T>: Sealed<[T]> {
    fn select_nth_unstable(&mut self, index: usize) -> (&mut [T], &mut T, &mut [T])
    where
        T: Ord;
    fn select_nth_unstable_by<F>(
        &mut self,
        index: usize,
        compare: F,
    ) -> (&mut [T], &mut T, &mut [T])
    where
        F: FnMut(&T, &T) -> Ordering;
    fn select_nth_unstable_by_key<K, F>(
        &mut self,
        index: usize,
        f: F,
    ) -> (&mut [T], &mut T, &mut [T])
    where
        F: FnMut(&T) -> K,
        K: Ord;
}

impl<T> Slice_v1_49<T> for [T] {
    fn select_nth_unstable(&mut self, index: usize) -> (&mut [T], &mut T, &mut [T])
    where
        T: Ord,
    {
        let mut f = |a: &T, b: &T| a.lt(b);
        sort::partition_at_index(self, index, &mut f)
    }

    fn select_nth_unstable_by<F>(
        &mut self,
        index: usize,
        mut compare: F,
    ) -> (&mut [T], &mut T, &mut [T])
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        let mut f = |a: &T, b: &T| compare(a, b) == Ordering::Less;
        sort::partition_at_index(self, index, &mut f)
    }

    fn select_nth_unstable_by_key<K, F>(
        &mut self,
        index: usize,
        mut f: F,
    ) -> (&mut [T], &mut T, &mut [T])
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        let mut g = |a: &T, b: &T| f(a).lt(&f(b));
        sort::partition_at_index(self, index, &mut g)
    }
}
