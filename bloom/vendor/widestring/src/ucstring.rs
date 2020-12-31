use crate::{MissingNulError, UCStr, UChar, UStr, UString, WideChar};
use core::{
    borrow::Borrow,
    mem,
    mem::ManuallyDrop,
    ops::{Deref, Index, RangeFull},
    ptr, slice,
};

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    vec::Vec,
};
#[cfg(feature = "std")]
use std::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    vec::Vec,
};

/// An owned, mutable C-style "wide" string for FFI that is nul-aware and nul-terminated.
///
/// `UCString` is aware of nul values. Unless unchecked conversions are used, all `UCString`
/// strings end with a nul-terminator in the underlying buffer and contain no internal nul values.
/// The strings may still contain invalid or ill-formed UTF-16 or UTF-32 data. These strings are
/// intended to be used with FFI functions such as Windows API that may require nul-terminated
/// strings.
///
/// `UCString` can be converted to and from many other string types, including `UString`,
/// `OsString`, and `String`, making proper Unicode FFI safe and easy.
///
/// Please prefer using the type aliases `U16CString` or `U32CString` or `WideCString` to using
/// this type directly.
///
/// # Examples
///
/// The following example constructs a `U16CString` and shows how to convert a `U16CString` to a
/// regular Rust `String`.
///
/// ```rust
/// use widestring::U16CString;
/// let s = "Test";
/// // Create a wide string from the rust string
/// let wstr = U16CString::from_str(s).unwrap();
/// // Convert back to a rust string
/// let rust_str = wstr.to_string_lossy();
/// assert_eq!(rust_str, "Test");
/// ```
///
/// The same example using `U32CString`:
///
/// ```rust
/// use widestring::U32CString;
/// let s = "Test";
/// // Create a wide string from the rust string
/// let wstr = U32CString::from_str(s).unwrap();
/// // Convert back to a rust string
/// let rust_str = wstr.to_string_lossy();
/// assert_eq!(rust_str, "Test");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UCString<C: UChar> {
    pub(crate) inner: Box<[C]>,
}

/// An error returned from `UCString` to indicate that an invalid nul value was found.
///
/// The error indicates the position in the vector where the nul value was found, as well as
/// returning the ownership of the invalid vector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NulError<C: UChar>(usize, Vec<C>);

impl<C: UChar> UCString<C> {
    /// Constructs a `UCString` from a container of wide character data.
    ///
    /// This method will consume the provided data and use the underlying elements to construct a
    /// new string. The data will be scanned for invalid nul values.
    ///
    /// # Failures
    ///
    /// This function will return an error if the data contains a nul value.
    /// The returned error will contain the `Vec` as well as the position of the nul value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use widestring::U16CString;
    /// let v = vec![84u16, 104u16, 101u16]; // 'T' 'h' 'e'
    /// # let cloned = v.clone();
    /// // Create a wide string from the vector
    /// let wcstr = U16CString::new(v).unwrap();
    /// # assert_eq!(wcstr.into_vec(), cloned);
    /// ```
    ///
    /// ```rust
    /// use widestring::U32CString;
    /// let v = vec![84u32, 104u32, 101u32]; // 'T' 'h' 'e'
    /// # let cloned = v.clone();
    /// // Create a wide string from the vector
    /// let wcstr = U32CString::new(v).unwrap();
    /// # assert_eq!(wcstr.into_vec(), cloned);
    /// ```
    ///
    /// The following example demonstrates errors from nul values in a vector.
    ///
    /// ```rust
    /// use widestring::U16CString;
    /// let v = vec![84u16, 0u16, 104u16, 101u16]; // 'T' NUL 'h' 'e'
    /// // Create a wide string from the vector
    /// let res = U16CString::new(v);
    /// assert!(res.is_err());
    /// assert_eq!(res.err().unwrap().nul_position(), 1);
    /// ```
    ///
    /// ```rust
    /// use widestring::U32CString;
    /// let v = vec![84u32, 0u32, 104u32, 101u32]; // 'T' NUL 'h' 'e'
    /// // Create a wide string from the vector
    /// let res = U32CString::new(v);
    /// assert!(res.is_err());
    /// assert_eq!(res.err().unwrap().nul_position(), 1);
    /// ```
    pub fn new(v: impl Into<Vec<C>>) -> Result<Self, NulError<C>> {
        let v = v.into();
        // Check for nul vals
        match v.iter().position(|&val| val == UChar::NUL) {
            None => Ok(unsafe { UCString::from_vec_unchecked(v) }),
            Some(pos) => Err(NulError(pos, v)),
        }
    }

    /// Constructs a `UCString` from a nul-terminated container of UTF-16 or UTF-32 data.
    ///
    /// This method will consume the provided data and use the underlying elements to construct a
    /// new string. The string will be truncated at the first nul value in the string.
    ///
    /// # Failures
    ///
    /// This function will return an error if the data does not contain a nul to terminate the
    /// string. The returned error will contain the consumed `Vec`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use widestring::U16CString;
    /// let v = vec![84u16, 104u16, 101u16, 0u16]; // 'T' 'h' 'e' NUL
    /// # let cloned = v[..3].to_owned();
    /// // Create a wide string from the vector
    /// let wcstr = U16CString::from_vec_with_nul(v).unwrap();
    /// # assert_eq!(wcstr.into_vec(), cloned);
    /// ```
    ///
    /// ```rust
    /// use widestring::U32CString;
    /// let v = vec![84u32, 104u32, 101u32, 0u32]; // 'T' 'h' 'e' NUL
    /// # let cloned = v[..3].to_owned();
    /// // Create a wide string from the vector
    /// let wcstr = U32CString::from_vec_with_nul(v).unwrap();
    /// # assert_eq!(wcstr.into_vec(), cloned);
    /// ```
    ///
    /// The following example demonstrates errors from missing nul values in a vector.
    ///
    /// ```rust
    /// use widestring::U16CString;
    /// let v = vec![84u16, 104u16, 101u16]; // 'T' 'h' 'e'
    /// // Create a wide string from the vector
    /// let res = U16CString::from_vec_with_nul(v);
    /// assert!(res.is_err());
    /// ```
    ///
    /// ```rust
    /// use widestring::U32CString;
    /// let v = vec![84u32, 104u32, 101u32]; // 'T' 'h' 'e'
    /// // Create a wide string from the vector
    /// let res = U32CString::from_vec_with_nul(v);
    /// assert!(res.is_err());
    /// ```
    pub fn from_vec_with_nul(v: impl Into<Vec<C>>) -> Result<Self, MissingNulError<C>> {
        let mut v = v.into();
        // Check for nul vals
        match v.iter().position(|&val| val == UChar::NUL) {
            None => Err(MissingNulError { inner: Some(v) }),
            Some(pos) => {
                v.truncate(pos + 1);
                Ok(unsafe { UCString::from_vec_with_nul_unchecked(v) })
            }
        }
    }

