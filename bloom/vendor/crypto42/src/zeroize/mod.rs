//! # Securely zero memory
//!
//! Securely zero memory with a simple trait ([Zeroize]) built on stable Rust
//! primitives which guarantee the operation will not be "optimized away".
//!
//! ## About
//!
//! [Zeroing memory securely is hard] - compilers optimize for performance, and
//! in doing so they love to "optimize away" unnecessary zeroing calls. There are
//! many documented "tricks" to attempt to avoid these optimizations and ensure
//! that a zeroing routine is performed reliably.
//!
//! This crate isn't about tricks: it uses [core::ptr::write_volatile]
//! and [core::sync::atomic] memory fences to provide easy-to-use, portable
//! zeroing behavior which works on all of Rust's core number types and slices
//! thereof, implemented in pure Rust with no usage of FFI or assembly.
//!
//! - No insecure fallbacks!
//! - No dependencies!
//! - No FFI or inline assembly! **WASM friendly** (and tested)!
//! - `#![no_std]` i.e. **embedded-friendly**!
//! - No functionality besides securely zeroing memory!
//! - (Optional) Custom derive support for zeroing complex structures
//!
//! ## Usage
//!
//! ```
//! use crypto42::zeroize::Zeroize;
//!
//! // Protip: don't embed secrets in your source code.
//! // This is just an example.
//! let mut secret = b"Air shield password: 1,2,3,4,5".to_vec();
//! // [ ... ] open the air shield here
//!
//! // Now that we're done using the secret, zero it out.
//! secret.zeroize();
//! ```
//!
//! The [Zeroize] trait is impl'd on all of Rust's core scalar types including
//! integers, floats, `bool`, and `char`.
//!
//! Additionally, it's implemented on slices and `IterMut`s of the above types.
//!
//! When the `alloc` feature is enabled (which it is by default), it's also
//! impl'd for `Vec<T>` for the above types as well as `String`, where it provides
//! [Vec::clear()] / [String::clear()]-like behavior (truncating to zero-length)
//! but ensures the backing memory is securely zeroed with some caveats.
//! (NOTE: see "Stack/Heap Zeroing Notes" for important `Vec`/`String` details)
//!
//! The [DefaultIsZeroes] marker trait can be impl'd on types which also
//! impl [Default], which implements [Zeroize] by overwriting a value with
//! the default value.
//!
//! ## Custom Derive Support
//!
//! This crate has custom derive support for the `Zeroize` trait,
//! gated under the `zeroize` crate's `zeroize_derive` Cargo feature,
//! which automatically calls `zeroize()` on all members of a struct
//! or tuple struct.
//!
//! Additionally it supports the following attribute:
//!
//! - `#[zeroize(drop)]`: call `zeroize()` when this item is dropped
//!
//! Example which derives `Drop`:
//!
//! ```ignore
//! use crypto42::zeroize::Zeroize;
//!
//! // This struct will be zeroized on drop
//! #[derive(Zeroize)]
//! #[zeroize(drop)]
//! struct MyStruct([u8; 32]);
//! ```
//!
//! Example which does not derive `Drop` (useful for e.g. `Copy` types)
//!
//! ```ignore
//! use crypto42::zeroize::Zeroize;
//!
//! // This struct will *NOT* be zeroized on drop
//! #[derive(Copy, Clone, Zeroize)]
//! struct MyStruct([u8; 32]);
//! ```
//!
//! ## `Zeroizing<Z>`: wrapper for zeroizing arbitrary values on drop
//!
//! `Zeroizing<Z: Zeroize>` is a generic wrapper type that impls `Deref`
//! and `DerefMut`, allowing access to an inner value of type `Z`, and also
//! impls a `Drop` handler which calls `zeroize()` on its contents:
//!
//! ```
//! use crypto42::zeroize::Zeroizing;
//!
//! let mut secret = Zeroizing::new([0u8; 5]);
//!
//! // Set the air shield password
//! // Protip (again): don't embed secrets in your source code.
//! secret.copy_from_slice(&[1, 2, 3, 4, 5]);
//! assert_eq!(secret.as_ref(), &[1, 2, 3, 4, 5]);
//!
//! // The contents of `secret` will be automatically zeroized on drop
//! ```
//!
//! ## What guarantees does this crate provide?
//!
//! This crate guarantees the following:
//!
//! 1. The zeroing operation can't be "optimized away" by the compiler.
//! 2. All subsequent reads to memory will see "zeroized" values.
//!
//! LLVM's volatile semantics ensure #1 is true.
//!
//! Additionally, thanks to work by the [Unsafe Code Guidelines Working Group],
//! we can now fairly confidently say #2 is true as well. Previously there were
//! worries that the approach used by this crate (mixing volatile and
//! non-volatile accesses) was undefined behavior due to language contained
//! in the documentation for `write_volatile`, however after some discussion
//! [these remarks have been removed] and the specific usage pattern in this
//! crate is considered to be well-defined.
//!
//! Additionally this crate leverages [compiler_fence] from
//! [core::sync::atomic] with the strictest ordering ([Ordering::SeqCst])
//! as a precaution to help ensure reads are not reordered before memory has
//! been zeroed.
//!
//! All of that said, there is still potential for microarchitectural attacks
//! (ala Spectre/Meltdown) to leak "zeroized" secrets through covert channels.
//! This crate makes no guarantees that zeroized values cannot be leaked
//! through such channels, as they represent flaws in the underlying hardware.
//!
//! ## Stack/Heap Zeroing Notes
//!
//! This crate can be used to zero values from either the stack or the heap.
//!
//! However, be aware several operations in Rust can unintentionally leave
//! copies of data in memory. This includes but is not limited to:
//!
//! - Moves and `Copy`
//! - Heap reallocation when using `Vec` and `String`
//! - Borrowers of a reference making copies of the data
//!
//! [`Pin`][pin] can be leveraged in conjunction with this crate to ensure
//! data kept on the stack isn't moved.
//!
//! The `Zeroize` impls for `Vec` and `String` zeroize the entire capacity of
//! their backing buffer, but cannot guarantee copies of the data were not
//! previously made by buffer reallocation. It's therefore important when
//! attempting to zeroize such buffers to initialize them to the correct
//! capacity, and take care to prevent subsequent reallocation.
//!
//! The `secrecy` crate provides higher-level abstractions for eliminating
//! usage patterns which can cause reallocations:
//!
//! <https://crates.io/crates/secrecy>
//!
//! ## What about: clearing registers, mlock, mprotect, etc?
//!
//! This crate is focused on providing simple, unobtrusive support for reliably
//! zeroing memory using the best approach possible on stable Rust.
//!
//! Clearing registers is a difficult problem that can't easily be solved by
//! something like a crate, and requires either inline ASM or rustc support.
//! See <https://github.com/rust-lang/rust/issues/17046> for background on
//! this particular problem.
//!
//! Other memory protection mechanisms are interesting and useful, but often
//! overkill (e.g. defending against RAM scraping or attackers with swap access).
//! In as much as there may be merit to these approaches, there are also many
//! other crates that already implement more sophisticated memory protections.
//! Such protections are explicitly out-of-scope for this crate.
//!
//! Zeroing memory is [good cryptographic hygiene] and this crate seeks to promote
//! it in the most unobtrusive manner possible. This includes omitting complex
//! `unsafe` memory protection systems and just trying to make the best memory
//! zeroing crate available.
//!
//! [Zeroize]: https://docs.rs/zeroize/latest/zeroize/trait.Zeroize.html
//! [Zeroing memory securely is hard]: http://www.daemonology.net/blog/2014-09-04-how-to-zero-a-buffer.html
//! [Vec::clear()]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.clear
//! [String::clear()]: https://doc.rust-lang.org/std/string/struct.String.html#method.clear
//! [DefaultIsZeroes]: https://docs.rs/zeroize/latest/zeroize/trait.DefaultIsZeroes.html
//! [Default]: https://doc.rust-lang.org/std/default/trait.Default.html
//! [core::ptr::write_volatile]: https://doc.rust-lang.org/core/ptr/fn.write_volatile.html
//! [Unsafe Code Guidelines Working Group]: https://github.com/rust-lang/unsafe-code-guidelines
//! [these remarks have been removed]: https://github.com/rust-lang/rust/pull/60972
//! [core::sync::atomic]: https://doc.rust-lang.org/stable/core/sync/atomic/index.html
//! [Ordering::SeqCst]: https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html#variant.SeqCst
//! [compiler_fence]: https://doc.rust-lang.org/stable/core/sync/atomic/fn.compiler_fence.html
//! [pin]: https://doc.rust-lang.org/std/pin/struct.Pin.html
//! [good cryptographic hygiene]: https://github.com/veorq/cryptocoding#clean-memory-of-secret-data

