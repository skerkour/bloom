
//!  This is a rather strange-looking workaround. The intention
//!  is to keep GetUnchecked/Mut out of scope, so that we can use the
//!  libcore SliceExt::get_unchecked/_mut methods.

use std::ops::{Range, RangeTo, RangeFrom, RangeFull};

use super::CheckIndex;

type Output<T, I> = <[T] as ::std::ops::Index<I>>::Output;

macro_rules! impl_for_slice {
    ($name:ident, $index_type:ty, $self_:ident, $index: ident, $assertion:expr) => {

        mod $name {
            use super::Output;
            #[allow(unused)]
            use std::ops::{Range, RangeTo, RangeFrom, RangeFull};

            #[inline]
            pub unsafe fn get<T>(s: &[T], index: $index_type) -> &Output<T, $index_type> {
                s.get_unchecked(index)
            }

            #[inline]
            pub unsafe fn getm<T>(s: &mut [T], index: $index_type) -> &mut Output<T, $index_type> {
                s.get_unchecked_mut(index)
            }
        }

        impl<T> CheckIndex<$index_type> for [T] {
            fn assert_indexable_with($self_: &Self, $index: &$index_type) {
                $assertion
            }
        }

        impl<T> super::GetUnchecked<$index_type> for [T] {
            type Output = Output<T, $index_type>;
            unsafe fn get_unchecked(&self, index: $index_type) -> &Self::Output {
                $name::get(self, index)
            }
        }

        impl<T> super::GetUncheckedMut<$index_type> for [T] {
            unsafe fn get_unchecked_mut(&mut self, index: $index_type) -> &mut Self::Output {
                $name::getm(self, index)
            }
        }
    }
}

impl_for_slice!(index, usize, self, index, {
    assert!(*index < self.len(),
            "assertion index < len failed: index out of bounds: \
            index = {}, len = {}",
            index, self.len())
});

impl_for_slice!(range, Range<usize>, self, index, {
  assert!(index.start <= index.end,
          "assertion start <= end failed: start = {}, end = {}, len = {}",
          index.start, index.end, self.len());
  assert!(index.end <= self.len(),
          "assertion end <= len failed: end = {}, len = {}",
          index.end, self.len());
});

impl_for_slice!(rangeto, RangeTo<usize>, self, index, {
  assert!(index.end <= self.len(),
          "assertion end <= len failed: end = {}, len = {}",
          index.end, self.len());
});

impl_for_slice!(rangefrom,RangeFrom<usize>, self, index, {
  assert!(index.start <= self.len(),
          "assertion start <= len failed: start = {}, len = {}",
          index.start, self.len());
});

impl_for_slice!(rangefull, RangeFull, self, _index, { });