    /// Creates a `UCString` from a vector without checking for interior nul values.
    ///
    /// A terminating nul value will be appended if the vector does not already have a terminating
    /// nul.
    ///
    /// # Safety
    ///
    /// This method is equivalent to `new` except that no runtime assertion is made that `v`
    /// contains no nul values. Providing a vector with nul values will result in an invalid
    /// `UCString`.
    pub unsafe fn from_vec_unchecked(v: impl Into<Vec<C>>) -> Self {
        let mut v = v.into();
        match v.last() {
            None => v.push(UChar::NUL),
            Some(&c) if c != UChar::NUL => v.push(UChar::NUL),
            Some(_) => (),
        }
        UCString::from_vec_with_nul_unchecked(v)
    }

    /// Creates a `UCString` from a vector that should have a nul terminator, without checking
    /// for any nul values.
    ///
    /// # Safety
    ///
    /// This method is equivalent to `from_vec_with_nul` except that no runtime assertion is made
    /// that `v` contains no nul values. Providing a vector with interior nul values or without a
    /// terminating nul value will result in an invalid `UCString`.
    pub unsafe fn from_vec_with_nul_unchecked(v: impl Into<Vec<C>>) -> Self {
        UCString {
            inner: v.into().into_boxed_slice(),
        }
    }

    /// Constructs a `UCString` from anything that can be converted to a `UStr`.
    ///
    /// The string will be scanned for invalid nul values.
    ///
    /// # Failures
    ///
    /// This function will return an error if the data contains a nul value.
    /// The returned error will contain a `Vec` as well as the position of the nul value.
    pub fn from_ustr(s: impl AsRef<UStr<C>>) -> Result<Self, NulError<C>> {
        UCString::new(s.as_ref().as_slice())
    }

    /// Constructs a `UCString` from anything that can be converted to a `UStr`, without
    /// scanning for invalid nul values.
    ///
    /// # Safety
    ///
    /// This method is equivalent to `from_u16_str` except that no runtime assertion is made that
    /// `s` contains no nul values. Providing a string with nul values will result in an invalid
    /// `UCString`.
    pub unsafe fn from_ustr_unchecked(s: impl AsRef<UStr<C>>) -> Self {
        UCString::from_vec_unchecked(s.as_ref().as_slice())
    }

    /// Constructs a `UCString` from anything that can be converted to a `UStr` with a nul
    /// terminator.
    ///
    /// The string will be truncated at the first nul value in the string.
    ///
    /// # Failures
    ///
    /// This function will return an error if the data does not contain a nul to terminate the
    /// string. The returned error will contain the consumed `Vec`.
    pub fn from_ustr_with_nul(s: impl AsRef<UStr<C>>) -> Result<Self, MissingNulError<C>> {
        UCString::from_vec_with_nul(s.as_ref().as_slice())
    }

    /// Constructs a `UCString` from anything that can be converted to a `UStr` with a nul
    /// terminator, without checking the string for any invalid interior nul values.
    ///
    /// # Safety
    ///
    /// This method is equivalent to `from_u16_str_with_nul` except that no runtime assertion is
    /// made that `s` contains no nul values. Providing a vector with interior nul values or
    /// without a terminating nul value will result in an invalid `UCString`.
    pub unsafe fn from_ustr_with_nul_unchecked(s: impl AsRef<UStr<C>>) -> Self {
        UCString::from_vec_with_nul_unchecked(s.as_ref().as_slice())
    }

    /// Constructs a new `UCString` copied from a nul-terminated string pointer.
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
    pub unsafe fn from_ptr_str(p: *const C) -> Self {
        assert!(!p.is_null());
        let mut i: isize = 0;
        while *p.offset(i) != UChar::NUL {
            i += 1;
        }
        let slice = slice::from_raw_parts(p, i as usize + 1);
        UCString::from_vec_with_nul_unchecked(slice)
    }

    /// Converts to a `UCStr` reference.
    pub fn as_ucstr(&self) -> &UCStr<C> {
        self
    }

    /// Converts the wide string into a `Vec` without a nul terminator, consuming the string in
    /// the process.
    ///
    /// The resulting vector will **not** contain a nul-terminator, and will contain no other nul
    /// values.
    pub fn into_vec(self) -> Vec<C> {
        let mut v = self.into_inner().into_vec();
        v.pop();
        v
    }

    /// Converts the wide string into a `Vec`, consuming the string in the process.
    ///
    /// The resulting vector will contain a nul-terminator and no interior nul values.
    pub fn into_vec_with_nul(self) -> Vec<C> {
        self.into_inner().into_vec()
    }

    /// Transfers ownership of the wide string to a C caller.
    ///
    /// # Safety
    ///
    /// The pointer must be returned to Rust and reconstituted using `from_raw` to be properly
    /// deallocated. Specifically, one should _not_ use the standard C `free` function to
    /// deallocate this string.
    ///
    /// Failure to call `from_raw` will lead to a memory leak.
    pub fn into_raw(self) -> *mut C {
        Box::into_raw(self.into_inner()) as *mut C
    }

    /// Retakes ownership of a `UCString` that was transferred to C.
    ///
    /// # Safety
    ///
    /// This should only ever be called with a pointer that was earlier obtained by calling
    /// `into_raw` on a `UCString`. Additionally, the length of the string will be recalculated
    /// from the pointer.
    pub unsafe fn from_raw(p: *mut C) -> Self {
        assert!(!p.is_null());
        let mut i: isize = 0;
        while *p.offset(i) != UChar::NUL {
            i += 1;
        }
        let slice = slice::from_raw_parts_mut(p, i as usize + 1);
        UCString {
            inner: mem::transmute(slice),
        }
    }

    /// Converts this `UCString` into a boxed `UCStr`.
    ///
    /// # Examples
    ///
    /// ```
    /// use widestring::{U16CString, U16CStr};
    ///
    /// let mut v = vec![102u16, 111u16, 111u16]; // "foo"
    /// let c_string = U16CString::new(v.clone()).unwrap();
    /// let boxed = c_string.into_boxed_ucstr();
    /// v.push(0);
    /// assert_eq!(&*boxed, U16CStr::from_slice_with_nul(&v).unwrap());
    /// ```
    ///
    /// ```
    /// use widestring::{U32CString, U32CStr};
    ///
    /// let mut v = vec![102u32, 111u32, 111u32]; // "foo"
    /// let c_string = U32CString::new(v.clone()).unwrap();
    /// let boxed = c_string.into_boxed_ucstr();
    /// v.push(0);
    /// assert_eq!(&*boxed, U32CStr::from_slice_with_nul(&v).unwrap());
    /// ```
    pub fn into_boxed_ucstr(self) -> Box<UCStr<C>> {
        unsafe { Box::from_raw(Box::into_raw(self.into_inner()) as *mut UCStr<C>) }
    }

