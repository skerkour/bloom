use core_foundation_sys::base::{Boolean, CFTypeID};
use core_foundation_sys::string::CFStringRef;

use crate::base::SecPolicyRef;

extern "C" {
    pub fn SecPolicyCreateSSL(server: Boolean, hostname: CFStringRef) -> SecPolicyRef;
    pub fn SecPolicyGetTypeID() -> CFTypeID;
    pub fn SecPolicyCreateBasicX509() -> SecPolicyRef;
}
