use crate::{UChar, WideChar};
use core::slice;

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::{
    borrow::ToOwned,
    boxed::Box,
    string::{FromUtf16Error, String},
    vec::Vec,
};
#[cfg(feature = "std")]
use std::{
    borrow::ToOwned,
    boxed::Box,
    string::{FromUtf16Error, String},
    vec::Vec,
};

/// An error returned from `UCString` and `UCStr` to indicate that a terminating nul value
/// was missing.
///
/// The error optionally returns the ownership of the invalid vector whenever a vector was owned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissingNulError<C> {
    #[cfg(feature = "alloc")]
    pub(crate) inner: Option<Vec<C>>,
    #[cfg(not(feature = "alloc"))]
    _p: core::marker::PhantomData<C>,
}

impl<C: UChar> MissingNulError<C> {
    #[cfg(feature = "alloc")]
    fn empty() -> Self {
        Self { inner: None }
    }

    #[cfg(not(feature = "alloc"))]
    fn empty() -> Self {
        Self {
            _p: core::marker::PhantomData,
        }
    }

    /// Consumes this error, returning the underlying vector of `u16` values which generated the
    /// error in the first place.
    #[cfg(feature = "alloc")]
    pub fn into_vec(self) -> Option<Vec<C>> {
        self.inner
    }
}

impl<C: UChar> core::fmt::Display for MissingNulError<C> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "missing terminating nul value")
    }
}

#[cfg(feature = "std")]
impl<C: UChar> std::error::Error for MissingNulError<C> {
    fn description(&self) -> &str {
        "missing terminating nul value"
    }
}

/// C-style wide string reference for `UCString`.
///
/// `UCStr` is aware of nul values. Unless unchecked conversions are used, all `UCStr`
/// strings end with a nul-terminator in the underlying buffer and contain no internal nul values.
/// The strings may still contain invalid or ill-formed UTF-16 or UTF-32 data. These strings are
/// intended to be used with FFI functions such as Windows API that may require nul-terminated
/// strings.
///
/// `UCStr` can be converted to and from many other string types, including `UString`,
/// `OsString`, and `String`, making proper Unicode FFI safe and easy.
///
/// Please prefer using the type aliases `U16CStr` or `U32CStr` or `WideCStr` to using
/// this type directly.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UCStr<C: UChar> {
    inner: [C],
}

impl<C: UChar> UCStr<C> {
    /// Coerces a value into a `UCStr`.
    pub fn new<S: AsRef<UCStr<C>> + ?Sized>(s: &S) -> &Self {
        s.as_ref()
    }

    /// Constructs a `UStr` from a nul-terminated string pointer.
    ///
    /// This will scan for nul values beginning with `p`. The first nul value will be used as the
    /// nul terminator for the string, similar to how libc string functions such as `strlen` work.
    ///
    /// # Safety
    ///
    /// This function is unsafe as there is no guarantee that the given pointer is valid or has a
    /// nul terminator, and the function could scan past the underlying buffer.
    ///
    /// `p` must be non-null.
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
    pub unsafe fn from_ptr_str<'a>(p: *const C) -> &'a Self {
        assert!(!p.is_null());
        let mut i: isize = 0;
        while *p.offset(i) != UChar::NUL {
            i += 1;
        }
        let ptr: *const [C] = slice::from_raw_parts(p, i as usize + 1);
        &*(ptr as *const UCStr<C>)
    }

    /// Constructs a `UStr` from a pointer and a length.
    ///
    /// The `len` argument is the number of elements, **not** the number of bytes, and does
    /// **not** include the nul terminator of the string. Thus, a `len` of 0 is valid and means that
    /// `p` is a pointer directly to the nul terminator of the string.
    ///
    /// # Safety
    ///
    /// This function is unsafe as there is no guarantee that the given pointer is valid for `len`
    /// elements.
    ///
    /// `p` must be non-null, even for zero `len`.
    ///
    /// The interior values of the pointer are not scanned for nul. Any interior nul values will
    /// result in an invalid `UCStr`.
    ///
    /// # Panics
    ///
    /// This function panics if `p` is null or if a nul value is not found at offset `len` of `p`.
    /// Only pointers with a nul terminator are valid.
    ///
    /// # Caveat
    ///
    /// The lifetime for the returned string is inferred from its usage. To prevent accidental
    /// misuse, it's suggested to tie the lifetime to whichever source lifetime is safe in the
    /// context, such as by providing a helper function taking the lifetime of a host value for the
    /// string, or by explicit annotation.
    pub unsafe fn from_ptr_with_nul<'a>(p: *const C, len: usize) -> &'a Self {
        assert!(*p.add(len) == UChar::NUL);
        let ptr: *const [C] = slice::from_raw_parts(p, len + 1);
        &*(ptr as *const UCStr<C>)
    }