#[cfg(feature = "zeroize_bytes")]
mod bytes;

#[cfg(feature = "zeroize_derive")]
pub use zeroize_derive::Zeroize;

#[cfg(feature = "zeroize_derive")]
#[doc(hidden)]
pub use zeroize_derive::*;

use core::{ops, ptr, slice::IterMut, sync::atomic};

/// Trait for securely erasing types from memory
pub trait Zeroize {
    /// Zero out this object from memory (using Rust or OS intrinsics which
    /// ensure the zeroization operation is not "optimized away")
    fn zeroize(&mut self);
}

/// Marker trait for types whose `Default` is the desired zeroization result
pub trait DefaultIsZeroes: Copy + Default + Sized {}

impl<Z> Zeroize for Z
where
    Z: DefaultIsZeroes,
{
    fn zeroize(&mut self) {
        volatile_write(self, Z::default());
        atomic_fence();
    }
}

macro_rules! impl_zeroize_with_default {
    ($($type:ty),+) => {
        $(impl DefaultIsZeroes for $type {})+
    };
}

impl_zeroize_with_default!(i8, i16, i32, i64, i128, isize);
impl_zeroize_with_default!(u8, u16, u32, u64, u128, usize);
impl_zeroize_with_default!(f32, f64, char, bool);

/// Implement `Zeroize` on arrays of types that impl `Zeroize`
macro_rules! impl_zeroize_for_array {
    ($($size:expr),+) => {
        $(
            impl<Z> Zeroize for [Z; $size]
            where
                Z: Zeroize
            {
                fn zeroize(&mut self) {
                    self.iter_mut().zeroize();
                }
            }
        )+
     };
}

