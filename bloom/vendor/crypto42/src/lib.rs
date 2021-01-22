//! `crypto42` is a cross-platform, secure, easy to use, and hard to misuse cryptographic library in Rust,
//! using [libsodium](https://github.com/jedisct1/libsodium) as backend.
//!
//! High level documentation: [https://theguide.bloom.sh/projects/crypto42](https://theguide.bloom.sh/projects/crypto42)
//!
//! Repository: [https://gitlab.com/bloom42/libs/crypto42-rs](https://gitlab.com/bloom42/libs/crypto42-rs)
//!
//! ## Usage
//!
//! ```toml
//! [dependencies]
//! crypto42 = { version = "0.5", git = "https://gitlab.com/bloom42/libs/crypto42-rs" }
//! ```
//!
//! See [https://gitlab.com/bloom42/libs/crypto42-rs/tree/dev/examples](https://gitlab.com/bloom42/libs/crypto42-rs/tree/dev/examples)
//! for complete examples.
//!
//! ## Thread Safety
//! All functions in this library are thread-safe provided that the [`init()`](fn.init.html)
//! function has been called during program execution.
//!
//! If [`init()`](fn.init.html) hasn't been called then all functions except the random-number
//! generation functions and the key-generation functions are thread-safe.
//!
// # Authenticated Encryption with Associated Data
//  [`aead`](aead/index.html)
//
// # One-way Hash functions
//  [`hash`](hash/index.html)
//
// # Key Derivation Functions
//  [`kdf`](kdf/index.html)
//
// # Public-key signatures
//  [`sign`](sign/index.html)
//
// # Random data
//  [`rand`](rand/index.html)
//
// # Constant time comparison
//  [`cmp`](cmp/index.html)

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;
#[cfg(all(test, not(feature = "std")))]
extern crate std;

#[cfg(all(not(test), not(feature = "std")))]
mod std {
    pub use core::{cmp, fmt, hash, iter, mem, ops, ptr, slice, str};
}

#[cfg(not(feature = "std"))]
mod prelude {
    pub use alloc::string::String;
    pub use alloc::vec::Vec;
}

/// `init()` initializes the sodium library and chooses faster versions of
/// the primitives if possible. `init()` also makes the random number generation
/// functions (`gen_key`, `gen_keypair`, `gen_nonce`, `randombytes`, `randombytes_into`)
/// thread-safe
///
/// `init()` returns `Ok` if initialization succeeded and `Err` if it failed.
pub fn init() -> Result<(), Error> {
    if unsafe { libsodium_sys::sodium_init() } >= 0 {
        Ok(())
    } else {
        Err(Error::InitializationFailed)
    }
}

#[macro_use]
mod newtype_macros;

mod c;
mod error;

pub use error::Error;

pub mod aead;
pub mod cmp;
pub mod hash;
pub mod kdf;
pub mod kx;
pub mod rand;
pub mod sign;
pub mod streaming_aead;
pub mod utils;
pub mod zeroize;
