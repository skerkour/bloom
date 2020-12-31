use core_foundation_sys::array::CFArrayRef;
use core_foundation_sys::base::{CFAllocatorRef, CFTypeID, OSStatus};
use core_foundation_sys::data::CFDataRef;
use core_foundation_sys::dictionary::CFDictionaryRef;
use core_foundation_sys::error::CFErrorRef;
use core_foundation_sys::string::CFStringRef;

use crate::base::{SecCertificateRef, SecKeyRef};

extern "C" {
    #[cfg(target_os = "macos")]
    pub static kSecPropertyKeyType: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecPropertyKeyLabel: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecPropertyKeyLocalizedLabel: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecPropertyKeyValue: CFStringRef;

    #[cfg(target_os = "macos")]
    pub static kSecPropertyTypeWarning: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecPropertyTypeSuccess: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecPropertyTypeSection: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecPropertyTypeData: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecPropertyTypeString: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecPropertyTypeURL: CFStringRef;
    #[cfg(target_os = "macos")]
    pub static kSecPropertyTypeDate: CFStringRef;

    pub fn SecCertificateGetTypeID() -> CFTypeID;
    pub fn SecCertificateCreateWithData(
        allocator: CFAllocatorRef,
        data: CFDataRef,
    ) -> SecCertificateRef;
    pub fn SecCertificateCopyData(certificate: SecCertificateRef) -> CFDataRef;
    pub fn SecCertificateCopySubjectSummary(certificate: SecCertificateRef) -> CFStringRef;
    pub fn SecCertificateCopyCommonName(
        certificate: SecCertificateRef,
        common_name: *mut CFStringRef,
    ) -> OSStatus;
    #[cfg(target_os = "macos")]
    pub fn SecCertificateCopyPublicKey(
        certificate: SecCertificateRef,
        key: *mut SecKeyRef,
    ) -> OSStatus;
    #[cfg(target_os = "macos")]
    pub fn SecCertificateCopyValues(
        certificate: SecCertificateRef,
        keys: CFArrayRef,
        error: *mut CFErrorRef,
    ) -> CFDictionaryRef;
}
