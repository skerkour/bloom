pub mod rand;

pub fn hash_password(_password: &str) -> String {
    unimplemented!(); // TODO
}

pub fn verify_password(_password: &str, _hash: &str) -> bool {
    unimplemented!(); // TODO
}

pub fn aead_decrypt(_key: &[u8], _ciphertext: &[u8], _nonce: &[u8], _ad: &[u8]) -> Vec<u8> {
    unimplemented!(); // TODO
}

pub fn aead_encrypt(_key: &[u8], _plaintext: &[u8], _ad: &[u8]) -> (Vec<u8>, Vec<u8>) {
    unimplemented!(); // TODO
}
