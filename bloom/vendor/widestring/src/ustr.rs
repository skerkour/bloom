use crate::{UChar, WideChar};
use core::{char, slice};

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::{
    boxed::Box,
    string::{FromUtf16Error, String},
    vec::Vec,
};
#[cfg(feature = "std")]
use std::{
    boxed::Box,
    string::{FromUtf16Error, String},
    vec::Vec,
};

/// A possible error value when converting a String from a UTF-32 byte slice.
///
/// This type is the error type for the `to_string` method on `U32Str`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FromUtf32Error();

impl core::fmt::Display for FromUtf32Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "error converting from UTF-32 to UTF-8")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FromUtf32Error {
    fn description(&self) -> &str {
        "error converting from UTF-32 to UTF-8"
    }
}

/// String slice reference for `U16String`.
///
/// `UStr` is to `UString` as `str` is to `String`.
///
/// `UStr` is not aware of nul values. Strings may or may not be nul-terminated, and may
/// contain invalid and ill-formed UTF-16 or UTF-32 data. These strings are intended to be used
/// with FFI functions that directly use string length, where the strings are known to have proper
/// nul-termination already, or where strings are merely being passed through without modification.
///
/// `UCStr` should be used instead of nul-aware strings are required.
///
/// `UStr` can be converted to many other string types, including `OsString` and `String`, making
/// proper Unicode FFI safe and easy.
///
/// Please prefer using the type aliases `U16Str` or `U32Str` or `WideStr` to using this type
/// directly.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UStr<C: UChar> {
    pub(crate) inner: [C],
}

impl<C: UChar> UStr<C> {
    /// Coerces a value into a `UStr`.
    pub fn new<S: AsRef<Self> + ?Sized>(s: &S) -> &Self {
        s.as_ref()
    }

    /// Constructs a `UStr` from a pointer and a length.
    ///
    /// The `len` argument is the number of elements, **not** the number of bytes.
    ///
    /// # Safety
    ///
    /// This function is unsafe as there is no guarantee that the given pointer is valid for `len`
    /// elements.
    ///
    /// # Panics
    ///
    /// This function panics if `p` is null.
    ///
    /// # Caveat
    ///
    /// The lifetime for the returned string is inferred from its usage. To prevent accidental
    /// misuse, it's suggested to tie the lifetime to whichever source lifetime is safe in the
    /// context, such as by providing a helper function taking the lifetime of a host value for the
    /// string, or by explicit annotation.
    pub unsafe fn from_ptr<'a>(p: *const C, len: usize) -> &'a Self {
        assert!(!p.is_null());
        let slice: *const [C] = slice::from_raw_parts(p, len);
        &*(slice as *const UStr<C>)
    }

    /// Constructs a `UStr` from a slice of code points.
    ///
    /// No checks are performed on the slice.
    pub fn from_slice(slice: &[C]) -> &Self {
        let ptr: *const [C] = slice;
        unsafe { &*(ptr as *const UStr<C>) }
    }

    /// Copies the wide string to a new owned `UString`.
    #[cfg(feature = "alloc")]
    pub fn to_ustring(&self) -> crate::UString<C> {
        crate::UString::from_vec(&self.inner)
    }

    /// Converts to a slice of the wide string.
    pub fn as_slice(&self) -> &[C] {
        &self.inner
    }

    /// Returns a raw pointer to the wide string.
    ///
    /// The pointer is valid only as long as the lifetime of this reference.
    pub fn as_ptr(&self) -> *const C {
        self.inner.as_ptr()
    }

    /// Returns the length of the wide string as number of elements (**not** number of bytes).
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns whether this wide string contains no data.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Converts a `Box<UStr>` into a `UString` without copying or allocating.
    #[cfg(feature = "alloc")]
    pub fn into_ustring(self: Box<Self>) -> crate::UString<C> {
        let boxed = unsafe { Box::from_raw(Box::into_raw(self) as *mut [C]) };
        crate::UString {
            inner: boxed.into_vec(),
        }
    }
}

