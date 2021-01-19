pub mod rand;

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
