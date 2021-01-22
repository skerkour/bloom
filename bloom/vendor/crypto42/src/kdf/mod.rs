//! # Key Derivation Functions
//!
//! Secret keys used to encrypt or sign confidential data have to be chosen from
//! a very large keyspace. However, passwords are usually short, human-generated
//! strings, making dictionary attacks practical.
//!
//! The kdf operation derives a secret key of any size from a password and a
//! salt.
//!
//! - The generated key has the size defined by the application, no matter what
//!   the password length is.
//! - The same password hashed with same parameters will
//!   always produce the same key.
//! - The same password hashed with different salts
//!   will produce different keys.
//! - The function deriving a key from a password
//!   and a salt is CPU intensive and intentionally requires a fair amount of
//!   memory. Therefore, it mitigates brute-force attacks by requiring a
//!   significant effort to verify each password.
//!
//! Common use cases:
//!
//! - Protecting an on-disk secret key with a password,
//! - Password storage, or rather: storing what it takes to verify a password
//!   without having to store the actual password.
//!
//! ## Available algorithms
//! -----------------------
//! |Algorithm | Source   |
//! |----------|----------|
//! |`Argon2id` | Password  |
//! |`Blake2b`   | High Entropy (such as random data) |
//!
//! ## Example (key derivation from password)
//!
//! ```
//! use crypto42::kdf::argon2id;
//!
//! let passwd = b"Correct Horse Battery Staple";
//! let salt = argon2id::gen_salt();
//! // derive a 512 bits (64 bytes key)
//! let key = argon2id::derive_from_password(64, passwd, &salt,
//!                        argon2id::OPSLIMIT_INTERACTIVE,
//!                        argon2id::MEMLIMIT_INTERACTIVE).unwrap();
//! ```
//!
//! ## Example (key derivation from high entropy source)
//!
//! ```
//! use crypto42::kdf::blake2b;
//!
//! let context = "__auth__";
//! let master_key = blake2b::gen_key();

//! // derive 512 bits keys
//! let key1 = blake2b::derive_from_key(64, 1, context, &master_key).expect("error deriving from key");
//! let key2 = blake2b::derive_from_key(64, 2, context, &master_key).expect("error deriving from key");
//! let key3 = blake2b::derive_from_key(64, 3, context, &master_key).expect("error deriving from key");
//! ```
//!
//! ## Example (password hashing)
//!
//! ```
//! use crypto42::kdf::argon2id;
//!
//! let passwd = b"Correct Horse Battery Staple";
//! let hashed = argon2id::hash_password(passwd,
//!                          argon2id::OPSLIMIT_INTERACTIVE,
//!                          argon2id::MEMLIMIT_INTERACTIVE).unwrap();
//! let hashed_bytes = hashed.as_ref();
//! // store hashed_bytes somewhere
//! ```
//!
//! ## Example (password verification)
//!
//! ```
//! use crypto42::kdf::argon2id;
//!
//! let passwd = b"Correct Horse Battery Staple";
//! // in reality we want to load the password hash from somewhere
//! // and we might want to create a `HashedPassword` from it using
//! // `HashedPassword::from_slice(kdf_bytes).unwrap()`
//! let hashed = argon2id::hash_password(passwd,
//!                          argon2id::OPSLIMIT_INTERACTIVE,
//!                          argon2id::MEMLIMIT_INTERACTIVE).unwrap();
//! assert!(argon2id::verify_password(&hashed, passwd));
//! ```

#[macro_use]
mod argon2_macros;

// pub mod argon2i;
pub mod argon2id;
pub mod blake2b;
// pub mod scrypt;
