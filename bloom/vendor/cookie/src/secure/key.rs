const PRIVATE_KEY_LEN: usize = 32;
const SIGNED_KEY_LEN: usize = 32;
const COMBINED_KEY_LENGTH: usize = PRIVATE_KEY_LEN + SIGNED_KEY_LEN;

// Statically ensure the numbers above are in-sync.
#[cfg(feature = "private")]
const_assert!(crate::secure::private::KEY_LEN == PRIVATE_KEY_LEN);
#[cfg(feature = "signed")]
const_assert!(crate::secure::signed::KEY_LEN == SIGNED_KEY_LEN);

/// A cryptographic master key for use with `Signed` and/or `Private` jars.
///
/// This structure encapsulates secure, cryptographic keys for use with both
/// [`PrivateJar`](crate::PrivateJar) and [`SignedJar`](crate::SignedJar). A
/// single instance of a `Key` can be used for both a `PrivateJar` and a
/// `SignedJar` simultaneously with no notable security implications.
#[cfg_attr(nightly, doc(cfg(any(feature = "private", feature = "signed"))))]
#[derive(Clone)]
pub struct Key {
    pub(crate) signing: [u8; SIGNED_KEY_LEN],
    pub(crate) encryption: [u8; PRIVATE_KEY_LEN]
}

impl Key {
    // An empty key structure, to be filled.
    const fn zero() -> Self {
        Key { signing: [0; SIGNED_KEY_LEN], encryption: [0; PRIVATE_KEY_LEN] }
    }

    /// Creates a new `Key` from a 512-bit cryptographically random string.
    ///
    /// The supplied key must be at least 512-bits (64 bytes). For security, the
    /// master key _must_ be cryptographically random.
    ///
    /// # Panics
    ///
    /// Panics if `key` is less than 64 bytes in length.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cookie::Key;
    ///
    /// # /*
    /// let key = { /* a cryptographically random key >= 64 bytes */ };
    /// # */
    /// # let key: &Vec<u8> = &(0..64).collect();
    ///
    /// let key = Key::from(key);
    /// ```
    pub fn from(key: &[u8]) -> Key {
        if key.len() < 64 {
            panic!("bad key length: expected >= 64 bytes, found {}", key.len());
        }

        let mut output = Key::zero();
        output.signing.copy_from_slice(&key[..SIGNED_KEY_LEN]);
        output.encryption.copy_from_slice(&key[SIGNED_KEY_LEN..COMBINED_KEY_LENGTH]);
        output
    }

    /// Derives new signing/encryption keys from a master key.
    ///
    /// The master key must be at least 256-bits (32 bytes). For security, the
    /// master key _must_ be cryptographically random. The keys are derived
    /// deterministically from the master key.
    ///
    /// # Panics
    ///
    /// Panics if `key` is less than 32 bytes in length.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cookie::Key;
    ///
    /// # /*
    /// let master_key = { /* a cryptographically random key >= 32 bytes */ };
    /// # */
    /// # let master_key: &Vec<u8> = &(0..32).collect();
    ///
    /// let key = Key::derive_from(master_key);
    /// ```
    #[cfg(feature = "key-expansion")]
    #[cfg_attr(nightly, doc(cfg(feature = "key-expansion")))]
    pub fn derive_from(master_key: &[u8]) -> Self {
        if master_key.len() < 32 {
            panic!("bad master key length: expected >= 32 bytes, found {}", master_key.len());
        }

        // Expand the master key into two HKDF generated keys.
        const KEYS_INFO: &[u8] = b"COOKIE;SIGNED:HMAC-SHA256;PRIVATE:AEAD-AES-256-GCM";
        let mut both_keys = [0; SIGNED_KEY_LEN + PRIVATE_KEY_LEN];
        let hk = hkdf::Hkdf::<sha2::Sha256>::from_prk(master_key).expect("key length prechecked");
        hk.expand(KEYS_INFO, &mut both_keys).expect("expand into keys");

        // Copy the key parts into their respective fields.
        Key::from(&both_keys)
    }