    /// Bypass "move out of struct which implements [`Drop`] trait" restriction.
    ///
    /// [`Drop`]: ../ops/trait.Drop.html
    fn into_inner(self) -> Box<[C]> {
        unsafe {
            let result = ptr::read(&self.inner);
            mem::forget(self);
            result
        }
    }
}

impl UCString<u16> {
    /// Constructs a `U16CString` from a `str`.
    ///
    /// The string will be scanned for invalid nul values.
    ///
    /// # Failures
    ///
    /// This function will return an error if the data contains a nul value.
    /// The returned error will contain a `Vec<u16>` as well as the position of the nul value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use widestring::U16CString;
    /// let s = "MyString";
    /// // Create a wide string from the string
    /// let wcstr = U16CString::from_str(s).unwrap();
    /// # assert_eq!(wcstr.to_string_lossy(), s);
    /// ```
    ///
    /// The following example demonstrates errors from nul values in a vector.
    ///
    /// ```rust
    /// use widestring::U16CString;
    /// let s = "My\u{0}String";
    /// // Create a wide string from the string
    /// let res = U16CString::from_str(s);
    /// assert!(res.is_err());
    /// assert_eq!(res.err().unwrap().nul_position(), 2);
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: impl AsRef<str>) -> Result<Self, NulError<u16>> {
        let v: Vec<u16> = s.as_ref().encode_utf16().collect();
        UCString::new(v)
    }

    /// Constructs a `U16CString` from a `str`, without checking for interior nul values.
    ///
    /// # Safety
    ///
    /// This method is equivalent to `from_str` except that no runtime assertion is made that `s`
    /// contains no nul values. Providing a string with nul values will result in an invalid
    /// `U16CString`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use widestring::U16CString;
    /// let s = "MyString";
    /// // Create a wide string from the string
    /// let wcstr = unsafe { U16CString::from_str_unchecked(s) };
    /// # assert_eq!(wcstr.to_string_lossy(), s);
    /// ```
    pub unsafe fn from_str_unchecked(s: impl AsRef<str>) -> Self {
        let v: Vec<u16> = s.as_ref().encode_utf16().collect();
        UCString::from_vec_unchecked(v)
    }

    /// Constructs a `U16CString` from a `str` with a nul terminator.
    ///
    /// The string will be truncated at the first nul value in the string.
    ///
    /// # Failures
    ///
    /// This function will return an error if the data does not contain a nul to terminate the
    /// string. The returned error will contain the consumed `Vec<u16>`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use widestring::U16CString;
    /// let s = "My\u{0}String";
    /// // Create a wide string from the string
    /// let wcstr = U16CString::from_str_with_nul(s).unwrap();
    /// assert_eq!(wcstr.to_string_lossy(), "My");
    /// ```
    ///
    /// The following example demonstrates errors from missing nul values in a vector.
    ///
    /// ```rust
    /// use widestring::U16CString;
    /// let s = "MyString";
    /// // Create a wide string from the string
    /// let res = U16CString::from_str_with_nul(s);
    /// assert!(res.is_err());
    /// ```
    pub fn from_str_with_nul(s: impl AsRef<str>) -> Result<Self, MissingNulError<u16>> {
        let v: Vec<u16> = s.as_ref().encode_utf16().collect();
        UCString::from_vec_with_nul(v)
    }

    /// Constructs a `U16CString` from str `str` that should have a terminating nul, but without
    /// checking for any nul values.
    ///
    /// # Safety
    ///
    /// This method is equivalent to `from_str_with_nul` except that no runtime assertion is made
    /// that `s` contains no nul values. Providing a vector with interior nul values or without a
    /// terminating nul value will result in an invalid `U16CString`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use widestring::U16CString;
    /// let s = "My String\u{0}";
    /// // Create a wide string from the string
    /// let wcstr = unsafe { U16CString::from_str_with_nul_unchecked(s) };
    /// assert_eq!(wcstr.to_string_lossy(), "My String");
    /// ```
    pub unsafe fn from_str_with_nul_unchecked(s: impl AsRef<str>) -> Self {
        let v: Vec<u16> = s.as_ref().encode_utf16().collect();
        UCString::from_vec_with_nul_unchecked(v)
    }

    /// Constructs a new `U16CString` copied from a `u16` pointer and a length.
    ///
    /// The `len` argument is the number of `u16` elements, **not** the number of bytes.
    ///
    /// The string will be scanned for invalid nul values.
    ///
    /// # Failures
    ///
    /// This function will return an error if the data contains a nul value.
    /// The returned error will contain a `Vec<u16>` as well as the position of the nul value.
    ///
    /// # Safety
    ///
    /// This function is unsafe as there is no guarantee that the given pointer is valid for `len`
    /// elements.
    ///
    /// # Panics
    ///
    /// Panics if `len` is greater than 0 but `p` is a null pointer.
    pub unsafe fn from_ptr(p: *const u16, len: usize) -> Result<Self, NulError<u16>> {
        if len == 0 {
            return Ok(UCString::default());
        }
        assert!(!p.is_null());
        let slice = slice::from_raw_parts(p, len);
        UCString::new(slice)
    }

    /// Constructs a new `U16CString` copied from a `u16` pointer and a length.
    ///
    /// The `len` argument is the number of `u16` elements, **not** the number of bytes.
    ///
    /// The string will **not** be checked for invalid nul values.
    ///
    /// # Safety
    ///
    /// This function is unsafe as there is no guarantee that the given pointer is valid for `len`
    /// elements. In addition, no checking for invalid nul values is performed, so if any elements
    /// of `p` are a nul value, the resulting `U16CString` will be invalid.
    ///
    /// # Panics
    ///
    /// Panics if `len` is greater than 0 but `p` is a null pointer.
    pub unsafe fn from_ptr_unchecked(p: *const u16, len: usize) -> Self {
        if len == 0 {
            return UCString::default();
        }
        assert!(!p.is_null());
        let slice = slice::from_raw_parts(p, len);
        UCString::from_vec_unchecked(slice)
    }

    /// Constructs a new `U16String` copied from a `u16` pointer and a length.
    ///
    /// The `len` argument is the number of `u16` elements, **not** the number of bytes.
    ///
    /// The string will be truncated at the first nul value in the string.
    ///
    /// # Failures
    ///
    /// This function will return an error if the data does not contain a nul to terminate the
    /// string. The returned error will contain the consumed `Vec<u16>`.
    ///
    /// # Safety
    ///
    /// This function is unsafe as there is no guarantee that the given pointer is valid for `len`
    /// elements.
    ///
    /// # Panics
    ///
    /// Panics if `len` is greater than 0 but `p` is a null pointer.
    pub unsafe fn from_ptr_with_nul(
        p: *const u16,
        len: usize,
    ) -> Result<Self, MissingNulError<u16>> {
        if len == 0 {
            return Ok(UCString::default());
        }
        assert!(!p.is_null());
        let slice = slice::from_raw_parts(p, len);
        UCString::from_vec_with_nul(slice)
    }

    /// Constructs a new `U16String` copied from a `u16` pointer and a length.
    ///
    /// The `len` argument is the number of `u16` elements, **not** the number of bytes.
    ///
    /// The data should end with a nul terminator, but no checking is done on whether the data
    /// actually ends with a nul terminator, or if the data contains any interior nul values.
    ///
    /// # Safety
    ///
    /// This function is unsafe as there is no guarantee that the given pointer is valid for `len`
    /// elements. In addition, no checking for nul values is performed, so if there data does not
    /// end with a nul terminator, or if there are any interior nul values, the resulting
    /// `U16CString` will be invalid.
    ///
    /// # Panics
    ///
    /// Panics if `len` is greater than 0 but `p` is a null pointer.
    pub unsafe fn from_ptr_with_nul_unchecked(p: *const u16, len: usize) -> Self {
        if len == 0 {
            return UCString::default();
        }
        assert!(!p.is_null());
        let slice = slice::from_raw_parts(p, len);
        UCString::from_vec_with_nul_unchecked(slice)
    }

    /// Constructs a `U16CString` from anything that can be converted to an `OsStr`.
    ///
    /// The string will be scanned for invalid nul values.
    ///
    /// # Failures
    ///
    /// This function will return an error if the data contains a nul value.
    /// The returned error will contain a `Vec<u16>` as well as the position of the nul value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use widestring::U16CString;
    /// let s = "MyString";
    /// // Create a wide string from the string
    /// let wcstr = U16CString::from_os_str(s).unwrap();
    /// # assert_eq!(wcstr.to_string_lossy(), s);
    /// ```
    ///
    /// The following example demonstrates errors from nul values in a vector.
    ///
    /// ```rust
    /// use widestring::U16CString;
    /// let s = "My\u{0}String";
    /// // Create a wide string from the string
    /// let res = U16CString::from_os_str(s);
    /// assert!(res.is_err());
    /// assert_eq!(res.err().unwrap().nul_position(), 2);
    /// ```
    #[cfg(feature = "std")]
    pub fn from_os_str(s: impl AsRef<std::ffi::OsStr>) -> Result<Self, NulError<u16>> {
        let v = crate::platform::os_to_wide(s.as_ref());
        UCString::new(v)
    }

    /// Constructs a `U16CString` from anything that can be converted to an `OsStr`, without
    /// checking for interior nul values.
    ///
    /// # Safety
    ///
    /// This method is equivalent to `from_os_str` except that no runtime assertion is made that
    /// `s` contains no nul values. Providing a string with nul values will result in an invalid
    /// `U16CString`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use widestring::U16CString;
    /// let s = "MyString";
    /// // Create a wide string from the string
    /// let wcstr = unsafe { U16CString::from_os_str_unchecked(s) };
    /// # assert_eq!(wcstr.to_string_lossy(), s);
    /// ```
    #[cfg(feature = "std")]
    pub unsafe fn from_os_str_unchecked(s: impl AsRef<std::ffi::OsStr>) -> Self {
        let v = crate::platform::os_to_wide(s.as_ref());
        UCString::from_vec_unchecked(v)
    }

    /// Constructs a `U16CString` from anything that can be converted to an `OsStr` with a nul
    /// terminator.
    ///
    /// The string will be truncated at the first nul value in the string.
    ///
    /// # Failures
    ///
    /// This function will return an error if the data does not contain a nul to terminate the
    /// string. The returned error will contain the consumed `Vec<u16>`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use widestring::U16CString;
    /// let s = "My\u{0}String";
    /// // Create a wide string from the string
    /// let wcstr = U16CString::from_os_str_with_nul(s).unwrap();
    /// assert_eq!(wcstr.to_string_lossy(), "My");
    /// ```
    ///
    /// The following example demonstrates errors from missing nul values in a vector.
    ///
    /// ```rust
    /// use widestring::U16CString;
    /// let s = "MyString";
    /// // Create a wide string from the string
    /// let res = U16CString::from_os_str_with_nul(s);
    /// assert!(res.is_err());
    /// ```
    #[cfg(feature = "std")]
    pub fn from_os_str_with_nul(
        s: impl AsRef<std::ffi::OsStr>,
    ) -> Result<Self, MissingNulError<u16>> {
        let v = crate::platform::os_to_wide(s.as_ref());
        UCString::from_vec_with_nul(v)
    }

    /// Constructs a `U16CString` from anything that can be converted to an `OsStr` that should
    /// have a terminating nul, but without checking for any nul values.
    ///
    /// # Safety
    ///
    /// This method is equivalent to `from_os_str_with_nul` except that no runtime assertion is
    /// made that `s` contains no nul values. Providing a vector with interior nul values or
    /// without a terminating nul value will result in an invalid `U16CString`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use widestring::U16CString;
    /// let s = "My String\u{0}";
    /// // Create a wide string from the string
    /// let wcstr = unsafe { U16CString::from_os_str_with_nul_unchecked(s) };
    /// assert_eq!(wcstr.to_string_lossy(), "My String");
    /// ```
    #[cfg(feature = "std")]
    pub unsafe fn from_os_str_with_nul_unchecked(s: impl AsRef<std::ffi::OsStr>) -> Self {
        let v = crate::platform::os_to_wide(s.as_ref());
        UCString::from_vec_with_nul_unchecked(v)
    }
}

