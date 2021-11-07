//! # Authenticated Encryption with Additional Data
//!
//! This operation:
//!
//! - Encrypts a message with a key and a nonce to keep it confidential
//! - Computes an authentication tag. This tag is used to make sure that the message,
//!   as well as optional, non-confidential (non-encrypted) data, haven't been tampered with.
//!
//! ## Example (combined mode)
//! ```
//! use crypto42::aead::xchacha20poly1305_ietf as aead;
//!
//! let key = aead::gen_key();
//! let nonce = aead::gen_nonce();
//! let plaintext = b"Some plaintext";
//! let ad = b"Some additional data";
//!
//! let ciphertext = aead::encrypt(plaintext, Some(ad), &nonce, &key);
//! let plaintext2 = aead::decrypt(&ciphertext, Some(ad), &nonce, &key).unwrap();
//!
//! assert_eq!(&plaintext[..], &plaintext2[..]);
//!```
//!
//! ## Example (detached mode)
//! ```
//! use crypto42::aead::xchacha20poly1305_ietf as aead;
//!
//! let key = aead::gen_key();
//! let nonce = aead::gen_nonce();
//! let mut plaintext_ciphertext = [0x41, 0x42, 0x43, 0x44];
//! let plaintext2 = plaintext_ciphertext.clone();
//! let ad = b"Some additional data";
//!
//! let tag = aead::encrypt_detached(&mut plaintext_ciphertext, Some(ad), &nonce, &key);
//! aead::decrypt_detached(&mut plaintext_ciphertext, Some(ad), &tag, &nonce, &key).unwrap();
//!
//! assert_eq!(plaintext_ciphertext, plaintext2);
//! ```

#[macro_use]
mod aead_macros;
pub mod xchacha20poly1305_ietf;
