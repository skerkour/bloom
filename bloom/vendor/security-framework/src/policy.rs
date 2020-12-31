//! Security Policies support.
use core_foundation::base::TCFType;
use core_foundation::string::CFString;
use security_framework_sys::base::SecPolicyRef;
use security_framework_sys::policy::*;
use std::fmt;
use std::ptr;

use crate::secure_transport::SslProtocolSide;

declare_TCFType! {
    /// A type representing a certificate validation policy.
    SecPolicy, SecPolicyRef
}
impl_TCFType!(SecPolicy, SecPolicyRef, SecPolicyGetTypeID);

unsafe impl Sync for SecPolicy {}
unsafe impl Send for SecPolicy {}

impl fmt::Debug for SecPolicy {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("SecPolicy").finish()
    }
}

impl SecPolicy {
    /// Creates a `SecPolicy` for evaluating SSL certificate chains.
    ///
    /// The side which you are evaluating should be provided (i.e. pass `SslSslProtocolSide::SERVER` if
    /// you are a client looking to validate a server's certificate chain).
    pub fn create_ssl(protocol_side: SslProtocolSide, hostname: Option<&str>) -> Self {
        let hostname = hostname.map(CFString::new);
        let hostname = hostname
            .as_ref()
            .map(|s| s.as_concrete_TypeRef())
            .unwrap_or(ptr::null_mut());
        let is_server = protocol_side == SslProtocolSide::SERVER;
        unsafe {
            let policy = SecPolicyCreateSSL(is_server as _, hostname);
            Self::wrap_under_create_rule(policy)
        }
    }

    /// Returns a policy object for the default X.509 policy.
    pub fn create_x509() -> Self {
        unsafe {
            let policy = SecPolicyCreateBasicX509();
            Self::wrap_under_create_rule(policy)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::policy::SecPolicy;
    use crate::secure_transport::SslProtocolSide;

    #[test]
    fn create_ssl() {
        SecPolicy::create_ssl(SslProtocolSide::SERVER, Some("certifi.org"));
    }
}
