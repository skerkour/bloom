//! Crate v_escape provides a macro, `new!` that define a `struct` with
//! escaping functionality. These macros are optimized using simd by default,
//! but this can be alter using sub-attributes.
//!
//! # Quick start
//! In order to use v_escape you will have to call one of the two macros
//! to create a escape `struct`. In this example, when using the macro
//! `new!(MyEscape, "62->bar");` a new a `struct` `MyEscape`
//! will be created that every time its method `MyEscape::fmt` is called
//! will replace all characters `">"` with `"bar"`.
//!
//! ```
//! v_escape::new!(MyEscape, "62->bar");
//!
//! # fn main() {
//! # let s = "foo>bar";
//! let escaped = escape(s);
//!
//! print!("{}", escaped);
//! # }
//! ```
//!
//! ## Pairs syntax
//! v_escape uses a simple syntax to replace characters
//! with their respective quotes. The tuple is named `Pair`,
//! and several can be defined, referred as `Pairs`. The syntax to define
//! `Pairs` consists of a character, followed
//! by the delimiter `->`, followed by the substitution quote
//! and the delimiter ` || ` (last delimiter is optional):
//!
//!    `([character]->[quote] || )*`
//!
//! * `character` :   Character to substitute. Accepts`i8+` from `0` to `i8::MAX` and
//!                 accepts the following formats: decimal (49), hexadecimal (0x31),
//!                 octal (0o61) or character (#1).
//!                 Note: Numbers are read in ASCII: `#6->foo`
//!
//! * `quote` :   Characters that will replace `character`
//!
//! ```
//! v_escape::new!(MyEscape, "49->bar");
//! # fn main() {
//! assert_eq!(escape("foo 1").to_string(), "foo bar");
//! # }
//! ```
//! ```
//! v_escape::new!(MyEscape, "0x31->bar");
//! # fn main() {
//! assert_eq!(escape("foo 1").to_string(), "foo bar");
//! # }
//! ```
//! ```
//! v_escape::new!(MyEscape, "0o61->bar");
//! # fn main() {
//! assert_eq!(escape("foo 1").to_string(), "foo bar");
//! # }
//! ```
//! ```
//! v_escape::new!(MyEscape, "#1->bar");
//! # fn main() {
//! assert_eq!(escape("foo 1").to_string(), "foo bar");
//! # }
//! ```
//!
//! In the following example more than 16 pairs are given, this exceeds simd's
//! boundary. If simd optimization is wanted, ranges must be enabled (default)
//! or an error will be thrown. It is possible to not use ranges but simd
//! optimization has to be disabled.
//!
//! ```
//! v_escape::new!(
//!     MyEscape,
//!     "62->b || 60->f || B->b || 65->f || 0o67->b || #6->f || 68->b || \
//!     71->f || 72->b || 73->f || 74->b || 75->f || 76->b || 77->f || \
//!     78->b || 79->f || 0x1A->f"
//! );
//! # fn main() {
//! assert_eq!(escape("foo>bar<").to_string(), "foobbarf");
//! # }
//! ```
//!
//! For debugging purposes, sub-attribute `print`, can be set to `true`
//! to print generated code
//!
//! ```
//! v_escape::new!(MyEscape, "o->bar", print = true);
//! # fn main() {
//! # assert_eq!(escape("foo").to_string(), "fbarbar");
//! # }
//! ```
//!
#![allow(unused_imports)]

pub use buf_min::Buffer;

pub use v_escape_derive::derive;

#[macro_use]
mod macros;
#[macro_use]
mod scalar;
#[macro_use]
mod ranges;
#[macro_use]
mod chars;

#[macro_export]
/// Generates struct `$name` with escaping functionality at `fmt`
///
/// It will get as input:
///
/// * $__name__: Name of escape class.
///
/// * $__pairs__: Pairs of `[character]->[quote] || [character]->[quote]` or
///              `[character]->[quote]`.
///
/// * $__t__: Optional boolean parameters (simd, avx, sse, print).
///     * __simd__:  If true (by default), simd optimizations are enabled. When false,
///         no matter value of avx, `sse4.2` will be used,
///     * __avx__:   If true (by default), avx optimization are enabled. When false,
///         `sse2`(if `ranges=true` and `simd=true`) or `scalar`(if `simd=false`) will be used.
///     * __ranges__:   If true (by default), ranges optimizations are enabled. When false,
///         `sse4.2`(if `simd=true`) or `scalar`(if `simd=false`) will be used.
///     * __print__: If true (false by default), prints out generated code to console.
///
/// and will:
///
/// 1. Import `std::fmt::{self, Display, Formatter}`
///
/// 2. Define basic struct with attribute `bytes` and `Escape`
///    derive functionality
///
/// 3. Implements for `$name` constructors `new` and `From<&'a str>`
///
/// 4. Implements trait `Display` for `$name` with escape functionality
///
/// 5. Implements function `escape(&str) -> $name`
///
/// #### Example
///
/// ```
/// use v_escape::new;
///
/// new!(MyEscape, "o->bar");
///
/// # fn main() {
/// assert_eq!(escape("foobar").to_string(), "fbarbarbar");
/// # }
/// ```
///
macro_rules! new {
    // Macro called without attributes
    ($name:ident, $pairs:expr) => {
        $crate::derive!($pairs);
        $crate::escape_new!($name);
    };
    // Macro called with attributes
    ($name:ident, $pairs:expr, $($t:tt)+) => {
        $crate::derive!($pairs, $($t)+);
        $crate::escape_new!($name);
    };
}