// TODO(tarcieri): const generics
impl_zeroize_for_array!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
);

impl<'a, Z> Zeroize for IterMut<'a, Z>
where
    Z: Zeroize,
{
    fn zeroize(&mut self) {
        for elem in self {
            elem.zeroize();
        }
    }
}

impl<Z> Zeroize for Option<Z>
where
    Z: Zeroize,
{
    fn zeroize(&mut self) {
        match self {
            Some(value) => value.zeroize(),
            None => (),
        }
    }
}

/// Impl `Zeroize` on slices of types that can be zeroized with `Default`.
///
/// This impl can eventually be optimized using an memset intrinsic,
/// such as `core::intrinsics::volatile_set_memory`. For that reason the blanket
/// impl on slices is bounded by `DefaultIsZeroes`.
///
/// To zeroize a mut slice of `Z: Zeroize` which does not impl
/// `DefaultIsZeroes`, call `iter_mut().zeroize()`.
impl<Z> Zeroize for [Z]
where
    Z: DefaultIsZeroes,
{
    fn zeroize(&mut self) {
        volatile_set(self, Z::default());
        atomic_fence();
    }
}

impl<Z> Zeroize for Vec<Z>
where
    Z: DefaultIsZeroes,
{
    fn zeroize(&mut self) {
        self.resize(self.capacity(), Default::default());
        self.as_mut_slice().zeroize();
        self.clear();
    }
}

