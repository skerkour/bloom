// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A fixed capacity array sized to match some other type `T`.
//!
//! See [`InlineArray`](struct.InlineArray.html)

use core::borrow::{Borrow, BorrowMut};
use core::cmp::Ordering;
use core::fmt::{Debug, Error, Formatter};
use core::hash::{Hash, Hasher};
use core::iter::FromIterator;
use core::marker::PhantomData;
use core::mem::{self, MaybeUninit};
use core::ops::{Deref, DerefMut};
use core::ptr;
use core::slice::{from_raw_parts, from_raw_parts_mut, Iter as SliceIter, IterMut as SliceIterMut};

mod iter;
pub use self::iter::{Drain, Iter};

/// A fixed capacity array sized to match some other type `T`.
///
/// This works like a vector, but allocated on the stack (and thus marginally
/// faster than `Vec`), with the allocated space exactly matching the size of
/// the given type `T`. The vector consists of a `usize` tracking its current
/// length, followed by zero or more elements of type `A`. The capacity is thus
/// `( size_of::<T>() - size_of::<usize>() ) / size_of::<A>()`. This could lead
/// to situations where the capacity is zero, if `size_of::<A>()` is greater
/// than `size_of::<T>() - size_of::<usize>()`, which is not an error and
/// handled properly by the data structure.
///
/// If `size_of::<T>()` is less than `size_of::<usize>()`, meaning the vector
/// has no space to store its length, `InlineArray::new()` will panic.
///
/// This is meant to facilitate optimisations where a list data structure
/// allocates a fairly large struct for itself, allowing you to replace it with
/// an `InlineArray` until it grows beyond its capacity. This not only gives you
/// a performance boost at very small sizes, it also saves you from having to
/// allocate anything on the heap until absolutely necessary.
///
/// For instance, `im::Vector<A>` in its final form currently looks like this
/// (approximately):
///
/// ```rust, ignore
/// struct RRB<A> {
///     length: usize,
///     tree_height: usize,
///     outer_head: Rc<Chunk<A>>,
///     inner_head: Rc<Chunk<A>>,
///     tree: Rc<TreeNode<A>>,
///     inner_tail: Rc<Chunk<A>>,
///     outer_tail: Rc<Chunk<A>>,
/// }
/// ```
///
/// That's two `usize`s and five `Rc`s, which comes in at 56 bytes on x86_64
/// architectures. With `InlineArray`, that leaves us with 56 -
/// `size_of::<usize>()` = 48 bytes we can use before having to expand into the
/// full data struture. If `A` is `u8`, that's 48 elements, and even if `A` is a
/// pointer we can still keep 6 of them inline before we run out of capacity.
///
/// We can declare an enum like this:
///
/// ```rust, ignore
/// enum VectorWrapper<A> {
///     Inline(InlineArray<A, RRB<A>>),
///     Full(RRB<A>),
/// }
/// ```
///
/// Both of these will have the same size, and we can swap the `Inline` case out
/// with the `Full` case once the `InlineArray` runs out of capacity.
pub struct InlineArray<A, T> {
    data: MaybeUninit<T>,
    phantom: PhantomData<A>,
}

impl<A, T> InlineArray<A, T> {
    const HOST_SIZE: usize = mem::size_of::<T>();
    const ELEMENT_SIZE: usize = mem::size_of::<A>();
    const HEADER_SIZE: usize = mem::size_of::<usize>();

    /// The maximum number of elements the `InlineArray` can hold.
    pub const CAPACITY: usize = (Self::HOST_SIZE - Self::HEADER_SIZE) / Self::ELEMENT_SIZE;

    #[inline]
    #[must_use]
    unsafe fn len_const(&self) -> *const usize {
        (&self.data) as *const _ as *const usize
    }

    #[inline]
    #[must_use]
    pub(crate) unsafe fn len_mut(&mut self) -> *mut usize {
        (&mut self.data) as *mut _ as *mut usize
    }

    #[inline]
    #[must_use]
    pub(crate) unsafe fn data(&self) -> *const A {
        self.len_const().add(1) as *const _ as *const A
    }

    #[inline]
    #[must_use]
    unsafe fn data_mut(&mut self) -> *mut A {
        self.len_mut().add(1) as *mut _ as *mut A
    }

    #[inline]
    #[must_use]
    unsafe fn ptr_at(&self, index: usize) -> *const A {
        self.data().add(index)
    }