#[macro_export]
#[doc(hidden)]
/// Escape implementation
///
/// Generates function new, and traits From and Display, for class `$name`
macro_rules! escape_new {
    ($name:ident) => {
        pub struct $name<'a> {
            bytes: &'a [u8],
        }

        impl<'a> $name<'a> {
            #[inline]
            pub fn new(bytes: &[u8]) -> $name {
                $name { bytes }
            }

            #[inline]
            pub fn f_escape(&self, buf: &mut [std::mem::MaybeUninit<u8>]) -> Option<usize> {
                #[allow(unused_unsafe)]
                unsafe {
                    _f_escape(self.bytes, buf)
                }
            }
        }

        impl<'a> From<&'a str> for $name<'a> {
            #[inline]
            fn from(s: &str) -> $name {
                $name {
                    bytes: s.as_bytes(),
                }
            }
        }

        #[inline]
        pub fn escape(s: &str) -> $name {
            $name::from(s)
        }

        impl<'a> std::fmt::Display for $name<'a> {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                #[allow(unused_unsafe)]
                unsafe {
                    _escape(self.bytes, fmt)
                }
            }
        }

        #[inline]
        pub fn escape_char(c: char) -> impl std::fmt::Display {
            struct EscapeChar(char);

            impl std::fmt::Display for EscapeChar {
                fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                    chars::escape_char(self.0, fmt)
                }
            }

            EscapeChar(c)
        }

        #[inline]
        pub fn f_escape(s: &[u8], buf: &mut [std::mem::MaybeUninit<u8>]) -> Option<usize> {
            #[allow(unused_unsafe)]
            unsafe {
                _f_escape(s, buf)
            }
        }

        #[inline]
        pub fn f_escape_char(c: char, buf: &mut [std::mem::MaybeUninit<u8>]) -> Option<usize> {
            #[allow(unused_unsafe)]
            unsafe {
                chars::f_escape_char(c, buf)
            }
        }

        /// Escape byte slice to `Buffer`
        ///
        /// # SIGILL
        /// Can produce **SIGILL** if compile with `sse2` or `avx2` and execute without they
        /// Because not exist way to build multiple static allocations by type
        /// And it's very expensive check it in runtime
        /// https://github.com/rust-lang/rust/issues/57775
        #[inline]
        pub fn b_escape<B: $crate::Buffer>(s: &[u8], buf: &mut B) {
            #[allow(unused_unsafe)]
            unsafe {
                _b_escape(s, buf)
            }
        }

        /// Escape char to `buf-min::Buffer`
        #[inline]
        pub fn b_escape_char<B: $crate::Buffer>(s: char, buf: &mut B) {
            #[allow(unused_unsafe)]
            unsafe {
                chars::b_escape_char(s, buf)
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
/// cfg_if for escape function
macro_rules! cfg_escape {
    (false, $($t:tt)+) => {
        $crate::cfg_escape!(fn);
    };
    (true, $($t:tt)+) => {
        #[cfg(target_arch = "x86_64")]
        #[inline(always)]
        // https://github.com/BurntSushi/rust-memchr/blob/master/src/x86/mod.rs#L9-L29
        fn _escape(bytes: &[u8], fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            use std::mem;
            use std::sync::atomic::{AtomicUsize, Ordering};
            use std::fmt::{self, Formatter};
            static mut FN: fn(&[u8], &mut Formatter) -> fmt::Result = detect;

            fn detect(bytes: &[u8], fmt: &mut Formatter) -> fmt::Result {
                let fun = $crate::cfg_escape!(if $($t)+);

                let slot = unsafe { &*(&FN as *const _ as *const AtomicUsize) };
                slot.store(fun, Ordering::Relaxed);
                unsafe {
                    mem::transmute::<usize, fn(&[u8], &mut Formatter) -> fmt::Result>(fun)(
                        bytes, fmt,
                    )
                }
            }

            unsafe {
                let slot = &*(&FN as *const _ as *const AtomicUsize);
                let fun = slot.load(Ordering::Relaxed);
                mem::transmute::<usize, fn(&[u8], &mut Formatter) -> fmt::Result>(fun)(bytes, fmt)
            }
        }

        #[cfg(not(target_arch = "x86_64"))]
        $crate::cfg_escape!(fn);
    };
    (fn) => {
        #[inline(always)]
        fn _escape(bytes: &[u8], fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            scalar::escape(bytes, fmt)
        }
    };
    (if true) => {
        if is_x86_feature_detected!("avx2") {
            ranges::avx::escape as usize
        } else if is_x86_feature_detected!("sse2") {
            ranges::sse::escape as usize
        } else {
            scalar::escape as usize
        }
    };
    (if false) => {
        if is_x86_feature_detected!("sse2") {
            ranges::sse::escape as usize
        } else {
            scalar::escape as usize
        }
    };
}

#[macro_export]
#[doc(hidden)]
/// cfg_if for escape function
macro_rules! cfg_escape_ptr {
    (false, $($t:tt)+) => {
        $crate::cfg_escape_ptr!(fn);
    };
    (true, $($t:tt)+) => {
        #[cfg(target_arch = "x86_64")]
        #[inline(always)]
        #[allow(unreachable_code)]
        // https://github.com/BurntSushi/rust-memchr/blob/master/src/x86/mod.rs#L9-L29
        pub unsafe fn _f_escape(bytes: &[u8], buf: &mut [std::mem::MaybeUninit<u8>]) -> Option<usize> {
            use std::mem;
            use std::sync::atomic::{AtomicUsize, Ordering};
            static mut FN: fn(&[u8], &mut [std::mem::MaybeUninit<u8>]) -> Option<usize> = detect;

            fn detect(bytes: &[u8], buf: &mut [std::mem::MaybeUninit<u8>]) -> Option<usize> {
                let fun = $crate::cfg_escape_ptr!(if $($t)+);

                let slot = unsafe { &*(&FN as *const _ as *const AtomicUsize) };
                slot.store(fun, Ordering::Relaxed);
                unsafe {
                    mem::transmute::<usize, fn(&[u8], &mut [std::mem::MaybeUninit<u8>]) -> Option<usize>>(fun)(
                        bytes, buf,
                    )
                }
            }

            unsafe {
                let slot = &*(&FN as *const _ as *const AtomicUsize);
                let fun = slot.load(Ordering::Relaxed);
                mem::transmute::<usize, fn(&[u8], &mut [std::mem::MaybeUninit<u8>]) -> Option<usize>>(fun)(bytes, buf)
            }
        }

        #[cfg(not(target_arch = "x86_64"))]
        $crate::cfg_escape_ptr!(fn);
    };
    (fn) => {
        #[inline(always)]
        pub unsafe fn _f_escape(bytes: &[u8], buf: &mut [std::mem::MaybeUninit<u8>]) -> Option<usize> {
            scalar::f_escape(bytes, buf)
        }
    };
    (if true) => {
        if is_x86_feature_detected!("avx2") {
            ranges::avx::f_escape as usize
        } else if is_x86_feature_detected!("sse2") {
            ranges::sse::f_escape as usize
        } else {
            scalar::f_escape as usize
        }
    };
    (if false) => {
        if is_x86_feature_detected!("sse2") {
            ranges::sse::f_escape as usize
        } else {
            scalar::f_escape as usize
        }
    };
}

#[macro_export]
#[doc(hidden)]
/// cfg_if for escape function
macro_rules! cfg_escape_bytes {
    (false, $($t:tt)+) => {
        $crate::cfg_escape_bytes!(fn);
    };
    (true, $($t:tt)+) => {
        #[cfg(target_arch = "x86_64")]
        #[inline(always)]
        pub unsafe fn _b_escape<B: $crate::Buffer>(bytes: &[u8], buf: &mut B) {
            $crate::cfg_escape_bytes!(if $($t)+, bytes, buf)
        }

        #[cfg(not(all(target_arch = "x86_64", not(b_escape_nosimd))))]
        $crate::cfg_escape_bytes!(fn);
    };
    (fn) => {
        #[inline(always)]
        pub unsafe fn _b_escape<B: $crate::Buffer>(bytes: &[u8], buf: &mut B) {
            scalar::b_escape(bytes, buf)
        }
    };
    (if true, $bytes:ident, $buf:ident) => {{
        #[cfg(not(v_escape_avx))] {
            #[cfg(not(v_escape_sse))] {
                scalar::b_escape($bytes, $buf)
            }
            #[cfg(v_escape_sse)] {
                ranges::sse::b_escape($bytes, $buf)
            }
        }
        #[cfg(v_escape_avx)] {
            ranges::avx::b_escape($bytes, $buf)
        }
    }};
    (if false, $bytes:ident, $buf:ident) => {{
        #[cfg(not(v_escape_sse))] {
            scalar::b_escape($bytes, $buf)
        }
        #[cfg(v_escape_sse)] {
            ranges::sse::b_escape($bytes, $buf)
        }
    }};
}
