//! # One-way Hash functions
//!
//! ## Security model
//!
//! The `hash()` function is designed to be usable as a strong
//! component of DSA, RSA-PSS, key derivation, hash-based
//! message-authentication codes, hash-based ciphers, and various other
//! common applications. "Strong" means that the security of these
//! applications, when instantiated with `hash()`, is the same
//! as the security of the applications against generic attacks. In
//! particular, the `hash()` function is designed to make
//! finding collisions difficult.
//!
//! ## Available algorithm
//!
//! -----------------------
//! |algorithm |BYTES     |
//! |----------|----------|
//! |`BLAKE2B` | Variable |
//!
//! ## Example
//!
//! ```
//! use crypto42::hash::blake2b;
//!
//! let data_to_hash = b"some data!";
//! let digest = blake2b::hash(data_to_hash, None).expect("error hashing data");
//!
//! // or
//! let mut hash_state = blake2b::State::new(blake2b::DIGEST_512, None).expect("error initializing state");
//! hash_state.update(b"some ");
//! hash_state.update(b"data!");
//! let digest2 = hash_state.finalize().expect("error finalizing hash");
//!
//! assert_eq!(digest, digest2);
//! ```

#[macro_use]
mod hash_macros;

pub mod blake2b;
// pub mod sha2_256;
// pub mod sha2_512;