    /// Constructs a `UCStr` from a slice of values that has a nul terminator.
    ///
    /// The slice will be scanned for nul values. When a nul value is found, it is treated as the
    /// terminator for the string, and the `UCStr` slice will be truncated to that nul.
    ///
    /// # Failure
    ///
    /// If there are no no nul values in the slice, an error is returned.
    pub fn from_slice_with_nul(slice: &[C]) -> Result<&Self, MissingNulError<C>> {
        match slice.iter().position(|x| *x == UChar::NUL) {
            None => Err(MissingNulError::empty()),
            Some(i) => Ok(unsafe { UCStr::from_slice_with_nul_unchecked(&slice[..i + 1]) }),
        }
    }

    /// Constructs a `UCStr` from a slice of values that has a nul terminator. No
    /// checking for nul values is performed.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it can lead to invalid `UCStr` values when the slice
    /// is missing a terminating nul value or there are non-terminating interior nul values
    /// in the slice.
    pub unsafe fn from_slice_with_nul_unchecked(slice: &[C]) -> &Self {
        let ptr: *const [C] = slice;
        &*(ptr as *const UCStr<C>)
    }

    /// Copies the wide string to an new owned `UString`.
    #[cfg(feature = "alloc")]
    pub fn to_ucstring(&self) -> crate::UCString<C> {
        unsafe { crate::UCString::from_vec_with_nul_unchecked(self.inner.to_owned()) }
    }

    /// Copies the wide string to a new owned `UString`.
    ///
    /// The `UString` will **not** have a nul terminator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use widestring::U16CString;
    /// let wcstr = U16CString::from_str("MyString").unwrap();
    /// // Convert U16CString to a U16String
    /// let wstr = wcstr.to_ustring();
    ///
    /// // U16CString will have a terminating nul
    /// let wcvec = wcstr.into_vec_with_nul();
    /// assert_eq!(wcvec[wcvec.len()-1], 0);
    /// // The resulting U16String will not have the terminating nul
    /// let wvec = wstr.into_vec();
    /// assert_ne!(wvec[wvec.len()-1], 0);
    /// ```
    ///
    /// ```rust
    /// use widestring::U32CString;
    /// let wcstr = U32CString::from_str("MyString").unwrap();
    /// // Convert U32CString to a U32String
    /// let wstr = wcstr.to_ustring();
    ///
    /// // U32CString will have a terminating nul
    /// let wcvec = wcstr.into_vec_with_nul();
    /// assert_eq!(wcvec[wcvec.len()-1], 0);
    /// // The resulting U32String will not have the terminating nul
    /// let wvec = wstr.into_vec();
    /// assert_ne!(wvec[wvec.len()-1], 0);
    /// ```
    #[cfg(feature = "alloc")]
    pub fn to_ustring(&self) -> crate::UString<C> {
        crate::UString::from_vec(self.as_slice())
    }

    /// Converts to a slice of the wide string.
    ///
    /// The slice will **not** include the nul terminator.
    pub fn as_slice(&self) -> &[C] {
        &self.inner[..self.len()]
    }

    /// Converts to a slice of the wide string, including the nul terminator.
    pub fn as_slice_with_nul(&self) -> &[C] {
        &self.inner
    }

    /// Returns a raw pointer to the wide string.
    ///
    /// The pointer is valid only as long as the lifetime of this reference.
    pub fn as_ptr(&self) -> *const C {
        self.inner.as_ptr()
    }

    /// Returns the length of the wide string as number of elements (**not** number of bytes)
    /// **not** including nul terminator.
    pub fn len(&self) -> usize {
        self.inner.len() - 1
    }

    /// Returns whether this wide string contains no data (i.e. is only the nul terminator).
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Converts a `Box<UCStr>` into a `UCString` without copying or allocating.
    ///
    /// # Examples
    ///
    /// ```
    /// use widestring::U16CString;
    ///
    /// let v = vec![102u16, 111u16, 111u16]; // "foo"
    /// let c_string = U16CString::new(v.clone()).unwrap();
    /// let boxed = c_string.into_boxed_ucstr();
    /// assert_eq!(boxed.into_ucstring(), U16CString::new(v).unwrap());
    /// ```
    ///
    /// ```
    /// use widestring::U32CString;
    ///
    /// let v = vec![102u32, 111u32, 111u32]; // "foo"
    /// let c_string = U32CString::new(v.clone()).unwrap();
    /// let boxed = c_string.into_boxed_ucstr();
    /// assert_eq!(boxed.into_ucstring(), U32CString::new(v).unwrap());
    /// ```
    #[cfg(feature = "alloc")]
    pub fn into_ucstring(self: Box<Self>) -> crate::UCString<C> {
        let raw = Box::into_raw(self) as *mut [C];
        crate::UCString {
            inner: unsafe { Box::from_raw(raw) },
        }
    }

