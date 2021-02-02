//! rustls-native-certs allows rustls to use the platform's native certificate
//! store when operating as a TLS client.
//!
//! It consists of a single function [load_native_certs](fn.load_native_certs.html) which returns a
//! `rustls::RootCertStore` pre-filled from the native certificate store.

/// Like `Result<T,E>`, but allows for functions that can return partially complete
/// work alongside an error.
pub type PartialResult<T, E> = Result<T, (Option<T>, E)>;

#[cfg(all(unix, not(target_os = "macos")))]
mod unix;
#[cfg(all(unix, not(target_os = "macos")))]
use unix as platform;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
use windows as platform;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use macos as platform;

pub use platform::load_native_certs;
