//! CryptoAPI private keys.
use winapi::um::wincrypt;

/// A handle to a key.
pub struct CryptKey(wincrypt::HCRYPTKEY);

impl Drop for CryptKey {
    fn drop(&mut self) {
        unsafe {
            wincrypt::CryptDestroyKey(self.0);
        }
    }
}

inner!(CryptKey, wincrypt::HCRYPTKEY);