    /// Derives new signing/encryption keys from a master key.
    ///
    /// The master key must be at least 256-bits (32 bytes). For security, the
    /// master key _must_ be cryptographically random. The keys are derived
    /// deterministically from the master key.
    ///
    /// # Panics
    ///
    /// Panics if `key` is less than 32 bytes in length.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cookie::Key;
    ///
    /// # /*
    /// let master_key = { /* a cryptographically random key >= 32 bytes */ };
    /// # */
    /// # let master_key: &Vec<u8> = &(0..32).collect();
    ///
    /// let key = Key::from_master(master_key);
    /// ```
    #[cfg(feature = "key-expansion")]
    #[cfg_attr(nightly, doc(cfg(feature = "key-expansion")))]
    #[deprecated(since = "0.14.0", note = "removed in favor of the more aptly named \
        `Key::derive_from()` and `Key::from()`; use one of those instead")]
    pub fn from_master(key: &[u8]) -> Self {
        Key::derive_from(key)
    }

    /// Generates signing/encryption keys from a secure, random source. Keys are
    /// generated nondeterministically.
    ///
    /// # Panics
    ///
    /// Panics if randomness cannot be retrieved from the operating system. See
    /// [`Key::try_generate()`] for a non-panicking version.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cookie::Key;
    ///
    /// let key = Key::generate();
    /// ```
    pub fn generate() -> Key {
        Self::try_generate().expect("failed to generate `Key` from randomness")
    }

    /// Attempts to generate signing/encryption keys from a secure, random
    /// source. Keys are generated nondeterministically. If randomness cannot be
    /// retrieved from the underlying operating system, returns `None`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cookie::Key;
    ///
    /// let key = Key::try_generate();
    /// ```
    pub fn try_generate() -> Option<Key> {
        use crate::secure::rand::RngCore;

        let mut rng = crate::secure::rand::thread_rng();
        let mut both_keys = [0; SIGNED_KEY_LEN + PRIVATE_KEY_LEN];
        rng.try_fill_bytes(&mut both_keys).ok()?;
        Some(Key::from(&both_keys))
    }

    /// Returns the raw bytes of a key suitable for signing cookies.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cookie::Key;
    ///
    /// let key = Key::generate();
    /// let signing_key = key.signing();
    /// ```
    pub fn signing(&self) -> &[u8] {
        &self.signing[..]
    }

    /// Returns the raw bytes of a key suitable for encrypting cookies.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cookie::Key;
    ///
    /// let key = Key::generate();
    /// let encryption_key = key.encryption();
    /// ```
    pub fn encryption(&self) -> &[u8] {
        &self.encryption[..]
    }
}

#[cfg(test)]
mod test {
    use super::Key;

    #[test]
    fn from_works() {
        let key = Key::from(&(0..64).collect::<Vec<_>>());

        let signing: Vec<u8> = (0..32).collect();
        assert_eq!(key.signing(), &*signing);

        let encryption: Vec<u8> = (32..64).collect();
        assert_eq!(key.encryption(), &*encryption);
    }

    #[test]
    #[cfg(feature = "key-expansion")]
    fn deterministic_derive() {
        let master_key: Vec<u8> = (0..32).collect();

        let key_a = Key::derive_from(&master_key);
        let key_b = Key::derive_from(&master_key);

        assert_eq!(key_a.signing(), key_b.signing());
        assert_eq!(key_a.encryption(), key_b.encryption());
        assert_ne!(key_a.encryption(), key_a.signing());

        let master_key_2: Vec<u8> = (32..64).collect();
        let key_2 = Key::derive_from(&master_key_2);

        assert_ne!(key_2.signing(), key_a.signing());
        assert_ne!(key_2.encryption(), key_a.encryption());
    }

    #[test]
    fn non_deterministic_generate() {
        let key_a = Key::generate();
        let key_b = Key::generate();

        assert_ne!(key_a.signing(), key_b.signing());
        assert_ne!(key_a.encryption(), key_b.encryption());
    }
}
