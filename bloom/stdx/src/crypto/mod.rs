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
    todo!();
}

pub fn derive_key_from_key(key: &[u8], info: &[u8], key_size: usize) -> Result<Vec<u8>, Error> {
    todo!();
}
