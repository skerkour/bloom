use core_foundation_sys::base::{CFTypeID, CFTypeRef, OSStatus};

use crate::base::{SecCertificateRef, SecIdentityRef, SecKeyRef};

extern "C" {
    pub fn SecIdentityGetTypeID() -> CFTypeID;
    pub fn SecIdentityCopyCertificate(
        identity: SecIdentityRef,
        certificate_ref: *mut SecCertificateRef,
    ) -> OSStatus;
    pub fn SecIdentityCopyPrivateKey(identity: SecIdentityRef, key_ref: *mut SecKeyRef)
        -> OSStatus;
    #[cfg(target_os = "macos")]
    pub fn SecIdentityCreateWithCertificate(
        keychain_or_Array: CFTypeRef,
        certificate_ref: SecCertificateRef,
        identity_ref: *mut SecIdentityRef,
    ) -> OSStatus;
}