impl UCString<u32> {
    /// Constructs a `U32CString` from a container of wide character data.
    ///
    /// This method will consume the provided data and use the underlying elements to construct a
    /// new string. The data will be scanned for invalid nul values.
    ///
    /// # Failures
    ///
    /// This function will return an error if the data contains a nul value.
    /// The returned error will contain the `Vec<u32>` as well as the position of the nul value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use widestring::U32CString;
    /// let v: Vec<char> = "Test".chars().collect();
    /// # let cloned: Vec<u32> = v.iter().map(|&c| c as u32).collect();
    /// // Create a wide string from the vector
    /// let wcstr = U32CString::from_chars(v).unwrap();
    /// # assert_eq!(wcstr.into_vec(), cloned);
    /// ```
    ///
    /// The following example demonstrates errors from nul values in a vector.
    ///
    /// ```rust
    /// use widestring::U32CString;
    /// let v: Vec<char> = "T\u{0}est".chars().collect();
    /// // Create a wide string from the vector
    /// let res = U32CString::from_chars(v);
    /// assert!(res.is_err());
    /// assert_eq!(res.err().unwrap().nul_position(), 1);
    /// ```
    pub fn from_chars(v: impl Into<Vec<char>>) -> Result<Self, NulError<u32>> {
        let mut chars = v.into();
        let v: Vec<u32> = unsafe {
            let ptr = chars.as_mut_ptr() as *mut u32;
            let len = chars.len();
            let cap = chars.capacity();
            ManuallyDrop::new(chars);
            Vec::from_raw_parts(ptr, len, cap)
        };
        UCString::new(v)
    }

