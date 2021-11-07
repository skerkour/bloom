use crypto42::{aead::xchacha20poly1305_ietf, hash::blake2b, kdf::argon2id};

#[derive(thiserror::Error, Debug, Clone, Copy)]
pub enum Error {
    #[error("crypto: Unknown")]
    Unknown,
}

pub const AEAD_KEY_SIZE: usize = 32;
pub const KEY_SIZE_512: usize = 64;

pub fn init() -> Result<(), Error> {
    crypto42::init().map_err(|_| Error::Unknown)?;
    Ok(())
}

pub fn hash_password(password: &str) -> Result<String, Error> {
    let hash_str = argon2id::hash_password(
        password.as_bytes(),
        argon2id::OPSLIMIT_INTERACTIVE,
        argon2id::MEMLIMIT_INTERACTIVE,
    )
    .map_err(|_| Error::Unknown)?
    .to_string();

    Ok(hash_str)
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let hashed = argon2id::HashedPassword::from(hash);

    argon2id::verify_password(&hashed, password.as_bytes())
}

pub fn aead_decrypt(
    key: &[u8],
    ciphertext: &[u8],
    nonce: &[u8],
    ad: &[u8],
) -> Result<Vec<u8>, Error> {
    let key = xchacha20poly1305_ietf::Key::from_slice(key).ok_or(Error::Unknown)?;
    let nonce = xchacha20poly1305_ietf::Nonce::from_slice(nonce).ok_or(Error::Unknown)?;

    let plaintext = xchacha20poly1305_ietf::decrypt(ciphertext, Some(ad), &nonce, &key)
        .map_err(|_| Error::Unknown)?;
    Ok(plaintext)
}

pub fn aead_encrypt(key: &[u8], plaintext: &[u8], ad: &[u8]) -> Result<(Vec<u8>, Vec<u8>), Error> {
    let nonce = xchacha20poly1305_ietf::gen_nonce();
    let key = xchacha20poly1305_ietf::Key::from_slice(key).ok_or(Error::Unknown)?;

    let ciphertext = xchacha20poly1305_ietf::encrypt(plaintext, Some(ad), &nonce, &key);
    Ok((ciphertext, nonce.0.into()))
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

pub fn derive_key_from_key(key: &[u8], data: &[u8], key_size: usize) -> Result<Vec<u8>, Error> {
    let mut hash_state = blake2b::State::new(key_size, Some(key)).map_err(|_| Error::Unknown)?;
    hash_state.update(data).map_err(|_| Error::Unknown)?;
    let digest = hash_state.finalize().map_err(|_| Error::Unknown)?;

    Ok(digest.as_ref().into())
}
