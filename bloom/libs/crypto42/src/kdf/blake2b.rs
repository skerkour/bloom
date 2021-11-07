//! Deriving keys from a single high-entropy master key

use crate::{rand, Error};
use libc::{c_char, size_t};

/// Number of bytes in the master `Key` (32).
pub const KEYBYTES: usize = libsodium_sys::crypto_kdf_blake2b_KEYBYTES as usize;

/// Minimium of allowed bytes in the derived key
pub const BYTES_MIN: usize = libsodium_sys::crypto_kdf_blake2b_BYTES_MIN as usize;

/// Maximum of allowed bytes in the derived key
pub const BYTES_MAX: usize = libsodium_sys::crypto_kdf_blake2b_BYTES_MAX as usize;

/// Number of bytes of `context` (8)
pub const CONTEXTBYTES: usize = libsodium_sys::crypto_kdf_blake2b_CONTEXTBYTES as usize;

new_type! {
    /// `Key` for symmetric authenticated encryption with additional data.
    ///
    /// When a `Key` goes out of scope its contents will
    /// be zeroed out
    secret Key(KEYBYTES);
}

/// `gen_key()` randomly generates a secret master key
///
/// THREAD SAFETY: `gen_key()` is thread-safe provided that you have
/// called `crypto42::init()` once before using any other function
/// from crypto42.
pub fn gen_key() -> Key {
    let mut k = Key([0u8; KEYBYTES]);
    rand::bytes_into(&mut k.0);
    k
}

impl From<&[u8]> for Key {
    fn from(source: &[u8]) -> Self {
        let mut k = Key([0u8; KEYBYTES]);
        k.0[..KEYBYTES].clone_from_slice(source);
        return k;
    }
}

/// `derive_from_key()` derive subkeys from a single high-entropy master key.
/// Given the master key and a key identifier, a subkey can be deterministically computed.
/// However, given a subkey, an attacker cannot compute the master key nor any other subkeys.
///
/// `key_len` is the expected derived key length in bytes.
///
/// `key_id` can be any value up to `(2^64)-1`.
///
/// Similar to a type, the `context` is a 8 characters string describing what the key is going to be used for.
/// Its purpose is to mitigate accidental bugs by separating domains.
/// The same function used with the same key but in two distinct contexts is likely to generate two different outputs.
/// Contexts don't have to be secret and can have a low entropy.
/// Examples of contexts include `UserName`, `__auth__`, `pictures` and `userdata`.
/// If more convenient, it is also fine to use a single global context for a whole application.
///
/// Algorithm details:
/// `BLAKE2B-subkeylen(key=key, message={}, salt=subkey_id || {0}, personal=ctx || {0})`
pub fn derive_from_key(
    key_len: usize,
    key_id: u64,
    context: &str,
    master_key: &Key,
) -> Result<Vec<u8>, Error> {
    if context.len() != CONTEXTBYTES {
        return Err(Error::ContextBadSize);
    }

    if key_len < BYTES_MIN {
        return Err(Error::KeyTooShort);
    }

    if key_len > BYTES_MAX {
        return Err(Error::KeyTooLong);
    }

    let mut key = vec![0u8; key_len];

    let res = unsafe {
        libsodium_sys::crypto_kdf_derive_from_key(
            key.as_mut_ptr(),
            key_len as size_t,
            key_id,
            context.as_ptr() as *const c_char,
            master_key.0.as_ptr(),
        )
    };

    match res {
        0 => Ok(key),
        _ => Err(Error::Unknown),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // /// Helper function to allow for testing derive_from_key with different configurations
    // fn run_derive_from_key_test(
    //     key_len: usize,
    //     key_id: u64,
    //     context: &[u8],
    //     master_key: Key,
    // ) -> Result<(), ()> {
    //     let result = derive_from_key(key_len, key_id, context, master_key);

    //     match result {
    //         Ok(out_bin) => {
    //             assert_eq!(
    //                 out_bin.len(),
    //                 key_len,
    //                 "out key len does not match requested key_len"
    //             );
    //             Ok(())
    //         }
    //         Err(_) => Err(()),
    //     }
    // }

    #[test]
    fn test_derive_from_key() {
        let context = "__auth__";
        let key_len = 64;
        let master_key = gen_key();
        let mut current = Vec::new();

        for i in 0..=255 {
            let prev = current;
            current = derive_from_key(key_len, i, context, &master_key).unwrap();
            assert_ne!(prev, current, "derived keys are equals");
        }
    }
}