    /// Constructs a `U32CString` from a nul-terminated container of UTF-32 data.
    ///
    /// This method will consume the provided data and use the underlying elements to construct a
    /// new string. The string will be truncated at the first nul value in the string.
    ///
    /// # Failures
    ///
    /// This function will return an error if the data does not contain a nul to terminate the
    /// string. The returned error will contain the consumed `Vec<u32>`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use widestring::U32CString;
    /// let v: Vec<char> = "Test\u{0}".chars().collect();
    /// # let cloned: Vec<u32> = v[..4].iter().map(|&c| c as u32).collect();
    /// // Create a wide string from the vector
    /// let wcstr = U32CString::from_chars_with_nul(v).unwrap();
    /// # assert_eq!(wcstr.into_vec(), cloned);
    /// ```
    ///
    /// The following example demonstrates errors from missing nul values in a vector.
    ///
    /// ```rust
    /// use widestring::U32CString;
    /// let v: Vec<char> = "Test".chars().collect();
    /// // Create a wide string from the vector
    /// let res = U32CString::from_chars_with_nul(v);
    /// assert!(res.is_err());
    /// ```
    pub fn from_chars_with_nul(v: impl Into<Vec<char>>) -> Result<Self, MissingNulError<u32>> {
        let mut chars = v.into();
        let v: Vec<u32> = unsafe {
            let ptr = chars.as_mut_ptr() as *mut u32;
            let len = chars.len();
            let cap = chars.capacity();
            ManuallyDrop::new(chars);
            Vec::from_raw_parts(ptr, len, cap)
        };
        UCString::from_vec_with_nul(v)
    }

    /// Creates a `U32CString` from a vector without checking for interior nul values.
    ///
    /// A terminating nul value will be appended if the vector does not already have a terminating
    /// nul.
    ///
    /// # Safety
    ///
    /// This method is equivalent to `new` except that no runtime assertion is made that `v`
    /// contains no nul values. Providing a vector with nul values will result in an invalid
    /// `U32CString`.
    pub unsafe fn from_chars_unchecked(v: impl Into<Vec<char>>) -> Self {
        let mut chars = v.into();
        let v: Vec<u32> = {
            let ptr = chars.as_mut_ptr() as *mut u32;
            let len = chars.len();
            let cap = chars.capacity();
            ManuallyDrop::new(chars);
            Vec::from_raw_parts(ptr, len, cap)
        };
        UCString::from_vec_unchecked(v)
    }

    /// Creates a `U32CString` from a vector that should have a nul terminator, without checking
    /// for any nul values.
    ///
    /// # Safety
    ///
    /// This method is equivalent to `from_vec_with_nul` except that no runtime assertion is made
    /// that `v` contains no nul values. Providing a vector with interior nul values or without a
    /// terminating nul value will result in an invalid `U32CString`.
    pub unsafe fn from_chars_with_nul_unchecked(v: impl Into<Vec<char>>) -> Self {
        let mut chars = v.into();
        let v: Vec<u32> = {
            let ptr = chars.as_mut_ptr() as *mut u32;
            let len = chars.len();
            let cap = chars.capacity();
            ManuallyDrop::new(chars);
            Vec::from_raw_parts(ptr, len, cap)
        };
        UCString::from_vec_with_nul_unchecked(v)
    }