    #[cfg(feature = "alloc")]
    pub(crate) fn from_inner(slice: &[C]) -> &UCStr<C> {
        let ptr: *const [C] = slice;
        unsafe { &*(ptr as *const UCStr<C>) }
    }
}

impl UCStr<u16> {
    /// Decodes a wide string to an owned `OsString`.
    ///
    /// This makes a string copy of the `U16CStr`. Since `U16CStr` makes no guarantees that it is
    /// valid UTF-16, there is no guarantee that the resulting `OsString` will be valid data. The
    /// `OsString` will **not** have a nul terminator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use widestring::U16CString;
    /// use std::ffi::OsString;
    /// let s = "MyString";
    /// // Create a wide string from the string
    /// let wstr = U16CString::from_str(s).unwrap();
    /// // Create an OsString from the wide string
    /// let osstr = wstr.to_os_string();
    ///
    /// assert_eq!(osstr, OsString::from(s));
    /// ```
    #[cfg(feature = "std")]
    pub fn to_os_string(&self) -> std::ffi::OsString {
        crate::platform::os_from_wide(self.as_slice())
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
    /// use widestring::U16CString;
    /// let s = "MyString";
    /// // Create a wide string from the string
    /// let wstr = U16CString::from_str(s).unwrap();
    /// // Create a regular string from the wide string
    /// let s2 = wstr.to_string().unwrap();
    ///
    /// assert_eq!(s2, s);
    /// ```
    #[cfg(feature = "alloc")]
    pub fn to_string(&self) -> Result<String, FromUtf16Error> {
        String::from_utf16(self.as_slice())
    }

    /// Copies the wide string to a `String`.
    ///
    /// Any non-Unicode sequences are replaced with U+FFFD REPLACEMENT CHARACTER.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use widestring::U16CString;
    /// let s = "MyString";
    /// // Create a wide string from the string
    /// let wstr = U16CString::from_str(s).unwrap();
    /// // Create a regular string from the wide string
    /// let s2 = wstr.to_string_lossy();
    ///
    /// assert_eq!(s2, s);
    /// ```
    #[cfg(feature = "alloc")]
    pub fn to_string_lossy(&self) -> String {
        String::from_utf16_lossy(self.as_slice())
    }
}

impl UCStr<u32> {
    /// Constructs a `U32Str` from a `char` nul-terminated string pointer.
    ///
    /// This will scan for nul values beginning with `p`. The first nul value will be used as the
    /// nul terminator for the string, similar to how libc string functions such as `strlen` work.
    ///
    /// # Safety
    ///
    /// This function is unsafe as there is no guarantee that the given pointer is valid or has a
    /// nul terminator, and the function could scan past the underlying buffer.
    ///
    /// `p` must be non-null.
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
    pub unsafe fn from_char_ptr_str<'a>(p: *const char) -> &'a Self {
        UCStr::from_ptr_str(p as *const u32)
    }

    /// Constructs a `U32Str` from a `char` pointer and a length.
    ///
    /// The `len` argument is the number of `char` elements, **not** the number of bytes, and does
    /// **not** include the nul terminator of the string. Thus, a `len` of 0 is valid and means that
    /// `p` is a pointer directly to the nul terminator of the string.
    ///
    /// # Safety
    ///
    /// This function is unsafe as there is no guarantee that the given pointer is valid for `len`
    /// elements.
    ///
    /// `p` must be non-null, even for zero `len`.
    ///
    /// The interior values of the pointer are not scanned for nul. Any interior nul values will
    /// result in an invalid `U32CStr`.
    ///
    /// # Panics
    ///
    /// This function panics if `p` is null or if a nul value is not found at offset `len` of `p`.
    /// Only pointers with a nul terminator are valid.
    ///
    /// # Caveat
    ///
    /// The lifetime for the returned string is inferred from its usage. To prevent accidental
    /// misuse, it's suggested to tie the lifetime to whichever source lifetime is safe in the
    /// context, such as by providing a helper function taking the lifetime of a host value for the
    /// string, or by explicit annotation.
    pub unsafe fn from_char_ptr_with_nul<'a>(p: *const char, len: usize) -> &'a Self {
        UCStr::from_ptr_with_nul(p as *const u32, len)
    }

