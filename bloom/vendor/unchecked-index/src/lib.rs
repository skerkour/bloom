
//! Unchecked indexing through the regular index syntax.
//!
//! Using a wrapper type that requires an `unsafe` block to create.
//!
//! *Note:* All unchecked indexing here is actually “checked” with *debug
//! assertions* when they are enabled (they are off by default in release
//! builds). This is a feature! Debug checking does **not** make your code safe,
//! but it helps finding bugs in `unsafe` code. Test your code responsibly.
//!
//! # Example
//!
//! ```rust
//!
//! use unchecked_index::unchecked_index;
//!
//! /// unsafe because: trusts the permutation to be correct
//! unsafe fn apply_permutation<T>(perm: &mut [usize], v: &mut [T]) {
//!     debug_assert_eq!(perm.len(), v.len());
//!     
//!     // use unchecked (in reality, debug-checked) indexing throughout
//!     let mut perm = unchecked_index(perm);
//!     
//!     for i in 0..perm.len() {
//!         let mut current = i;
//!         while i != perm[current] {
//!             let next = perm[current];
//!             // move element from next to current
//!             v.swap(next, current);
//!             perm[current] = current;
//!             current = next;
//!         }
//!         perm[current] = current;
//!     }
//! }
//! ```
//!
//! # Crate Features
//!
//! This crate is always `#![no_std]`.
//!
//! # Rust Version
//!
//! This version of the crate requires Rust 1.15 or later.

#![cfg_attr(not(test), no_std)]

#[cfg(not(test))]
extern crate core as std;

/// Wrapper type for unchecked indexing through the regular index syntax
///
/// Note that the indexing is checked with debug assertions, but unchecked
/// in release mode. Test your code responsibly.
#[derive(Copy)]
pub struct UncheckedIndex<S>(S);

impl<S: Copy> Clone for UncheckedIndex<S> {
    fn clone(&self) -> Self { *self }
}

/// Create a new unchecked indexing wrapper.
///
/// This function is `unsafe` to call because it allows all further indexing
/// on the wrapper to omit bounds checks.
///
/// # Safety
///
/// The caller must ensure that **all** indexing of the resulting
/// `UncheckedIndex` wrapper is in bounds of the underlying container.
pub unsafe fn unchecked_index<T>(v: T) -> UncheckedIndex<T>
{
    UncheckedIndex(v)
}

/// Access the element(s) at `index`, without bounds checks!
///
/// *Note:* Will use *debug assertions* to check that the index is actually
/// valid. In release mode, debug assertions are *off* by default.
///
/// # Safety
///
/// The caller must ensure that `index` is in bounds of the underlying
/// container.
pub unsafe fn get_unchecked<T: ?Sized, I>(v: &T, index: I) -> &T::Output
    where T: GetUnchecked<I>
{
    #[cfg(debug_assertions)]
    v.assert_indexable_with(&index);
    v.get_unchecked(index)
}

/// Access the element(s) at `index`, without bounds checks!
///
/// *Note:* Will use *debug assertions* to check that the index is actually
/// valid. In release mode, debug assertions are *off* by default.
///
/// # Safety
///
/// The caller must ensure that `index` is in bounds of the underlying
/// container.
pub unsafe fn get_unchecked_mut<T: ?Sized, I>(v: &mut T, index: I) -> &mut T::Output
    where T: GetUncheckedMut<I>
{
    #[cfg(debug_assertions)]
    v.assert_indexable_with(&index);
    v.get_unchecked_mut(index)
}

use std::ops::{Deref, DerefMut, Index, IndexMut};

impl<T> Deref for UncheckedIndex<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for UncheckedIndex<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T, I> Index<I> for UncheckedIndex<T>
    where T: GetUnchecked<I>
{
    type Output = T::Output;

    /// Access the element(s) at `index`, without bounds checks!
    ///
    /// *Note:* Will use *debug assertions* to check that the index is actually
    /// valid. In release mode, debug assertions are *off* by default.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `index` is always in bounds of the
    /// underlying container.
    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        unsafe {
            get_unchecked(&self.0, index)
        }
    }
}

impl<T, I> IndexMut<I> for UncheckedIndex<T>
    where T: GetUncheckedMut<I>
{
    /// Access the element(s) at `index`, without bounds checks!
    ///
    /// *Note:* Will use *debug assertions* to check that the index is actually
    /// valid. In release mode, debug assertions are *off* by default.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `index` is always in bounds of the
    /// underlying container.
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        unsafe {
            get_unchecked_mut(&mut self.0, index)
        }
    }
}

