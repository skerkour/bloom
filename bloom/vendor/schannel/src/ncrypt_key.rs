//! CNG private keys.
use winapi::um::ncrypt;

/// A CNG handle to a key.
pub struct NcryptKey(ncrypt::NCRYPT_KEY_HANDLE);

impl Drop for NcryptKey {
    fn drop(&mut self) {
        unsafe {
            ncrypt::NCryptFreeObject(self.0);
        }
    }
}

inner!(NcryptKey, ncrypt::NCRYPT_KEY_HANDLE);
