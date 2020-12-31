//! AHash is a hashing algorithm is intended to be a high performance, (hardware specific), keyed hash function.
//! This can be seen as a DOS resistant alternative to `FxHash`, or a fast equivalent to `SipHash`.
//! It provides a high speed hash algorithm, but where the result is not predictable without knowing a Key.
//! This allows it to be used in a `HashMap` without allowing for the possibility that an malicious user can
//! induce a collision.
//!
//! # How aHash works
//!
//! aHash uses the hardware AES instruction on x86 processors to provide a keyed hash function.
//! aHash is not a cryptographically secure hash.
//!
//! # Example
//! ```
//! use ahash::{AHasher, RandomState};
//! use std::collections::HashMap;
//!
//! let mut map: HashMap<i32, i32, RandomState> = HashMap::default();
//! map.insert(12, 34);
//! ```
//! For convinence wrappers called `AHashMap` and `AHashSet` are also provided.
//! These to the same thing with slightly less typing.
//! ```ignore
//! use ahash::AHashMap;
//!
//! let mut map: AHashMap<i32, i32> = AHashMap::with_capacity(4);
//! map.insert(12, 34);
//! map.insert(56, 78);
//! ```
#![deny(clippy::correctness, clippy::complexity, clippy::perf)]
#![allow(clippy::pedantic, clippy::cast_lossless, clippy::unreadable_literal)]
#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
#![cfg_attr(feature = "specialize", feature(min_specialization))]

#[macro_use]
mod convert;

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "aes", not(miri)))]
mod aes_hash;
mod fallback_hash;
#[cfg(test)]
mod hash_quality_test;

mod operations;
#[cfg(feature = "std")]
mod hash_map;
#[cfg(feature = "std")]
mod hash_set;
mod random_state;
mod specialize;

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "aes", not(miri)))]
pub use crate::aes_hash::AHasher;

#[cfg(not(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "aes", not(miri))))]
pub use crate::fallback_hash::AHasher;
pub use crate::random_state::RandomState;

pub use crate::specialize::CallHasher;

#[cfg(feature = "std")]
pub use crate::hash_map::AHashMap;
#[cfg(feature = "std")]
pub use crate::hash_set::AHashSet;
use core::hash::Hasher;
use core::hash::BuildHasher;

/// Provides a default [Hasher] with fixed keys.
/// This is typically used in conjunction with [BuildHasherDefault] to create
/// [AHasher]s in order to hash the keys of the map.
///
/// Generally it is preferable to use [RandomState] instead, so that different
/// hashmaps will have different keys. However if fixed keys are desireable this
/// may be used instead.
///
/// # Example
/// ```
/// use std::hash::BuildHasherDefault;
/// use ahash::{AHasher, RandomState};
/// use std::collections::HashMap;
///
/// let mut map: HashMap<i32, i32, BuildHasherDefault<AHasher>> = HashMap::default();
/// map.insert(12, 34);
/// ```
///
/// [BuildHasherDefault]: std::hash::BuildHasherDefault
/// [Hasher]: std::hash::Hasher
/// [HashMap]: std::collections::HashMap
impl Default for AHasher {

    /// Constructs a new [AHasher] with fixed keys.
    /// If `std` is enabled these will be generated upon first invocation.
    /// Otherwise if the `compile-time-rng`feature is enabled these will be generated at compile time.
    /// If neither of these features are available, hardcoded constants will be used.
    /// 
    /// Because the values are fixed, different hashers will all hash elements the same way.
    /// This could make hash values predictable, if DOS attacks are a concern. If this behaviour is
    /// not required, it may be preferable to use [RandomState] instead.
    /// 
    /// # Examples
    ///
    /// ```
    /// use ahash::AHasher;
    /// use std::hash::Hasher;
    ///
    /// let mut hasher_1 = AHasher::default();
    /// let mut hasher_2 = AHasher::default();
    ///
    /// hasher_1.write_u32(1234);
    /// hasher_2.write_u32(1234);
    ///
    /// assert_eq!(hasher_1.finish(), hasher_2.finish());
    /// ```
    #[inline]
    fn default() -> AHasher {
        RandomState::with_fixed_keys().build_hasher()
    }
}

/// Used for specialization. (Sealed)
pub(crate) trait HasherExt: Hasher {
    #[doc(hidden)]
    fn hash_u64(self, value: u64) -> u64;

    #[doc(hidden)]
    fn short_finish(&self) -> u64;
}

impl<T: Hasher> HasherExt for T {
    #[inline]
    #[cfg(feature = "specialize")]
    default fn hash_u64(mut self, value: u64) -> u64 {
        use core::hash::Hash;
        value.hash(&mut self);
        self.finish()
    }
    #[inline]
    #[cfg(not(feature = "specialize"))]
    fn hash_u64(mut self, value: u64) -> u64 {
        use core::hash::Hash;
        value.hash(&mut self);
        self.finish()
    }
    #[inline]
    #[cfg(feature = "specialize")]
    default fn short_finish(&self) -> u64 {
        self.finish()
    }
    #[inline]
    #[cfg(not(feature = "specialize"))]
    fn short_finish(&self) -> u64 {
        self.finish()
    }
}

// #[inline(never)]
// #[doc(hidden)]
// pub fn hash_test(input: &[u8]) -> u64 {
//     let a = AHasher::new_with_keys(11111111111_u128, 2222222222_u128);
//     input.get_hash(a)
// }

#[cfg(feature = "std")]
#[cfg(test)]
mod test {
    use crate::convert::Convert;
    use crate::*;
    use std::collections::HashMap;

    #[test]
    fn test_default_builder() {
        use core::hash::BuildHasherDefault;

        let mut map = HashMap::<u32, u64, BuildHasherDefault<AHasher>>::default();
        map.insert(1, 3);
    }

    #[test]
    fn test_builder() {
        let mut map = HashMap::<u32, u64, RandomState>::default();
        map.insert(1, 3);
    }

    #[test]
    fn test_conversion() {
        let input: &[u8] = b"dddddddd";
        let bytes: u64 = as_array!(input, 8).convert();
        assert_eq!(bytes, 0x6464646464646464);
    }

    #[test]
    fn test_ahasher_construction() {
        let _ = AHasher::new_with_keys(1234, 5678);
    }
}