    /// Constructs a `U32CStr` from a slice of `char` values that has a nul terminator.
    ///
    /// The slice will be scanned for nul values. When a nul value is found, it is treated as the
    /// terminator for the string, and the `U32CStr` slice will be truncated to that nul.
    ///
    /// # Failure
    ///
    /// If there are no no nul values in `slice`, an error is returned.
    pub fn from_char_slice_with_nul(slice: &[char]) -> Result<&Self, MissingNulError<u32>> {
        let ptr: *const [char] = slice;
        UCStr::from_slice_with_nul(unsafe { &*(ptr as *const [u32]) })
    }

    /// Constructs a `U32CStr` from a slice of `char` values that has a nul terminator. No
    /// checking for nul values is performed.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it can lead to invalid `U32CStr` values when `slice`
    /// is missing a terminating nul value or there are non-terminating interior nul values
    /// in the slice.
    pub unsafe fn from_char_slice_with_nul_unchecked(slice: &[char]) -> &Self {
        let ptr: *const [char] = slice;
        UCStr::from_slice_with_nul_unchecked(&*(ptr as *const [u32]))
    }

    /// Decodes a wide string to an owned `OsString`.
    ///
    /// This makes a string copy of the `U32CStr`. Since `U32CStr` makes no guarantees that it is
    /// valid UTF-32, there is no guarantee that the resulting `OsString` will be valid data. The
    /// `OsString` will **not** have a nul terminator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use widestring::U32CString;
    /// use std::ffi::OsString;
    /// let s = "MyString";
    /// // Create a wide string from the string
    /// let wstr = U32CString::from_str(s).unwrap();
    /// // Create an OsString from the wide string
    /// let osstr = wstr.to_os_string();
    ///
    /// assert_eq!(osstr, OsString::from(s));
    /// ```
    #[cfg(feature = "std")]
    pub fn to_os_string(&self) -> std::ffi::OsString {
        self.to_ustring().to_os_string()
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
    /// use widestring::U32CString;
    /// let s = "MyString";
    /// // Create a wide string from the string
    /// let wstr = U32CString::from_str(s).unwrap();
    /// // Create a regular string from the wide string
    /// let s2 = wstr.to_string().unwrap();
    ///
    /// assert_eq!(s2, s);
    /// ```
    #[cfg(feature = "alloc")]
    pub fn to_string(&self) -> Result<String, crate::FromUtf32Error> {
        self.to_ustring().to_string()
    }

    /// Copies the wide string to a `String`.
    ///
    /// Any non-Unicode sequences are replaced with U+FFFD REPLACEMENT CHARACTER.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use widestring::U32CString;
    /// let s = "MyString";
    /// // Create a wide string from the string
    /// let wstr = U32CString::from_str(s).unwrap();
    /// // Create a regular string from the wide string
    /// let s2 = wstr.to_string_lossy();
    ///
    /// assert_eq!(s2, s);
    /// ```
    #[cfg(feature = "alloc")]
    pub fn to_string_lossy(&self) -> String {
        self.to_ustring().to_string_lossy()
    }
}

/// C-style wide string reference for `U16CString`.
///
/// `U16CStr` is aware of nul values. Unless unchecked conversions are used, all `U16CStr`
/// strings end with a nul-terminator in the underlying buffer and contain no internal nul values.
/// The strings may still contain invalid or ill-formed UTF-16 data. These strings are intended to
/// be used with FFI functions such as Windows API that may require nul-terminated strings.
///
/// `U16CStr` can be converted to and from many other string types, including `U16String`,
/// `OsString`, and `String`, making proper Unicode FFI safe and easy.
pub type U16CStr = UCStr<u16>;

/// C-style wide string reference for `U32CString`.
///
/// `U32CStr` is aware of nul values. Unless unchecked conversions are used, all `U32CStr`
/// strings end with a nul-terminator in the underlying buffer and contain no internal nul values.
/// The strings may still contain invalid or ill-formed UTF-32 data. These strings are intended to
/// be used with FFI functions such as Windows API that may require nul-terminated strings.
///
/// `U32CStr` can be converted to and from many other string types, including `U32String`,
/// `OsString`, and `String`, making proper Unicode FFI safe and easy.
pub type U32CStr = UCStr<u32>;

/// Alias for `U16CStr` or `U32CStr` depending on platform. Intended to match typical C `wchar_t` size on platform.
pub type WideCStr = UCStr<WideChar>;