    /// Constructs a `U32CString` from a `str`.
    ///
    /// The string will be scanned for invalid nul values.
    ///
    /// # Failures
    ///
    /// This function will return an error if the data contains a nul value.
    /// The returned error will contain a `Vec<u32>` as well as the position of the nul value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use widestring::U32CString;
    /// let s = "MyString";
    /// // Create a wide string from the string
    /// let wcstr = U32CString::from_str(s).unwrap();
    /// # assert_eq!(wcstr.to_string_lossy(), s);
    /// ```
    ///
    /// The following example demonstrates errors from nul values in a vector.
    ///
    /// ```rust
    /// use widestring::U32CString;
    /// let s = "My\u{0}String";
    /// // Create a wide string from the string
    /// let res = U32CString::from_str(s);
    /// assert!(res.is_err());
    /// assert_eq!(res.err().unwrap().nul_position(), 2);
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: impl AsRef<str>) -> Result<Self, NulError<u32>> {
        let v: Vec<char> = s.as_ref().chars().collect();
        UCString::from_chars(v)
    }

    /// Constructs a `U32CString` from a `str`, without checking for interior nul values.
    ///
    /// # Safety
    ///
    /// This method is equivalent to `from_str` except that no runtime assertion is made that `s`
    /// contains no nul values. Providing a string with nul values will result in an invalid
    /// `U32CString`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use widestring::U32CString;
    /// let s = "MyString";
    /// // Create a wide string from the string
    /// let wcstr = unsafe { U32CString::from_str_unchecked(s) };
    /// # assert_eq!(wcstr.to_string_lossy(), s);
    /// ```
    pub unsafe fn from_str_unchecked(s: impl AsRef<str>) -> Self {
        let v: Vec<char> = s.as_ref().chars().collect();
        UCString::from_chars_unchecked(v)
    }

    /// Constructs a `U32CString` from a `str` with a nul terminator.
    ///
    /// The string will be truncated at the first nul value in the string.
    ///
    /// # Failures
    ///
    /// This function will return an error if the data does not contain a nul to terminate the
    /// string. The returned error will contain the consumed `Vec<u32>`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use widestring::U32CString;
    /// let s = "My\u{0}String";
    /// // Create a wide string from the string
    /// let wcstr = U32CString::from_str_with_nul(s).unwrap();
    /// assert_eq!(wcstr.to_string_lossy(), "My");
    /// ```
    ///
    /// The following example demonstrates errors from missing nul values in a vector.
    ///
    /// ```rust
    /// use widestring::U32CString;
    /// let s = "MyString";
    /// // Create a wide string from the string
    /// let res = U32CString::from_str_with_nul(s);
    /// assert!(res.is_err());
    /// ```
    pub fn from_str_with_nul(s: impl AsRef<str>) -> Result<Self, MissingNulError<u32>> {
        let v: Vec<char> = s.as_ref().chars().collect();
        UCString::from_chars_with_nul(v)
    }

    /// Constructs a `U32CString` from a `str` that should have a terminating nul, but without
    /// checking for any nul values.
    ///
    /// # Safety
    ///
    /// This method is equivalent to `from_str_with_nul` except that no runtime assertion is made
    /// that `s` contains no nul values. Providing a vector with interior nul values or without a
    /// terminating nul value will result in an invalid `U32CString`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use widestring::U32CString;
    /// let s = "My String\u{0}";
    /// // Create a wide string from the string
    /// let wcstr = unsafe { U32CString::from_str_with_nul_unchecked(s) };
    /// assert_eq!(wcstr.to_string_lossy(), "My String");
    /// ```
    pub unsafe fn from_str_with_nul_unchecked(s: impl AsRef<str>) -> Self {
        let v: Vec<char> = s.as_ref().chars().collect();
        UCString::from_chars_with_nul_unchecked(v)
    }

    /// Constructs a new `U32CString` copied from a `u32` pointer and a length.
    ///
    /// The `len` argument is the number of `u32` elements, **not** the number of bytes.
    ///
    /// The string will be scanned for invalid nul values.
    ///
    /// # Failures
    ///
    /// This function will return an error if the data contains a nul value.
    /// The returned error will contain a `Vec<u32>` as well as the position of the nul value.
    ///
    /// # Safety
    ///
    /// This function is unsafe as there is no guarantee that the given pointer is valid for `len`
    /// elements.
    ///
    /// # Panics
    ///
    /// Panics if `len` is greater than 0 but `p` is a null pointer.
    pub unsafe fn from_ptr(p: *const u32, len: usize) -> Result<Self, NulError<u32>> {
        if len == 0 {
            return Ok(UCString::default());
        }
        assert!(!p.is_null());
        let slice = slice::from_raw_parts(p, len);
        UCString::new(slice)
    }

    /// Constructs a new `U32CString` copied from a `u32` pointer and a length.
    ///
    /// The `len` argument is the number of `u32` elements, **not** the number of bytes.
    ///
    /// The string will **not** be checked for invalid nul values.
    ///
    /// # Safety
    ///
    /// This function is unsafe as there is no guarantee that the given pointer is valid for `len`
    /// elements. In addition, no checking for invalid nul values is performed, so if any elements
    /// of `p` are a nul value, the resulting `U16CString` will be invalid.
    ///
    /// # Panics
    ///
    /// Panics if `len` is greater than 0 but `p` is a null pointer.
    pub unsafe fn from_ptr_unchecked(p: *const u32, len: usize) -> Self {
        if len == 0 {
            return UCString::default();
        }
        assert!(!p.is_null());
        let slice = slice::from_raw_parts(p, len);
        UCString::from_vec_unchecked(slice)
    }

    /// Constructs a new `U32String` copied from a `u32` pointer and a length.
    ///
    /// The `len` argument is the number of `u32` elements, **not** the number of bytes.
    ///
    /// The string will be truncated at the first nul value in the string.
    ///
    /// # Failures
    ///
    /// This function will return an error if the data does not contain a nul to terminate the
    /// string. The returned error will contain the consumed `Vec<u32>`.
    ///
    /// # Safety
    ///
    /// This function is unsafe as there is no guarantee that the given pointer is valid for `len`
    /// elements.
    ///
    /// # Panics
    ///
    /// Panics if `len` is greater than 0 but `p` is a null pointer.
    pub unsafe fn from_ptr_with_nul(
        p: *const u32,
        len: usize,
    ) -> Result<Self, MissingNulError<u32>> {
        if len == 0 {
            return Ok(UCString::default());
        }
        assert!(!p.is_null());
        let slice = slice::from_raw_parts(p, len);
        UCString::from_vec_with_nul(slice)
    }

    /// Constructs a new `U32String` copied from a `u32` pointer and a length.
    ///
    /// The `len` argument is the number of `u32` elements, **not** the number of bytes.
    ///
    /// The data should end with a nul terminator, but no checking is done on whether the data
    /// actually ends with a nul terminator, or if the data contains any interior nul values.
    ///
    /// # Safety
    ///
    /// This function is unsafe as there is no guarantee that the given pointer is valid for `len`
    /// elements. In addition, no checking for nul values is performed, so if there data does not
    /// end with a nul terminator, or if there are any interior nul values, the resulting
    /// `U32CString` will be invalid.
    ///
    /// # Panics
    ///
    /// Panics if `len` is greater than 0 but `p` is a null pointer.
    pub unsafe fn from_ptr_with_nul_unchecked(p: *const u32, len: usize) -> Self {
        if len == 0 {
            return UCString::default();
        }
        assert!(!p.is_null());
        let slice = slice::from_raw_parts(p, len);
        UCString::from_vec_with_nul_unchecked(slice)
    }

    /// Constructs a new `U32CString` copied from a `char` pointer and a length.
    ///
    /// The `len` argument is the number of `char` elements, **not** the number of bytes.
    ///
    /// The string will be scanned for invalid nul values.
    ///
    /// # Failures
    ///
    /// This function will return an error if the data contains a nul value.
    /// The returned error will contain a `Vec<u32>` as well as the position of the nul value.
    ///
    /// # Safety
    ///
    /// This function is unsafe as there is no guarantee that the given pointer is valid for `len`
    /// elements.
    ///
    /// # Panics
    ///
    /// Panics if `len` is greater than 0 but `p` is a null pointer.
    pub unsafe fn from_char_ptr(p: *const char, len: usize) -> Result<Self, NulError<u32>> {
        UCString::<u32>::from_ptr(p as *const u32, len)
    }

    /// Constructs a new `U32CString` copied from a `char` pointer and a length.
    ///
    /// The `len` argument is the number of `char` elements, **not** the number of bytes.
    ///
    /// The string will **not** be checked for invalid nul values.
    ///
    /// # Safety
    ///
    /// This function is unsafe as there is no guarantee that the given pointer is valid for `len`
    /// elements. In addition, no checking for invalid nul values is performed, so if any elements
    /// of `p` are a nul value, the resulting `U32CString` will be invalid.
    ///
    /// # Panics
    ///
    /// Panics if `len` is greater than 0 but `p` is a null pointer.
    pub unsafe fn from_char_ptr_unchecked(p: *const char, len: usize) -> Self {
        UCString::<u32>::from_ptr_unchecked(p as *const u32, len)
    }

    /// Constructs a new `U32String` copied from a `char` pointer and a length.
    ///
    /// The `len` argument is the number of `char` elements, **not** the number of bytes.
    ///
    /// The string will be truncated at the first nul value in the string.
    ///
    /// # Failures
    ///
    /// This function will return an error if the data does not contain a nul to terminate the
    /// string. The returned error will contain the consumed `Vec<u32>`.
    ///
    /// # Safety
    ///
    /// This function is unsafe as there is no guarantee that the given pointer is valid for `len`
    /// elements.
    ///
    /// # Panics
    ///
    /// Panics if `len` is greater than 0 but `p` is a null pointer.
    pub unsafe fn from_char_ptr_with_nul(
        p: *const char,
        len: usize,
    ) -> Result<Self, MissingNulError<u32>> {
        UCString::<u32>::from_ptr_with_nul(p as *const u32, len)
    }

    /// Constructs a new `U32String` copied from a `char` pointer and a length.
    ///
    /// The `len` argument is the number of `char` elements, **not** the number of bytes.
    ///
    /// The data should end with a nul terminator, but no checking is done on whether the data
    /// actually ends with a nul terminator, or if the data contains any interior nul values.
    ///
    /// # Safety
    ///
    /// This function is unsafe as there is no guarantee that the given pointer is valid for `len`
    /// elements. In addition, no checking for nul values is performed, so if there data does not
    /// end with a nul terminator, or if there are any interior nul values, the resulting
    /// `U32CString` will be invalid.
    ///
    /// # Panics
    ///
    /// Panics if `len` is greater than 0 but `p` is a null pointer.
    pub unsafe fn from_char_ptr_with_nul_unchecked(p: *const char, len: usize) -> Self {
        UCString::<u32>::from_ptr_with_nul_unchecked(p as *const u32, len)
    }

    /// Constructs a `U32CString` from anything that can be converted to an `OsStr`.
    ///
    /// The string will be scanned for invalid nul values.
    ///
    /// # Failures
    ///
    /// This function will return an error if the data contains a nul value.
    /// The returned error will contain a `Vec<u16>` as well as the position of the nul value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use widestring::U32CString;
    /// let s = "MyString";
    /// // Create a wide string from the string
    /// let wcstr = U32CString::from_os_str(s).unwrap();
    /// # assert_eq!(wcstr.to_string_lossy(), s);
    /// ```
    ///
    /// The following example demonstrates errors from nul values in a vector.
    ///
    /// ```rust
    /// use widestring::U32CString;
    /// let s = "My\u{0}String";
    /// // Create a wide string from the string
    /// let res = U32CString::from_os_str(s);
    /// assert!(res.is_err());
    /// assert_eq!(res.err().unwrap().nul_position(), 2);
    /// ```
    #[cfg(feature = "std")]
    pub fn from_os_str(s: impl AsRef<std::ffi::OsStr>) -> Result<Self, NulError<u32>> {
        let v: Vec<char> = s.as_ref().to_string_lossy().chars().collect();
        UCString::from_chars(v)
    }

    /// Constructs a `U32CString` from anything that can be converted to an `OsStr`, without
    /// checking for interior nul values.
    ///
    /// # Safety
    ///
    /// This method is equivalent to `from_os_str` except that no runtime assertion is made that
    /// `s` contains no nul values. Providing a string with nul values will result in an invalid
    /// `U32CString`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use widestring::U32CString;
    /// let s = "MyString";
    /// // Create a wide string from the string
    /// let wcstr = unsafe { U32CString::from_os_str_unchecked(s) };
    /// # assert_eq!(wcstr.to_string_lossy(), s);
    /// ```
    #[cfg(feature = "std")]
    pub unsafe fn from_os_str_unchecked(s: impl AsRef<std::ffi::OsStr>) -> Self {
        let v: Vec<char> = s.as_ref().to_string_lossy().chars().collect();
        UCString::from_chars_unchecked(v)
    }

    /// Constructs a `U32CString` from anything that can be converted to an `OsStr` with a nul
    /// terminator.
    ///
    /// The string will be truncated at the first nul value in the string.
    ///
    /// # Failures
    ///
    /// This function will return an error if the data does not contain a nul to terminate the
    /// string. The returned error will contain the consumed `Vec<u16>`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use widestring::U32CString;
    /// let s = "My\u{0}String";
    /// // Create a wide string from the string
    /// let wcstr = U32CString::from_os_str_with_nul(s).unwrap();
    /// assert_eq!(wcstr.to_string_lossy(), "My");
    /// ```
    ///
    /// The following example demonstrates errors from missing nul values in a vector.
    ///
    /// ```rust
    /// use widestring::U32CString;
    /// let s = "MyString";
    /// // Create a wide string from the string
    /// let res = U32CString::from_os_str_with_nul(s);
    /// assert!(res.is_err());
    /// ```
    #[cfg(feature = "std")]
    pub fn from_os_str_with_nul(
        s: impl AsRef<std::ffi::OsStr>,
    ) -> Result<Self, MissingNulError<u32>> {
        let v: Vec<char> = s.as_ref().to_string_lossy().chars().collect();
        UCString::from_chars_with_nul(v)
    }

    /// Constructs a `U32CString` from anything that can be converted to an `OsStr` that should
    /// have a terminating nul, but without checking for any nul values.
    ///
    /// # Safety
    ///
    /// This method is equivalent to `from_os_str_with_nul` except that no runtime assertion is
    /// made that `s` contains no nul values. Providing a vector with interior nul values or
    /// without a terminating nul value will result in an invalid `U32CString`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use widestring::U32CString;
    /// let s = "My String\u{0}";
    /// // Create a wide string from the string
    /// let wcstr = unsafe { U32CString::from_os_str_with_nul_unchecked(s) };
    /// assert_eq!(wcstr.to_string_lossy(), "My String");
    /// ```
    #[cfg(feature = "std")]
    pub unsafe fn from_os_str_with_nul_unchecked(s: impl AsRef<std::ffi::OsStr>) -> Self {
        let v: Vec<char> = s.as_ref().to_string_lossy().chars().collect();
        UCString::from_chars_with_nul_unchecked(v)
    }
}