    #[inline]
    #[must_use]
    unsafe fn ptr_at_mut(&mut self, index: usize) -> *mut A {
        self.data_mut().add(index)
    }

    #[inline]
    unsafe fn read_at(&self, index: usize) -> A {
        ptr::read(self.ptr_at(index))
    }

    #[inline]
    unsafe fn write_at(&mut self, index: usize, value: A) {
        ptr::write(self.ptr_at_mut(index), value);
    }

    /// Get the length of the array.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        unsafe { *self.len_const() }
    }

    /// Test if the array is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Test if the array is at capacity.
    #[inline]
    #[must_use]
    pub fn is_full(&self) -> bool {
        self.len() >= Self::CAPACITY
    }

    /// Construct a new empty array.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        debug_assert!(Self::HOST_SIZE > Self::HEADER_SIZE);
        let mut self_ = Self {
            data: MaybeUninit::uninit(),
            phantom: PhantomData,
        };
        unsafe { *self_.len_mut() = 0 }
        self_
    }

    /// Push an item to the back of the array.
    ///
    /// Panics if the capacity of the array is exceeded.
    ///
    /// Time: O(1)
    pub fn push(&mut self, value: A) {
        if self.is_full() {
            panic!("InlineArray::push: chunk size overflow");
        }
        unsafe {
            self.write_at(self.len(), value);
            *self.len_mut() += 1;
        }
    }

    /// Pop an item from the back of the array.
    ///
    /// Returns `None` if the array is empty.
    ///
    /// Time: O(1)
    pub fn pop(&mut self) -> Option<A> {
        if self.is_empty() {
            None
        } else {
            unsafe {
                *self.len_mut() -= 1;
            }
            Some(unsafe { self.read_at(self.len()) })
        }
    }

    /// Insert a new value at index `index`, shifting all the following values
    /// to the right.
    ///
    /// Panics if the index is out of bounds or the array is at capacity.
    ///
    /// Time: O(n) for the number of items shifted
    pub fn insert(&mut self, index: usize, value: A) {
        if self.is_full() {
            panic!("InlineArray::push: chunk size overflow");
        }
        if index > self.len() {
            panic!("InlineArray::insert: index out of bounds");
        }
        unsafe {
            let src = self.ptr_at_mut(index);
            ptr::copy(src, src.add(1), self.len() - index);
            ptr::write(src, value);
            *self.len_mut() += 1;
        }
    }

    /// Remove the value at index `index`, shifting all the following values to
    /// the left.
    ///
    /// Returns the removed value, or `None` if the array is empty or the index
    /// is out of bounds.
    ///
    /// Time: O(n) for the number of items shifted
    pub fn remove(&mut self, index: usize) -> Option<A> {
        if index >= self.len() {
            None
        } else {
            unsafe {
                let src = self.ptr_at_mut(index);
                let value = ptr::read(src);
                *self.len_mut() -= 1;
                ptr::copy(src.add(1), src, self.len() - index);
                Some(value)
            }
        }
    }

    /// Split an array into two, the original array containing
    /// everything up to `index` and the returned array containing
    /// everything from `index` onwards.
    ///
    /// Panics if `index` is out of bounds.
    ///
    /// Time: O(n) for the number of items in the new chunk
    pub fn split_off(&mut self, index: usize) -> Self {
        if index > self.len() {
            panic!("InlineArray::split_off: index out of bounds");
        }
        let mut out = Self::new();
        if index < self.len() {
            unsafe {
                ptr::copy(self.ptr_at(index), out.data_mut(), self.len() - index);
                *out.len_mut() = self.len() - index;
                *self.len_mut() = index;
            }
        }
        out
    }

    #[inline]
    unsafe fn drop_contents(&mut self) {
        ptr::drop_in_place::<[A]>(&mut **self)
    }

    /// Discard the contents of the array.
    ///
    /// Time: O(n)
    pub fn clear(&mut self) {
        unsafe {
            self.drop_contents();
            *self.len_mut() = 0;
        }
    }

    /// Construct an iterator that drains values from the front of the array.
    pub fn drain(&mut self) -> Drain<'_, A, T> {
        Drain { array: self }
    }
}

impl<A, T> Drop for InlineArray<A, T> {
    fn drop(&mut self) {
        unsafe { self.drop_contents() }
    }
}

impl<A, T> Default for InlineArray<A, T> {
    fn default() -> Self {
        Self::new()
    }
}

// WANT:
// impl<A, T> Copy for InlineArray<A, T> where A: Copy {}

