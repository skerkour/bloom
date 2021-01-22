pub mod rand;

#[derive(thiserror::Error, Debug, Clone, Copy)]
pub enum Error {
    #[error("crypto: Unknown")]
    Unknown,
}

pub const AEAD_KEY_SIZE: usize = 32;
pub const KEY_SIZE_512: usize = 64;

pub fn hash_password(_password: &str) -> String {
    todo!(); // TODO
}

pub fn verify_password(_password: &str, _hash: &str) -> bool {
    todo!(); // TODO
}

pub fn aead_decrypt(_key: &[u8], _ciphertext: &[u8], _nonce: &[u8], _ad: &[u8]) -> Vec<u8> {
    todo!(); // TODO
}

pub fn aead_encrypt(_key: &[u8], _plaintext: &[u8], _ad: &[u8]) -> (Vec<u8>, Vec<u8>) {
    todo!(); // TODO
}

pub fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && constant_time_ne(a, b) == 0
}

// This function is non-inline to prevent the optimizer from looking inside it.
// taken from https://docs.rs/constant_time_eq/0.1.5/src/constant_time_eq/lib.rs.html#38-40
#[inline(never)]
fn constant_time_ne(a: &[u8], b: &[u8]) -> u8 {
    // These useless slices make the optimizer elide the bounds checks.
    // See the comment in clone_from_slice() added on Rust commit 6a7bc47.
    let len = a.len();
    let a = &a[..len];
    let b = &b[..len];

    let mut tmp = 0;
    for i in 0..len {
        tmp |= a[i] ^ b[i];
    }
    tmp // The compare with 0 must happen outside this function.
}

pub fn derive_key_from_key(key: &[u8], info: &[u8], key_size: usize) -> Result<Vec<u8>, Error> {
    todo!();
}