pub trait CheckIndex<I> {
    /// Assert (using a regular assertion) that the index is valid.
    /// Must not return if the index is invalid for indexing self.
    ///
    /// ***Panics*** if `index` is invalid.
    fn assert_indexable_with(&self, index: &I);
}

impl<'a, T: ?Sized, I> CheckIndex<I> for &'a T where T: CheckIndex<I> {
    fn assert_indexable_with(&self, index: &I) {
        (**self).assert_indexable_with(index)
    }
}

impl<'a, T: ?Sized, I> CheckIndex<I> for &'a mut T where T: CheckIndex<I> {
    fn assert_indexable_with(&self, index: &I) {
        (**self).assert_indexable_with(index)
    }
}

pub trait GetUnchecked<I>: CheckIndex<I> {
    type Output: ?Sized;
    unsafe fn get_unchecked(&self, index: I) -> &Self::Output;
}

pub trait GetUncheckedMut<I>: GetUnchecked<I> {
    unsafe fn get_unchecked_mut(&mut self, index: I) -> &mut Self::Output;
}

impl<'a, T: ?Sized, I> GetUnchecked<I> for &'a T
    where T: GetUnchecked<I>
{
    type Output = T::Output;
    unsafe fn get_unchecked(&self, index: I) -> &Self::Output {
        (**self).get_unchecked(index)
    }
}

impl<'a, T: ?Sized, I> GetUnchecked<I> for &'a mut T
    where T: GetUnchecked<I>
{
    type Output = T::Output;
    unsafe fn get_unchecked(&self, index: I) -> &Self::Output {
        (**self).get_unchecked(index)
    }
}

impl<'a, T: ?Sized, I> GetUncheckedMut<I> for &'a mut T
    where T: GetUncheckedMut<I>
{
    unsafe fn get_unchecked_mut(&mut self, index: I) -> &mut Self::Output {
        (**self).get_unchecked_mut(index)
    }
}

mod slice_impls;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut data = [0; 8];
        unsafe {
            let mut data = unchecked_index(&mut data);
            for i in 0..data.len() {
                data[i] = i;
            }
        }
        assert_eq!(data, [0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn debug_oob_check_write() {
        let mut data = [0; 8];
        unsafe {
            let mut data = unchecked_index(&mut data[..7]);
            data[7] = 1;
        }
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn debug_oob_check_read() {
        let mut data = [0; 8];
        unsafe {
            let data = unchecked_index(&mut data[..7]);
            println!("{}", data[17]);
        }
    }

    #[cfg(not(debug_assertions))]
    #[test]
    fn non_debug_oob() {
        // outside bounds of the slice but not the data -- should be ok
        let mut data = [0; 8];
        unsafe {
            let mut data = unchecked_index(&mut data[..7]);
            data[7] = 1;
        }
        assert_eq!(data, [0, 0, 0, 0, 0, 0, 0, 1]);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn debug_oob_check_read_slice_1() {
        let mut data = [0; 8];
        unsafe {
            let data = unchecked_index(&mut data[..7]);
            println!("{:?}", &data[5..10]);
        }
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn debug_oob_check_read_slice_2() {
        let mut data = [0; 8];
        unsafe {
            let data = unchecked_index(&mut data[..7]);
            println!("{:?}", &data[7..6]);
        }
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn debug_oob_check_read_slice_3() {
        let mut data = [0; 8];
        unsafe {
            let data = unchecked_index(&mut data[..7]);
            println!("{:?}", &data[8..]);
        }
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn debug_oob_check_read_slice_4() {
        let mut data = [0; 8];
        unsafe {
            let data = unchecked_index(&mut data[..7]);
            println!("{:?}", &data[..9]);
        }
    }

    #[cfg(not(debug_assertions))]
    #[test]
    fn non_debug_oob_slice() {
        // outside bounds of the slice but not the data -- should be ok
        let mut data = [0; 8];
        unsafe {
            let mut data = unchecked_index(&mut data[..7]);
            data[7..8][0] = 1;
        }
        assert_eq!(data, [0, 0, 0, 0, 0, 0, 0, 1]);
    }
}
