#![cfg(feature = "extern_crate_alloc")]

//! Stuff to boost things in the `alloc` crate.
//!
//! * You must enable the `extern_crate_alloc` feature of `bytemuck` or you will
//!   not be able to use this module!

use super::*;
use alloc::{
  alloc::{alloc_zeroed, Layout},
  boxed::Box,
  vec,
  vec::Vec,
};
use core::convert::TryInto;

/// As [`try_cast_box`](try_cast_box), but unwraps for you.
#[inline]
pub fn cast_box<A: Pod, B: Pod>(input: Box<A>) -> Box<B> {
  try_cast_box(input).map_err(|(e, _v)| e).unwrap()
}

/// Attempts to cast the content type of a [`Box`](alloc::boxed::Box).
///
/// On failure you get back an error along with the starting `Box`.
///
/// ## Failure
///
/// * The start and end content type of the `Box` must have the exact same
///   alignment.
/// * The start and end size of the `Box` must have the exact same size.
#[inline]
pub fn try_cast_box<A: Pod, B: Pod>(
  input: Box<A>,
) -> Result<Box<B>, (PodCastError, Box<A>)> {
  if align_of::<A>() != align_of::<B>() {
    Err((PodCastError::AlignmentMismatch, input))
  } else if size_of::<A>() != size_of::<B>() {
    Err((PodCastError::SizeMismatch, input))
  } else {
    // Note(Lokathor): This is much simpler than with the Vec casting!
    let ptr: *mut B = Box::into_raw(input) as *mut B;
    Ok(unsafe { Box::from_raw(ptr) })
  }
}

/// Allocates a `Box<T>` with all of the contents being zeroed out.
///
/// This uses the global allocator to create a zeroed allocation and _then_
/// turns it into a Box. In other words, it's 100% assured that the zeroed data
/// won't be put temporarily on the stack. You can make a box of any size
/// without fear of a stack overflow.
///
/// ## Failure
///
/// This fails if the allocation fails.
#[inline]
pub fn try_zeroed_box<T: Zeroable>() -> Result<Box<T>, ()> {
  if size_of::<T>() == 0 {
    // This will not allocate but simple create a dangling slice pointer.
    // NB: We go the way via a push to `Vec<T>` to ensure the compiler
    // does not allocate space for T on the stack even if the branch
    // would not be taken.
    let mut vec = Vec::with_capacity(1);
    vec.resize_with(1, || T::zeroed());
    let ptr: Box<[T; 1]> = vec.into_boxed_slice().try_into().ok().unwrap();
    debug_assert!(
      align_of::<[T; 1]>() == align_of::<T>()
        && size_of::<[T; 1]>() == size_of::<T>()
    );
    // NB: We basically do the same as in try_cast_box here:
    let ptr: Box<T> = unsafe { Box::from_raw(Box::into_raw(ptr) as *mut _) };
    return Ok(ptr);
  }
  let layout =
    Layout::from_size_align(size_of::<T>(), align_of::<T>()).unwrap();
  let ptr = unsafe { alloc_zeroed(layout) };
  if ptr.is_null() {
    // we don't know what the error is because `alloc_zeroed` is a dumb API
    Err(())
  } else {
    Ok(unsafe { Box::<T>::from_raw(ptr as *mut T) })
  }
}

/// As [`try_zeroed_box`], but unwraps for you.
#[inline]
pub fn zeroed_box<T: Zeroable>() -> Box<T> {
  try_zeroed_box().unwrap()
}

/// Allocates a `Box<[T]>` with all contents being zeroed out.
///
/// This uses the global allocator to create a zeroed allocation and _then_
/// turns it into a Box. In other words, it's 100% assured that the zeroed data
/// won't be put temporarily on the stack. You can make a box of any size
/// without fear of a stack overflow.
///
/// ## Failure
///
/// This fails if the allocation fails.
#[inline]
pub fn try_zeroed_slice_box<T: Zeroable>(
  length: usize,
) -> Result<Box<[T]>, ()> {
  if size_of::<T>() == 0 {
    // This will not allocate but simple create a dangling slice pointer.
    let mut vec = Vec::with_capacity(length);
    vec.resize_with(length, || T::zeroed());
    return Ok(vec.into_boxed_slice());
  }
  if length == 0 {
    // This will also not allocate.
    return Ok(Vec::new().into_boxed_slice());
  }
  // For Pod types, the layout of the array/slice is equivalent to repeating the
  // type.
  let layout_length = size_of::<T>().checked_mul(length).ok_or(())?;
  assert!(layout_length != 0);
  let layout =
    Layout::from_size_align(layout_length, align_of::<T>()).map_err(|_| ())?;
  let ptr = unsafe { alloc_zeroed(layout) };
  if ptr.is_null() {
    // we don't know what the error is because `alloc_zeroed` is a dumb API
    Err(())
  } else {
    let slice =
      unsafe { core::slice::from_raw_parts_mut(ptr as *mut T, length) };
    Ok(unsafe { Box::<[T]>::from_raw(slice) })
  }
}

