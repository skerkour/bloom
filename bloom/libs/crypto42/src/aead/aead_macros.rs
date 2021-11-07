macro_rules! aead_module (($encrypt_name:ident,
                           $decrypt_name:ident,
                           $encrypt_detached_name:ident,
                           $decrypt_detached_name:ident,
                           $keybytes:expr,
                           $noncebytes:expr,
                           $tagbytes:expr) => (

#[cfg(not(feature = "std"))] use prelude::*;
use libc::c_ulonglong;
use std::ptr;
use crate::{Error, rand};

/// Number of bytes in a `Key`.
pub const KEYBYTES: usize = $keybytes;

/// Number of bytes in a `Nonce`.
pub const NONCEBYTES: usize = $noncebytes;

/// Number of bytes in an authentication `Tag`.
pub const TAGBYTES: usize = $tagbytes;

new_type! {
    /// `Key` for symmetric authenticated encryption with additional data.
    ///
    /// When a `Key` goes out of scope its contents will
    /// be zeroed out
    secret Key(KEYBYTES);
}

new_type! {
    /// `Nonce` for symmetric authenticated encryption with additional data.
    nonce Nonce(NONCEBYTES);
}

new_type! {
    /// Authentication `Tag` for symmetric authenticated encryption with additional data in
    /// detached mode.
    public Tag(TAGBYTES);
}

/// `gen_key()` randomly generates a secret key
///
/// THREAD SAFETY: `gen_key()` is thread-safe provided that you have
/// called `crypto42::init()` once before using any other function
/// from crypto42.
pub fn gen_key() -> Key {
    let mut k = Key([0u8; KEYBYTES]);
    rand::bytes_into(&mut k.0);
    k
}

/// `gen_nonce()` randomly generates a nonce
///
/// THREAD SAFETY: `gen_key()` is thread-safe provided that you have
/// called `crypto42::init()` once before using any other function
/// from crypto42.
pub fn gen_nonce() -> Nonce {
    let mut n = Nonce([0u8; NONCEBYTES]);
    rand::bytes_into(&mut n.0);
    n
}

/// `encrypt()` encrypts and authenticates a message `m` together with optional plaintext data `ad`
/// using a secret key `k` and a nonce `n`. It returns a ciphertext `c`.
pub fn encrypt(m: &[u8], ad: Option<&[u8]>, n: &Nonce, k: &Key) -> Vec<u8> {
    let (ad_p, ad_len) = ad.map(|ad| (ad.as_ptr(), ad.len() as c_ulonglong)).unwrap_or((ptr::null(), 0));
    let mut c = vec![0u8; m.len() + TAGBYTES];
    let mut clen = c.len() as c_ulonglong;

    unsafe {
        $encrypt_name(
            c.as_mut_ptr(),
            &mut clen,
            m.as_ptr(),
            m.len() as c_ulonglong,
            ad_p,
            ad_len,
            ptr::null_mut(),
            n.0.as_ptr(),
            k.0.as_ptr()
        );
        // c.set_len(clen as usize);
    }
    c
}

/// `encrypt_detached()` encrypts and authenticates a message `m` together with optional plaintext data
/// `ad` using a secret key `k` and a nonce `n`.
/// `m` is encrypted in place, so after this function returns it will contain the ciphertext.
/// The detached authentication tag is returned by value.
pub fn encrypt_detached(m: &mut [u8], ad: Option<&[u8]>, n: &Nonce, k: &Key) -> Tag {
    let (ad_p, ad_len) = ad.map(|ad| (ad.as_ptr(), ad.len() as c_ulonglong)).unwrap_or((ptr::null(), 0));
    let mut tag = Tag([0u8; TAGBYTES]);
    let mut maclen = TAGBYTES as c_ulonglong;
    unsafe {
        $encrypt_detached_name(
            m.as_mut_ptr(),
            tag.0.as_mut_ptr(),
            &mut maclen,
            m.as_ptr(),
            m.len() as c_ulonglong,
            ad_p,
            ad_len,
            ptr::null_mut(),
            n.0.as_ptr(),
            k.0.as_ptr()
        );
    }
    tag
}

/// `decrypt()` verifies and decrypts a ciphertext `c` together with optional plaintext data `ad`
/// using a secret key `k` and a nonce `n`.
/// It returns a plaintext `Ok(m)`.
/// If the ciphertext fails verification, `decrypt()` returns `Err(())`.
pub fn decrypt(c: &[u8], ad: Option<&[u8]>, n: &Nonce, k: &Key) -> Result<Vec<u8>, Error> {
    if c.len() < TAGBYTES {
        return Err(Error::TagTooShort);
    }
    let (ad_p, ad_len) = ad.map(|ad| (ad.as_ptr(), ad.len() as c_ulonglong)).unwrap_or((ptr::null(), 0));
    let mut m = vec![0u8; c.len() - TAGBYTES];
    let mut mlen = m.len() as c_ulonglong;

    unsafe {
        let ret =
            $decrypt_name(
                m.as_mut_ptr(),
                &mut mlen,
                ptr::null_mut(),
                c.as_ptr(),
                c.len() as c_ulonglong,
                ad_p,
                ad_len,
                n.0.as_ptr(),
                k.0.as_ptr()
            );
        if ret != 0 {
            return Err(Error::Unknown);
        }
        // m.set_len(mlen as usize);
    }
    Ok(m)
}
/// `decrypt_detached()` verifies and decrypts a ciphertext `c` toghether with optional plaintext data
/// `ad` and and authentication tag `tag`, using a secret key `k` and a nonce `n`.
/// `c` is decrypted in place, so if this function is successful it will contain the plaintext.
/// If the ciphertext fails verification, `decrypt_detached()` returns `Err(())`,
/// and the ciphertext is not modified.
pub fn decrypt_detached(c: &mut [u8], ad: Option<&[u8]>, t: &Tag, n: &Nonce, k: &Key) -> Result<(), Error> {
    let (ad_p, ad_len) = ad.map(|ad| (ad.as_ptr(), ad.len() as c_ulonglong)).unwrap_or((ptr::null(), 0));
    let ret = unsafe {
        $decrypt_detached_name(
            c.as_mut_ptr(),
            ptr::null_mut(),
            c.as_ptr(),
            c.len() as c_ulonglong,
            t.0.as_ptr(),
            ad_p,
            ad_len,
            n.0.as_ptr(),
            k.0.as_ptr()
        )
    };
    if ret == 0 {
        Ok(())
    } else {
        Err(Error::Unknown)
    }
}

#[cfg(test)]
mod test_m {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        use crate::rand;

        for i in 0..256usize {
            let k = gen_key();
            let n = gen_nonce();
            let ad = rand::bytes(i);
            let m = rand::bytes(i);
            let c = encrypt(&m, Some(&ad), &n, &k);
            let m2 = decrypt(&c, Some(&ad), &n, &k).unwrap();
            assert_eq!(m, m2);
        }
    }

    #[test]
    fn test_encrypt_decrypt_tamper() {
        use crate::rand;

        for i in 0..32usize {
            let k = gen_key();
            let n = gen_nonce();
            let mut ad = rand::bytes(i);
            let m = rand::bytes(i);
            let mut c = encrypt(&m, Some(&ad), &n, &k);
            for j in 0..c.len() {
                c[j] ^= 0x20;
                let m2 = decrypt(&c, Some(&ad), &n, &k);
                c[j] ^= 0x20;
                assert!(m2.is_err());
            }
            for j in 0..ad.len() {
                ad[j] ^= 0x20;
                let m2 = decrypt(&c, Some(&ad), &n, &k);
                ad[j] ^= 0x20;
                assert!(m2.is_err());
            }
        }
    }

    #[test]
    fn test_encrypt_decrypt_detached() {
        use crate::rand;

        for i in 0..256usize {
            let k = gen_key();
            let n = gen_nonce();
            let ad = rand::bytes(i);
            let mut m = rand::bytes(i);
            let m2 = m.clone();
            let t = encrypt_detached(&mut m, Some(&ad), &n, &k);
            decrypt_detached(&mut m, Some(&ad), &t, &n, &k).unwrap();
            assert_eq!(m, m2);
        }
    }

    #[test]
    fn test_encrypt_decrypt_detached_tamper() {
        use crate::rand;

        for i in 0..32usize {
            let k = gen_key();
            let n = gen_nonce();
            let mut ad = rand::bytes(i);
            let mut m = rand::bytes(i);
            let mut t = encrypt_detached(&mut m, Some(&ad), &n, &k);
            for j in 0..m.len() {
                m[j] ^= 0x20;
                let r = decrypt_detached(&mut m, Some(&ad), &t, &n, &k);
                m[j] ^= 0x20;
                assert!(r.is_err());
            }
            for j in 0..ad.len() {
                ad[j] ^= 0x20;
                let r = decrypt_detached(&mut m, Some(&ad), &t, &n, &k);
                ad[j] ^= 0x20;
                assert!(r.is_err());
            }
            for j in 0..t.0.len() {
                t.0[j] ^= 0x20;
                let r = decrypt_detached(&mut m, Some(&ad), &t, &n, &k);
                t.0[j] ^= 0x20;
                assert!(r.is_err());
            }
        }
    }

    #[test]
    fn test_encrypt_decrypt_detached_same() {
        use crate::rand;

        for i in 0..256usize {
            let k = gen_key();
            let n = gen_nonce();
            let ad = rand::bytes(i);
            let mut m = rand::bytes(i);

            let c = encrypt(&m, Some(&ad), &n, &k);
            let t = encrypt_detached(&mut m, Some(&ad), &n, &k);

            assert_eq!(&c[0..c.len()-TAGBYTES], &m[..]);
            assert_eq!(&c[c.len()-TAGBYTES..], &t.0[..]);

            let m2 = decrypt(&c, Some(&ad), &n, &k).unwrap();
            decrypt_detached(&mut m, Some(&ad), &t, &n, &k).unwrap();

            assert_eq!(m2, m);
        }
    }
}

));