impl<C: UChar> Into<Vec<C>> for UCString<C> {
    fn into(self) -> Vec<C> {
        self.into_vec()
    }
}

impl<'a> From<UCString<u16>> for Cow<'a, UCStr<u16>> {
    fn from(s: UCString<u16>) -> Cow<'a, UCStr<u16>> {
        Cow::Owned(s)
    }
}

impl<'a> From<UCString<u32>> for Cow<'a, UCStr<u32>> {
    fn from(s: UCString<u32>) -> Cow<'a, UCStr<u32>> {
        Cow::Owned(s)
    }
}

#[cfg(feature = "std")]
impl From<UCString<u16>> for std::ffi::OsString {
    fn from(s: UCString<u16>) -> std::ffi::OsString {
        s.to_os_string()
    }
}

#[cfg(feature = "std")]
impl From<UCString<u32>> for std::ffi::OsString {
    fn from(s: UCString<u32>) -> std::ffi::OsString {
        s.to_os_string()
    }
}

impl<C: UChar> From<UCString<C>> for UString<C> {
    fn from(s: UCString<C>) -> Self {
        s.to_ustring()
    }
}

impl<'a, C: UChar, T: ?Sized + AsRef<UCStr<C>>> From<&'a T> for UCString<C> {
    fn from(s: &'a T) -> Self {
        s.as_ref().to_ucstring()
    }
}

impl<C: UChar> Index<RangeFull> for UCString<C> {
    type Output = UCStr<C>;

    #[inline]
    fn index(&self, _index: RangeFull) -> &UCStr<C> {
        UCStr::from_inner(&self.inner)
    }
}