/// As [`try_zeroed_slice_box`](try_zeroed_slice_box), but unwraps for you.
pub fn zeroed_slice_box<T: Zeroable>(length: usize) -> Box<[T]> {
  try_zeroed_slice_box(length).unwrap()
}

/// As [`try_cast_vec`](try_cast_vec), but unwraps for you.
#[inline]
pub fn cast_vec<A: Pod, B: Pod>(input: Vec<A>) -> Vec<B> {
  try_cast_vec(input).map_err(|(e, _v)| e).unwrap()
}

/// Attempts to cast the content type of a [`Vec`](alloc::vec::Vec).
///
/// On failure you get back an error along with the starting `Vec`.
///
/// ## Failure
///
/// * The start and end content type of the `Vec` must have the exact same
///   alignment.
/// * The start and end size of the `Vec` must have the exact same size.
/// * In the future this second restriction might be lessened by having the
///   capacity and length get adjusted during transmutation, but for now it's
///   absolute.
#[inline]
pub fn try_cast_vec<A: Pod, B: Pod>(
  input: Vec<A>,
) -> Result<Vec<B>, (PodCastError, Vec<A>)> {
  if align_of::<A>() != align_of::<B>() {
    Err((PodCastError::AlignmentMismatch, input))
  } else if size_of::<A>() != size_of::<B>() {
    // Note(Lokathor): Under some conditions it would be possible to cast
    // between Vec content types of the same alignment but different sizes by
    // changing the capacity and len values in the output Vec. However, we will
    // not attempt that for now.
    Err((PodCastError::SizeMismatch, input))
  } else {
    // Note(Lokathor): First we record the length and capacity, which don't have
    // any secret provenance metadata.
    let length: usize = input.len();
    let capacity: usize = input.capacity();
    // Note(Lokathor): Next we "pre-forget" the old Vec by wrapping with
    // ManuallyDrop, because if we used `core::mem::forget` after taking the
    // pointer then that would invalidate our pointer. In nightly there's a
    // "into raw parts" method, which we can switch this too eventually.
    let mut manual_drop_vec = ManuallyDrop::new(input);
    let vec_ptr: *mut A = manual_drop_vec.as_mut_ptr();
    let ptr: *mut B = vec_ptr as *mut B;
    Ok(unsafe { Vec::from_raw_parts(ptr, length, capacity) })
  }
}

/// This "collects" a slice of pod data into a vec of a different pod type.
///
/// Unlike with [`cast_slice`] and [`cast_slice_mut`], this will always work.
///
/// The output vec will be of a minimal size/capacity to hold the slice given.
///
/// ```rust
/// # use bytemuck::*;
/// let halfwords: [u16; 4] = [5, 6, 7, 8];
/// let vec_of_words: Vec<u32> = pod_collect_to_vec(&halfwords);
/// if cfg!(target_endian = "little") {
///   assert_eq!(&vec_of_words[..], &[0x0006_0005, 0x0008_0007][..])
/// } else {
///   assert_eq!(&vec_of_words[..], &[0x0005_0006, 0x0007_0008][..])
/// }
/// ```
pub fn pod_collect_to_vec<A: Pod, B: Pod>(src: &[A]) -> Vec<B> {
  let src_size = size_of_val(src);
  // Note(Lokathor): dst_count is rounded up so that the dest will always be at
  // least as many bytes as the src.
  let dst_count = src_size / size_of::<B>()
    + if src_size % size_of::<B>() != 0 { 1 } else { 0 };
  let mut dst = vec![B::zeroed(); dst_count];

  let src_bytes: &[u8] = cast_slice(src);
  let dst_bytes: &mut [u8] = cast_slice_mut(&mut dst[..]);
  dst_bytes[..src_size].copy_from_slice(src_bytes);
  dst
}
