//! # Cryptographic Random data generation

use libsodium_sys;
#[cfg(not(feature = "std"))]
use prelude::*;

/// `bytes()` randomly generates size bytes of data.
///
/// THREAD SAFETY: `bytes()` is thread-safe provided that you have
/// called `crypto42::init()` once before using any other function
/// from crypto42.
pub fn bytes(size: usize) -> Vec<u8> {
    unsafe {
        let mut buf = vec![0u8; size];
        libsodium_sys::randombytes_buf(buf.as_mut_ptr() as *mut _, size);
        buf
    }
}

/// `bytes_into()` fills a buffer `buf` with random data.
///
/// THREAD SAFETY: `bytes_into()` is thread-safe provided that you have
/// called `crypto42::init()` once before using any other function
/// from crypto42.
pub fn bytes_into(buf: &mut [u8]) {
    unsafe {
        libsodium_sys::randombytes_buf(buf.as_mut_ptr() as *mut _, buf.len());
    }
}

/// `uniform()` returns an unpredictable value between 0 and
/// `upper_bound` (excluded). It guarantees a uniform distribution of the
/// possible output values even when `upper_bound` is not a power of 2. Note
/// that an `upper_bound` < 2 leaves only a  single element to be chosen, namely
/// 0.
///
/// THREAD SAFETY: `uniform()` is thread-safe provided that you have
/// called `crypto42::init()` once before using any other function
/// from crypto42.
pub fn uniform(upper_bound: u32) -> u32 {
    unsafe { libsodium_sys::randombytes_uniform(upper_bound) }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_uniform_0() {
        crate::init().unwrap();

        assert_eq!(uniform(0), 0);
    }

    #[test]
    fn test_uniform_1() {
        crate::init().unwrap();

        assert_eq!(uniform(1), 0);
    }

    #[test]
    fn test_uniform_7() {
        crate::init().unwrap();

        assert!(uniform(7) < 7);
    }
}