impl Zeroize for String {
    fn zeroize(&mut self) {
        unsafe { self.as_bytes_mut() }.zeroize();
        debug_assert!(self.as_bytes().iter().all(|b| *b == 0));
        self.clear();
    }
}

/// `Zeroizing` is a a wrapper for any `Z: Zeroize` type which implements a
/// `Drop` handler which zeroizes dropped values.
pub struct Zeroizing<Z: Zeroize>(Z);

impl<Z> Zeroizing<Z>
where
    Z: Zeroize,
{
    /// Wrap a value in `Zeroizing`, ensuring it's zeroized on drop.
    pub fn new(value: Z) -> Self {
        Zeroizing(value)
    }
}

impl<Z> ops::Deref for Zeroizing<Z>
where
    Z: Zeroize,
{
    type Target = Z;

    fn deref(&self) -> &Z {
        &self.0
    }
}

impl<Z> ops::DerefMut for Zeroizing<Z>
where
    Z: Zeroize,
{
    fn deref_mut(&mut self) -> &mut Z {
        &mut self.0
    }
}

impl<Z> Zeroize for Zeroizing<Z>
where
    Z: Zeroize,
{
    fn zeroize(&mut self) {
        self.0.zeroize();
    }
}

// We could `derive(Zeroize)` for this, but doing it by hand allows `Zeroizing`
// to function regardless of whether the `zeroize_derive` feature is enabled
// or not.
impl<Z> Drop for Zeroizing<Z>
where
    Z: Zeroize,
{
    fn drop(&mut self) {
        self.0.zeroize()
    }
}

/// Use fences to prevent accesses from being reordered before this
/// point, which should hopefully help ensure that all accessors
/// see zeroes after this point.
#[inline]
fn atomic_fence() {
    atomic::compiler_fence(atomic::Ordering::SeqCst);
}

/// Perform a volatile write to the destination
#[inline]
fn volatile_write<T: Copy + Sized>(dst: &mut T, src: T) {
    unsafe { ptr::write_volatile(dst, src) }
}

/// Perform a volatile `memset` operation which fills a slice with a value
#[inline]
fn volatile_set<T: Copy + Sized>(dst: &mut [T], src: T) {
    // TODO(tarcieri): use `volatile_set_memory` on nightly?
    for elem in dst {
        volatile_write(elem, src);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::boxed::Box;

    #[test]
    fn zeroize_byte_arrays() {
        let mut arr = [42u8; 64];
        arr.zeroize();
        assert_eq!(arr.as_ref(), [0u8; 64].as_ref());
    }

    #[test]
    fn zeroize_vec() {
        let mut vec = vec![42; 3];
        vec.zeroize();
        assert!(vec.is_empty());
    }

    #[test]
    fn zeroize_vec_past_len() {
        let mut vec = Vec::with_capacity(5);
        for i in 0..4 {
            vec.push(10 + i);
        }
        vec.clear();

        // safe if: new_len <= capacity AND elements "were initialised"
        unsafe {
            vec.set_len(1);
        }
        assert_eq!(10, vec[0], "clear() hasn't erased our push()es");

        vec.clear();
        vec.zeroize();

        unsafe {
            vec.set_len(4);
        }
        for i in 0..4 {
            assert_eq!(0, vec[i], "it's been zero'd");
        }
    }

    #[test]
    fn zeroize_string() {
        let mut string = String::from("Hello, world!");
        string.zeroize();
        assert!(string.is_empty());
    }

    #[test]
    fn zeroize_box() {
        let mut boxed_arr = Box::new([42u8; 3]);
        boxed_arr.zeroize();
        assert_eq!(boxed_arr.as_ref(), &[0u8; 3]);
    }
}
