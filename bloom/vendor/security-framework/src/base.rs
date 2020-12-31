//! Support types for other modules.

use core_foundation::string::CFString;
use core_foundation_sys::base::OSStatus;
use std::error;
use std::fmt;
use std::result;

/// A `Result` type commonly returned by functions.
pub type Result<T> = result::Result<T, Error>;

/// A Security Framework error.
#[derive(Copy, Clone)]
pub struct Error(OSStatus);

impl fmt::Debug for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = fmt.debug_struct("Error");
        builder.field("code", &self.0);
        if let Some(message) = self.message() {
            builder.field("message", &message);
        }
        builder.finish()
    }
}

impl Error {
    /// Creates a new `Error` from a status code.
    pub fn from_code(code: OSStatus) -> Self {
        Self(code)
    }

    /// Returns a string describing the current error, if available.
    pub fn message(self) -> Option<String> {
        self.inner_message()
    }

    #[cfg(target_os = "macos")]
    fn inner_message(self) -> Option<String> {
        use core_foundation::base::TCFType;
        use security_framework_sys::base::SecCopyErrorMessageString;
        use std::ptr;

        unsafe {
            let s = SecCopyErrorMessageString(self.0, ptr::null_mut());
            if s.is_null() {
                None
            } else {
                Some(CFString::wrap_under_create_rule(s).to_string())
            }
        }
    }

    #[cfg(not(target_os = "macos"))]
    fn inner_message(&self) -> Option<String> {
        None
    }

    /// Returns the code of the current error.
    pub fn code(self) -> OSStatus {
        self.0
    }
}

impl From<OSStatus> for Error {
    fn from(code: OSStatus) -> Self {
        Self::from_code(code)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(message) = self.message() {
            write!(fmt, "{}", message)
        } else {
            write!(fmt, "error code {}", self.code())
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "Security Framework error"
    }
}