impl<C: UChar> Deref for UCString<C> {
    type Target = UCStr<C>;

    #[inline]
    fn deref(&self) -> &UCStr<C> {
        &self[..]
    }
}

impl<'a> Default for &'a UCStr<u16> {
    fn default() -> Self {
        const SLICE: &[u16] = &[UChar::NUL];
        unsafe { UCStr::from_slice_with_nul_unchecked(SLICE) }
    }
}

impl<'a> Default for &'a UCStr<u32> {
    fn default() -> Self {
        const SLICE: &[u32] = &[UChar::NUL];
        unsafe { UCStr::from_slice_with_nul_unchecked(SLICE) }
    }
}

impl Default for UCString<u16> {
    fn default() -> Self {
        let def: &UCStr<u16> = Default::default();
        def.to_ucstring()
    }
}

impl Default for UCString<u32> {
    fn default() -> Self {
        let def: &UCStr<u32> = Default::default();
        def.to_ucstring()
    }
}

// Turns this `U16CString` into an empty string to prevent
// memory unsafe code from working by accident. Inline
// to prevent LLVM from optimizing it away in debug builds.
impl<C: UChar> Drop for UCString<C> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            *self.inner.get_unchecked_mut(0) = UChar::NUL;
        }
    }
}

impl<C: UChar> Borrow<UCStr<C>> for UCString<C> {
    fn borrow(&self) -> &UCStr<C> {
        &self[..]
    }
}

impl<C: UChar> ToOwned for UCStr<C> {
    type Owned = UCString<C>;
    fn to_owned(&self) -> UCString<C> {
        self.to_ucstring()
    }
}

impl<'a> From<&'a UCStr<u16>> for Cow<'a, UCStr<u16>> {
    fn from(s: &'a UCStr<u16>) -> Cow<'a, UCStr<u16>> {
        Cow::Borrowed(s)
    }
}

impl<'a> From<&'a UCStr<u32>> for Cow<'a, UCStr<u32>> {
    fn from(s: &'a UCStr<u32>) -> Cow<'a, UCStr<u32>> {
        Cow::Borrowed(s)
    }
}

impl<C: UChar> AsRef<UCStr<C>> for UCStr<C> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<C: UChar> AsRef<UCStr<C>> for UCString<C> {
    fn as_ref(&self) -> &UCStr<C> {
        self
    }
}

impl<C: UChar> AsRef<[C]> for UCStr<C> {
    fn as_ref(&self) -> &[C] {
        self.as_slice()
    }
}

impl<C: UChar> AsRef<[C]> for UCString<C> {
    fn as_ref(&self) -> &[C] {
        self.as_slice()
    }
}

impl<'a, C: UChar> From<&'a UCStr<C>> for Box<UCStr<C>> {
    fn from(s: &'a UCStr<C>) -> Box<UCStr<C>> {
        let boxed: Box<[C]> = Box::from(s.as_slice_with_nul());
        unsafe { Box::from_raw(Box::into_raw(boxed) as *mut UCStr<C>) }
    }
}

impl<C: UChar> From<Box<UCStr<C>>> for UCString<C> {
    #[inline]
    fn from(s: Box<UCStr<C>>) -> Self {
        s.into_ucstring()
    }
}

impl<C: UChar> From<UCString<C>> for Box<UCStr<C>> {
    #[inline]
    fn from(s: UCString<C>) -> Box<UCStr<C>> {
        s.into_boxed_ucstr()
    }
}

impl<C: UChar> Default for Box<UCStr<C>> {
    fn default() -> Box<UCStr<C>> {
        let boxed: Box<[C]> = Box::from([UChar::NUL]);
        unsafe { Box::from_raw(Box::into_raw(boxed) as *mut UCStr<C>) }
    }
}

impl<C: UChar> NulError<C> {
    /// Returns the position of the nul value in the slice that was provided to `U16CString`.
    pub fn nul_position(&self) -> usize {
        self.0
    }

    /// Consumes this error, returning the underlying vector of u16 values which generated the error
    /// in the first place.
    pub fn into_vec(self) -> Vec<C> {
        self.1
    }
}

impl<C: UChar> Into<Vec<C>> for NulError<C> {
    fn into(self) -> Vec<C> {
        self.into_vec()
    }
}

impl<C: UChar> core::fmt::Display for NulError<C> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "nul value found at position {}", self.0)
    }
}

#[cfg(feature = "std")]
impl<C: UChar> std::error::Error for NulError<C> {
    fn description(&self) -> &str {
        "nul value found"
    }
}

/// An owned, mutable C-style "wide" string for FFI that is nul-aware and nul-terminated.
///
/// `U16CString` is aware of nul values. Unless unchecked conversions are used, all `U16CString`
/// strings end with a nul-terminator in the underlying buffer and contain no internal nul values.
/// The strings may still contain invalid or ill-formed UTF-16 data. These strings are intended to
/// be used with FFI functions such as Windows API that may require nul-terminated strings.
///
/// `U16CString` can be converted to and from many other string types, including `U16String`,
/// `OsString`, and `String`, making proper Unicode FFI safe and easy.
///
/// # Examples
///
/// The following example constructs a `U16CString` and shows how to convert a `U16CString` to a
/// regular Rust `String`.
///
/// ```rust
/// use widestring::U16CString;
/// let s = "Test";
/// // Create a wide string from the rust string
/// let wstr = U16CString::from_str(s).unwrap();
/// // Convert back to a rust string
/// let rust_str = wstr.to_string_lossy();
/// assert_eq!(rust_str, "Test");
/// ```
pub type U16CString = UCString<u16>;

/// An owned, mutable C-style wide string for FFI that is nul-aware and nul-terminated.
///
/// `U32CString` is aware of nul values. Unless unchecked conversions are used, all `U32CString`
/// strings end with a nul-terminator in the underlying buffer and contain no internal nul values.
/// The strings may still contain invalid or ill-formed UTF-32 data. These strings are intended to
/// be used with FFI functions such as Windows API that may require nul-terminated strings.
///
/// `U32CString` can be converted to and from many other string types, including `U32String`,
/// `OsString`, and `String`, making proper Unicode FFI safe and easy.
///
/// # Examples
///
/// The following example constructs a `U32CString` and shows how to convert a `U32CString` to a
/// regular Rust `String`.
///
/// ```rust
/// use widestring::U32CString;
/// let s = "Test";
/// // Create a wide string from the rust string
/// let wstr = U32CString::from_str(s).unwrap();
/// // Convert back to a rust string
/// let rust_str = wstr.to_string_lossy();
/// assert_eq!(rust_str, "Test");
/// ```
pub type U32CString = UCString<u32>;

/// Alias for `U16String` or `U32String` depending on platform. Intended to match typical C `wchar_t` size on platform.
pub type WideCString = UCString<WideChar>;
