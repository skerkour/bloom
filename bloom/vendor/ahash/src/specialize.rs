#[cfg(feature = "specialize")]
use crate::HasherExt;
use core::hash::Hash;
use core::hash::Hasher;

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std as alloc;

#[cfg(feature = "specialize")]
use alloc::string::String;
#[cfg(feature = "specialize")]
use alloc::vec::Vec;

/// Provides a way to get an optimized hasher for a given data type.
/// Rather than using a Hasher generically which can hash any value, this provides a way to get a specialized hash
/// for a specific type. So this may be faster for primitive types. It does however consume the hasher in the process.
/// # Example
/// ```
/// use std::hash::BuildHasher;
/// use ahash::RandomState;
/// use ahash::CallHasher;
///
/// let hash_builder = RandomState::new();
/// //...
/// let value = 17;
/// let hash = u32::get_hash(&value, hash_builder.build_hasher());
/// ```
pub trait CallHasher {
    fn get_hash<H: Hasher>(value: &Self, hasher: H) -> u64;
}

#[cfg(not(feature = "specialize"))]
impl<T> CallHasher for T
where
    T: Hash + ?Sized,
{
    #[inline]
    fn get_hash<H: Hasher>(value: &T, mut hasher: H) -> u64 {
        value.hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(feature = "specialize")]
impl<T> CallHasher for T
where
    T: Hash + ?Sized,
{
    #[inline]
    default fn get_hash<H: Hasher>(value: &T, mut hasher: H) -> u64 {
        value.hash(&mut hasher);
        hasher.finish()
    }
}

macro_rules! call_hasher_impl {
    ($typ:ty) => {
        #[cfg(feature = "specialize")]
        impl CallHasher for $typ {
            #[inline]
            fn get_hash<H: Hasher>(value: &$typ, hasher: H) -> u64 {
                hasher.hash_u64(*value as u64)
            }
        }
        #[cfg(feature = "specialize")]
        impl CallHasher for &$typ {
            #[inline]
            fn get_hash<H: Hasher>(value: &&$typ, hasher: H) -> u64 {
                hasher.hash_u64(**value as u64)
            }
        }
        #[cfg(feature = "specialize")]
        impl CallHasher for &&$typ {
            #[inline]
            fn get_hash<H: Hasher>(value: &&&$typ, hasher: H) -> u64 {
                hasher.hash_u64(***value as u64)
            }
        }
    };
}
call_hasher_impl!(u8);
call_hasher_impl!(u16);
call_hasher_impl!(u32);
call_hasher_impl!(u64);
call_hasher_impl!(i8);
call_hasher_impl!(i16);
call_hasher_impl!(i32);
call_hasher_impl!(i64);

#[cfg(feature = "specialize")]
impl CallHasher for u128 {
    #[inline]
    fn get_hash<H: Hasher>(value: &u128, mut hasher: H) -> u64 {
        hasher.write_u128(*value);
        hasher.short_finish()
    }
}

#[cfg(feature = "specialize")]
impl CallHasher for &u128 {
    #[inline]
    fn get_hash<H: Hasher>(value: &&u128, mut hasher: H) -> u64 {
        hasher.write_u128(**value);
        hasher.short_finish()
    }
}

#[cfg(feature = "specialize")]
impl CallHasher for &&u128 {
    #[inline]
    fn get_hash<H: Hasher>(value: &&&u128, mut hasher: H) -> u64 {
        hasher.write_u128(***value);
        hasher.short_finish()
    }
}

#[cfg(feature = "specialize")]
impl CallHasher for i128 {
    #[inline]
    fn get_hash<H: Hasher>(value: &i128,  mut hasher: H) -> u64 {
        hasher.write_u128(*value as u128);
        hasher.short_finish()
    }
}

#[cfg(feature = "specialize")]
impl CallHasher for &i128 {
    #[inline]
    fn get_hash<H: Hasher>(value: &&i128,  mut hasher: H) -> u64 {
        hasher.write_u128(**value as u128);
        hasher.short_finish()
    }
}

#[cfg(feature = "specialize")]
impl CallHasher for &&i128 {
    #[inline]
    fn get_hash<H: Hasher>(value: &&&i128,  mut hasher: H) -> u64 {
        hasher.write_u128(***value as u128);
        hasher.short_finish()
    }
}

#[cfg(feature = "specialize")]
impl CallHasher for [u8] {
    #[inline]
    fn get_hash<H: Hasher>(value: &Self, mut hasher: H) -> u64 {
        hasher.write(value);
        hasher.finish()
    }
}

#[cfg(feature = "specialize")]
impl CallHasher for &[u8] {
    #[inline]
    fn get_hash<H: Hasher>(value: &Self, mut hasher: H) -> u64 {
        hasher.write(*value);
        hasher.finish()
    }
}

#[cfg(feature = "specialize")]
impl CallHasher for &&[u8] {
    #[inline]
    fn get_hash<H: Hasher>(value: &Self, mut hasher: H) -> u64 {
        hasher.write(**value);
        hasher.finish()
    }
}

#[cfg(feature = "specialize")]
impl CallHasher for Vec<u8> {
    #[inline]
    fn get_hash<H: Hasher>(value: &Self, mut hasher: H) -> u64 {
        hasher.write(value);
        hasher.finish()
    }
}

#[cfg(feature = "specialize")]
impl CallHasher for &Vec<u8> {
    #[inline]
    fn get_hash<H: Hasher>(value: &Self, mut hasher: H) -> u64 {
        hasher.write(*value);
        hasher.finish()
    }
}

#[cfg(feature = "specialize")]
impl CallHasher for &&Vec<u8> {
    #[inline]
    fn get_hash<H: Hasher>(value: &Self, mut hasher: H) -> u64 {
        hasher.write(**value);
        hasher.finish()
    }
}

#[cfg(feature = "specialize")]
impl CallHasher for str {
    #[inline]
    fn get_hash<H: Hasher>(value: &Self, mut hasher: H) -> u64 {
        hasher.write(value.as_bytes());
        hasher.finish()
    }
}

#[cfg(feature = "specialize")]
impl CallHasher for &str {
    #[inline]
    fn get_hash<H: Hasher>(value: &Self, mut hasher: H) -> u64 {
        hasher.write(value.as_bytes());
        hasher.finish()
    }
}

#[cfg(feature = "specialize")]
impl CallHasher for &&str {
    #[inline]
    fn get_hash<H: Hasher>(value: &Self, mut hasher: H) -> u64 {
        hasher.write(value.as_bytes());
        hasher.finish()
    }
}

#[cfg(feature = "specialize")]
impl CallHasher for &&&str {
    #[inline]
    fn get_hash<H: Hasher>(value: &Self, mut hasher: H) -> u64 {
        hasher.write(value.as_bytes());
        hasher.finish()
    }
}

#[cfg(all(feature = "specialize"))]
impl CallHasher for String {
    #[inline]
    fn get_hash<H: Hasher>(value: &Self, mut hasher: H) -> u64 {
        hasher.write(value.as_bytes());
        hasher.finish()
    }
}

#[cfg(all(feature = "specialize"))]
impl CallHasher for &String {
    #[inline]
    fn get_hash<H: Hasher>(value: &Self, mut hasher: H) -> u64 {
        hasher.write(value.as_bytes());
        hasher.finish()
    }
}

#[cfg(all(feature = "specialize"))]
impl CallHasher for &&String {
    #[inline]
    fn get_hash<H: Hasher>(value: &Self, mut hasher: H) -> u64 {
        hasher.write(value.as_bytes());
        hasher.finish()
    }
}

#[cfg(all(feature = "specialize"))]
impl CallHasher for &&&String {
    #[inline]
    fn get_hash<H: Hasher>(value: &Self, mut hasher: H) -> u64 {
        hasher.write(value.as_bytes());
        hasher.finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    #[test]
    #[cfg(feature = "specialize")]
    pub fn test_specialized_invoked() {
        let shortened = u64::get_hash(&0, AHasher::new_with_keys(1, 2));
        let mut hasher = AHasher::new_with_keys(1, 2);
        0_u64.hash(&mut hasher);
        assert_ne!(hasher.finish(), shortened);
    }

    /// Tests that some non-trivial transformation takes place.
    #[test]
    pub fn test_input_processed() {
        let hasher = || AHasher::new_with_keys(3, 2);
        assert_ne!(0, u64::get_hash(&0, hasher()));
        assert_ne!(1, u64::get_hash(&0, hasher()));
        assert_ne!(2, u64::get_hash(&0, hasher()));
        assert_ne!(3, u64::get_hash(&0, hasher()));
        assert_ne!(4, u64::get_hash(&0, hasher()));
        assert_ne!(5, u64::get_hash(&0, hasher()));

        assert_ne!(0, u64::get_hash(&1, hasher()));
        assert_ne!(1, u64::get_hash(&1, hasher()));
        assert_ne!(2, u64::get_hash(&1, hasher()));
        assert_ne!(3, u64::get_hash(&1, hasher()));
        assert_ne!(4, u64::get_hash(&1, hasher()));
        assert_ne!(5, u64::get_hash(&1, hasher()));

        let xored = u64::get_hash(&0, hasher()) ^ u64::get_hash(&1, hasher());
        assert_ne!(0, xored);
        assert_ne!(1, xored);
        assert_ne!(2, xored);
        assert_ne!(3, xored);
        assert_ne!(4, xored);
        assert_ne!(5, xored);
    }

    #[test]
    pub fn test_ref_independent() {
        let hasher = || AHasher::new_with_keys(3, 2);
        assert_eq!(<&u8>::get_hash(&&1, hasher()), u8::get_hash(&1, hasher()));
        assert_eq!(<&u16>::get_hash(&&2, hasher()), u16::get_hash(&2, hasher()));
        assert_eq!(<&u32>::get_hash(&&3, hasher()), u32::get_hash(&3, hasher()));
        assert_eq!(<&u64>::get_hash(&&4, hasher()), u64::get_hash(&4, hasher()));
        assert_eq!(<&u128>::get_hash(&&5, hasher()), u128::get_hash(&5, hasher()));
        assert_eq!(<&str>::get_hash(&"test", hasher()), str::get_hash("test", hasher()));
        assert_eq!(<&str>::get_hash(&"test", hasher()), String::get_hash(&"test".to_string(), hasher()));
        #[cfg(feature = "specialize")]
        assert_eq!(<&str>::get_hash(&"test", hasher()), <[u8]>::get_hash("test".as_bytes(), hasher()));

        let hasher = || AHasher::new_with_keys(3, 2);
        assert_eq!(<&&u8>::get_hash(&&&1, hasher()), u8::get_hash(&1, hasher()));
        assert_eq!(<&&u16>::get_hash(&&&2, hasher()), u16::get_hash(&2, hasher()));
        assert_eq!(<&&u32>::get_hash(&&&3, hasher()), u32::get_hash(&3, hasher()));
        assert_eq!(<&&u64>::get_hash(&&&4, hasher()), u64::get_hash(&4, hasher()));
        assert_eq!(<&&u128>::get_hash(&&&5, hasher()), u128::get_hash(&5, hasher()));
        assert_eq!(<&&str>::get_hash(&&"test", hasher()), str::get_hash("test",hasher()));
        assert_eq!(<&&str>::get_hash(&&"test", hasher()), String::get_hash(&"test".to_string(), hasher()));
        #[cfg(feature = "specialize")]
        assert_eq!(<&&str>::get_hash(&&"test", hasher()), <[u8]>::get_hash(&"test".to_string().into_bytes(), hasher()));
    }
}