impl<A, T> Clone for InlineArray<A, T>
where
    A: Clone,
{
    fn clone(&self) -> Self {
        let mut copy = Self::new();
        for i in 0..self.len() {
            unsafe {
                copy.write_at(i, self.get_unchecked(i).clone());
            }
        }
        unsafe {
            *copy.len_mut() = self.len();
        }
        copy
    }
}

impl<A, T> Deref for InlineArray<A, T> {
    type Target = [A];
    fn deref(&self) -> &Self::Target {
        unsafe { from_raw_parts(self.data(), self.len()) }
    }
}

impl<A, T> DerefMut for InlineArray<A, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { from_raw_parts_mut(self.data_mut(), self.len()) }
    }
}

impl<A, T> Borrow<[A]> for InlineArray<A, T> {
    fn borrow(&self) -> &[A] {
        self.deref()
    }
}

impl<A, T> BorrowMut<[A]> for InlineArray<A, T> {
    fn borrow_mut(&mut self) -> &mut [A] {
        self.deref_mut()
    }
}

impl<A, T> AsRef<[A]> for InlineArray<A, T> {
    fn as_ref(&self) -> &[A] {
        self.deref()
    }
}

impl<A, T> AsMut<[A]> for InlineArray<A, T> {
    fn as_mut(&mut self) -> &mut [A] {
        self.deref_mut()
    }
}
impl<A, T, Slice> PartialEq<Slice> for InlineArray<A, T>
where
    Slice: Borrow<[A]>,
    A: PartialEq,
{
    fn eq(&self, other: &Slice) -> bool {
        self.deref() == other.borrow()
    }
}

impl<A, T> Eq for InlineArray<A, T> where A: Eq {}

impl<A, T> PartialOrd for InlineArray<A, T>
where
    A: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(other.iter())
    }
}

impl<A, T> Ord for InlineArray<A, T>
where
    A: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other.iter())
    }
}

impl<A, T> Debug for InlineArray<A, T>
where
    A: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str("Chunk")?;
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<A, T> Hash for InlineArray<A, T>
where
    A: Hash,
{
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        for item in self {
            item.hash(hasher)
        }
    }
}

impl<A, T> IntoIterator for InlineArray<A, T> {
    type Item = A;
    type IntoIter = Iter<A, T>;
    fn into_iter(self) -> Self::IntoIter {
        Iter { array: self }
    }
}

impl<A, T> FromIterator<A> for InlineArray<A, T> {
    fn from_iter<I>(it: I) -> Self
    where
        I: IntoIterator<Item = A>,
    {
        let mut chunk = Self::new();
        for item in it {
            chunk.push(item);
        }
        chunk
    }
}

impl<'a, A, T> IntoIterator for &'a InlineArray<A, T> {
    type Item = &'a A;
    type IntoIter = SliceIter<'a, A>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, A, T> IntoIterator for &'a mut InlineArray<A, T> {
    type Item = &'a mut A;
    type IntoIter = SliceIterMut<'a, A>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<A, T> Extend<A> for InlineArray<A, T> {
    /// Append the contents of the iterator to the back of the array.
    ///
    /// Panics if the array exceeds its capacity.
    ///
    /// Time: O(n) for the length of the iterator
    fn extend<I>(&mut self, it: I)
    where
        I: IntoIterator<Item = A>,
    {
        for item in it {
            self.push(item);
        }
    }
}

impl<'a, A, T> Extend<&'a A> for InlineArray<A, T>
where
    A: 'a + Copy,
{
    /// Append the contents of the iterator to the back of the array.
    ///
    /// Panics if the array exceeds its capacity.
    ///
    /// Time: O(n) for the length of the iterator
    fn extend<I>(&mut self, it: I)
    where
        I: IntoIterator<Item = &'a A>,
    {
        for item in it {
            self.push(*item);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tests::DropTest;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn dropping() {
        let counter = AtomicUsize::new(0);
        {
            let mut chunk: InlineArray<DropTest<'_>, [usize; 32]> = InlineArray::new();
            for _i in 0..16 {
                chunk.push(DropTest::new(&counter));
            }
            assert_eq!(16, counter.load(Ordering::Relaxed));
            for _i in 0..8 {
                chunk.pop();
            }
            assert_eq!(8, counter.load(Ordering::Relaxed));
        }
        assert_eq!(0, counter.load(Ordering::Relaxed));
    }
}
