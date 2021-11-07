//! # Public-key signatures
//!
//! ## Security model
//! The [`sign()`](ed25519/fn.sign.html) function is designed to meet the standard
//! notion of unforgeability for a public-key signature scheme under
//! chosen-message attacks.
//!
//! ## Algorithms
//!
//! ----------------------------------------------------------------------------------
//! |`crypto_sign`                         | PUBLICKEYBYTES | SECRETKEYBYTES | BYTES |
//! |--------------------------------------|----------------|----------------|-------|
//! |`crypto_sign_ed25519`                 | 32             | 64             | 64    |
//!
//! ## Example
//! ```
//! use crypto42::sign::ed25519;
//!
//! let (pk, sk) = ed25519::gen_keypair();
//! let data_to_sign = b"some data";
//! let signature = ed25519::sign(data_to_sign, &sk);
//! assert!(ed25519::verify(&signature, data_to_sign, &pk));
//! ```
//!
//! ## Example (streaming)
//! ```
//! use crypto42::sign::ed25519;
//!
//! let (pk, sk) = ed25519::gen_keypair();
//! let data_to_sign = b"some data";
//!
//! let mut sign_state = ed25519::State::init();
//! sign_state.update(&data_to_sign[0..4]);
//! sign_state.update(&data_to_sign[4..]);
//! let signature = sign_state.finalize(&sk);
//!
//! let mut verify_state = ed25519::State::init();
//! verify_state.update(&data_to_sign[0..4]);
//! verify_state.update(&data_to_sign[4..]);
//! assert!(verify_state.verify(&signature, &pk));
//! ```
//!
//! ## Example (combined signature)
//! ```
//! use crypto42::sign::ed25519;
//!
//! let (pk, sk) = ed25519::gen_keypair();
//! let data_to_sign = b"some data";
//! let signed_data = ed25519::sign_combined(data_to_sign, &sk);
//! let verified_data = ed25519::verify_combined(&signed_data, &pk).unwrap();
//! assert!(data_to_sign == &verified_data[..]);
//! ```
pub mod ed25519;
