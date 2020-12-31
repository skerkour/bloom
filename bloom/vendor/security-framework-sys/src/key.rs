use core_foundation_sys::base::CFTypeID;
use core_foundation_sys::data::CFDataRef;
use core_foundation_sys::dictionary::CFDictionaryRef;
use core_foundation_sys::error::CFErrorRef;

use crate::base::SecKeyRef;

extern "C" {
    pub fn SecKeyGetTypeID() -> CFTypeID;

    #[cfg(target_os = "macos")]
    pub fn SecKeyCreateFromData(
        parameters: CFDictionaryRef,
        keyData: CFDataRef,
        error: *mut CFErrorRef,
    ) -> SecKeyRef;

    #[cfg(any(feature = "OSX_10_12", target_os = "ios"))]
    pub fn SecKeyCopyExternalRepresentation(key: SecKeyRef, error: *mut CFErrorRef) -> CFDataRef;
    #[cfg(any(feature = "OSX_10_12", target_os = "ios"))]
    pub fn SecKeyCopyAttributes(key: SecKeyRef) -> CFDictionaryRef;
}