impl UStr<u16> {
    /// Decodes a wide string to an owned `OsString`.
    ///
    /// This makes a string copy of the `U16Str`. Since `U16Str` makes no guarantees that it is
    /// valid UTF-16, there is no guarantee that the resulting `OsString` will be valid data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use widestring::U16String;
    /// use std::ffi::OsString;
    /// let s = "MyString";
    /// // Create a wide string from the string
    /// let wstr = U16String::from_str(s);
    /// // Create an OsString from the wide string
    /// let osstr = wstr.to_os_string();
    ///
    /// assert_eq!(osstr, OsString::from(s));
    /// ```
    #[cfg(feature = "std")]
    pub fn to_os_string(&self) -> std::ffi::OsString {
        crate::platform::os_from_wide(&self.inner)
    }

    /// Copies the wide string to a `String` if it contains valid UTF-16 data.
    ///
    /// # Failures
    ///
    /// Returns an error if the string contains any invalid UTF-16 data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use widestring::U16String;
    /// let s = "MyString";
    /// // Create a wide string from the string
    /// let wstr = U16String::from_str(s);
    /// // Create a regular string from the wide string
    /// let s2 = wstr.to_string().unwrap();
    ///
    /// assert_eq!(s2, s);
    /// ```
    #[cfg(feature = "alloc")]
    pub fn to_string(&self) -> Result<String, FromUtf16Error> {
        String::from_utf16(&self.inner)
    }

    /// Copies the wide string to a `String`.
    ///
    /// Any non-Unicode sequences are replaced with *U+FFFD REPLACEMENT CHARACTER*.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use widestring::U16String;
    /// let s = "MyString";
    /// // Create a wide string from the string
    /// let wstr = U16String::from_str(s);
    /// // Create a regular string from the wide string
    /// let lossy = wstr.to_string_lossy();
    ///
    /// assert_eq!(lossy, s);
    /// ```
    #[cfg(feature = "alloc")]
    pub fn to_string_lossy(&self) -> String {
        String::from_utf16_lossy(&self.inner)
    }
}

impl UStr<u32> {
    /// Constructs a `U32Str` from a `char` pointer and a length.
    ///
    /// The `len` argument is the number of `char` elements, **not** the number of bytes.
    ///
    /// # Safety
    ///
    /// This function is unsafe as there is no guarantee that the given pointer is valid for `len`
    /// elements.
    ///
    /// # Panics
    ///
    /// This function panics if `p` is null.
    ///
    /// # Caveat
    ///
    /// The lifetime for the returned string is inferred from its usage. To prevent accidental
    /// misuse, it's suggested to tie the lifetime to whichever source lifetime is safe in the
    /// context, such as by providing a helper function taking the lifetime of a host value for the
    /// string, or by explicit annotation.
    pub unsafe fn from_char_ptr<'a>(p: *const char, len: usize) -> &'a Self {
        UStr::from_ptr(p as *const u32, len)
    }

    /// Constructs a `U32Str` from a slice of `u32` code points.
    ///
    /// No checks are performed on the slice.
    pub fn from_char_slice(slice: &[char]) -> &Self {
        let ptr: *const [char] = slice;
        unsafe { &*(ptr as *const UStr<u32>) }
    }

    /// Decodes a wide string to an owned `OsString`.
    ///
    /// This makes a string copy of the `U32Str`. Since `U32Str` makes no guarantees that it is
    /// valid UTF-32, there is no guarantee that the resulting `OsString` will be valid data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use widestring::U32String;
    /// use std::ffi::OsString;
    /// let s = "MyString";
    /// // Create a wide string from the string
    /// let wstr = U32String::from_str(s);
    /// // Create an OsString from the wide string
    /// let osstr = wstr.to_os_string();
    ///
    /// assert_eq!(osstr, OsString::from(s));
    /// ```
    #[cfg(feature = "std")]
    pub fn to_os_string(&self) -> std::ffi::OsString {
        self.to_string_lossy().into()
    }

