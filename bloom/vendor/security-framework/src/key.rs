//! Encryption key support

use core_foundation::base::TCFType;
#[cfg(any(feature = "OSX_10_12", target_os = "ios"))]
use core_foundation::base::ToVoid;
#[cfg(any(feature = "OSX_10_12", target_os = "ios"))]
use core_foundation::data::CFData;
#[cfg(any(feature = "OSX_10_12", target_os = "ios"))]
use core_foundation::dictionary::CFDictionary;
#[cfg(any(feature = "OSX_10_12", target_os = "ios"))]
use core_foundation_sys::error::CFErrorRef;
use security_framework_sys::base::SecKeyRef;
use security_framework_sys::key::SecKeyGetTypeID;
#[cfg(any(feature = "OSX_10_12", target_os = "ios"))]
use security_framework_sys::key::{SecKeyCopyAttributes, SecKeyCopyExternalRepresentation};
use std::fmt;

declare_TCFType! {
    /// A type representing an encryption key.
    SecKey, SecKeyRef
}
impl_TCFType!(SecKey, SecKeyRef, SecKeyGetTypeID);

unsafe impl Sync for SecKey {}
unsafe impl Send for SecKey {}

impl SecKey {
    #[cfg(any(feature = "OSX_10_12", target_os = "ios"))]
    /// Translates to SecKeyCopyAttributes
    pub fn attributes(&self) -> CFDictionary {
        let pka = unsafe { SecKeyCopyAttributes(self.to_void() as _) };
        unsafe { CFDictionary::wrap_under_create_rule(pka) }
    }

    #[cfg(any(feature = "OSX_10_12", target_os = "ios"))]
    /// Translates to SecKeyCopyExternalRepresentation
    pub fn external_representation(&self) -> Option<CFData> {
        let mut error: CFErrorRef = ::std::ptr::null_mut();
        let data = unsafe { SecKeyCopyExternalRepresentation(self.to_void() as _, &mut error) };
        if data == ::std::ptr::null() {
            return None;
        }
        Some(unsafe { CFData::wrap_under_create_rule(data) })
    }
}

// FIXME
impl fmt::Debug for SecKey {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "SecKey")
    }
}