    /// Copies the wide string to a `String` if it contains valid UTF-32 data.
    ///
    /// # Failures
    ///
    /// Returns an error if the string contains any invalid UTF-32 data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use widestring::U32String;
    /// let s = "MyString";
    /// // Create a wide string from the string
    /// let wstr = U32String::from_str(s);
    /// // Create a regular string from the wide string
    /// let s2 = wstr.to_string().unwrap();
    ///
    /// assert_eq!(s2, s);
    /// ```
    #[cfg(feature = "alloc")]
    pub fn to_string(&self) -> Result<String, FromUtf32Error> {
        let chars: Vec<Option<char>> = self.inner.iter().map(|c| char::from_u32(*c)).collect();
        if chars.iter().any(|c| c.is_none()) {
            return Err(FromUtf32Error());
        }
        let size = chars.iter().filter_map(|o| o.map(|c| c.len_utf8())).sum();
        let mut vec = Vec::with_capacity(size);
        unsafe { vec.set_len(size) };
        let mut i = 0;
        for c in chars.iter().filter_map(|&o| o) {
            c.encode_utf8(&mut vec[i..]);
            i += c.len_utf8();
        }
        Ok(unsafe { String::from_utf8_unchecked(vec) })
    }

    /// Copies the wide string to a `String`.
    ///
    /// Any non-Unicode sequences are replaced with *U+FFFD REPLACEMENT CHARACTER*.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use widestring::U32String;
    /// let s = "MyString";
    /// // Create a wide string from the string
    /// let wstr = U32String::from_str(s);
    /// // Create a regular string from the wide string
    /// let lossy = wstr.to_string_lossy();
    ///
    /// assert_eq!(lossy, s);
    /// ```
    #[cfg(feature = "alloc")]
    pub fn to_string_lossy(&self) -> String {
        let chars: Vec<char> = self
            .inner
            .iter()
            .map(|&c| char::from_u32(c).unwrap_or(char::REPLACEMENT_CHARACTER))
            .collect();
        let size = chars.iter().map(|c| c.len_utf8()).sum();
        let mut vec = Vec::with_capacity(size);
        unsafe { vec.set_len(size) };
        let mut i = 0;
        for c in chars {
            c.encode_utf8(&mut vec[i..]);
            i += c.len_utf8();
        }
        unsafe { String::from_utf8_unchecked(vec) }
    }
}

/// String slice reference for `U16String`.
///
/// `U16Str` is to `U16String` as `str` is to `String`.
///
/// `U16Str` is not aware of nul values. Strings may or may not be nul-terminated, and may
/// contain invalid and ill-formed UTF-16 data. These strings are intended to be used with
/// FFI functions that directly use string length, where the strings are known to have proper
/// nul-termination already, or where strings are merely being passed through without modification.
///
/// `WideCStr` should be used instead of nul-aware strings are required.
///
/// `U16Str` can be converted to many other string types, including `OsString` and `String`, making
/// proper Unicode FFI safe and easy.
pub type U16Str = UStr<u16>;

/// String slice reference for `U32String`.
///
/// `U32Str` is to `U32String` as `str` is to `String`.
///
/// `U32Str` is not aware of nul values. Strings may or may not be nul-terminated, and may
/// contain invalid and ill-formed UTF-32 data. These strings are intended to be used with
/// FFI functions that directly use string length, where the strings are known to have proper
/// nul-termination already, or where strings are merely being passed through without modification.
///
/// `WideCStr` should be used instead of nul-aware strings are required.
///
/// `U32Str` can be converted to many other string types, including `OsString` and `String`, making
/// proper Unicode FFI safe and easy.
pub type U32Str = UStr<u32>;

/// Alias for `U16Str` or `U32Str` depending on platform. Intended to match typical C `wchar_t` size on platform.
pub type WideStr = UStr<WideChar>;
