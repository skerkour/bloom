//! TLS acceptor services for Actix ecosystem.
//!
//! ## Crate Features
//! * `openssl` - TLS acceptor using the `openssl` crate.
//! * `rustls` - TLS acceptor using the `rustls` crate.
//! * `nativetls` - TLS acceptor using the `native-tls` crate.

#![deny(rust_2018_idioms)]

use std::sync::atomic::{AtomicUsize, Ordering};

use actix_utils::counter::Counter;

#[cfg(feature = "openssl")]
pub mod openssl;

#[cfg(feature = "rustls")]
pub mod rustls;

#[cfg(feature = "nativetls")]
pub mod nativetls;

pub(crate) static MAX_CONN: AtomicUsize = AtomicUsize::new(256);

thread_local! {
    static MAX_CONN_COUNTER: Counter = Counter::new(MAX_CONN.load(Ordering::Relaxed));
}

/// Sets the maximum per-worker concurrent TLS connection limit.
///
/// All listeners will stop accepting connections when this limit is reached.
/// It can be used to regulate the global TLS CPU usage.
///
/// By default, the connection limit is 256.
pub fn max_concurrent_tls_connect(num: usize) {
    MAX_CONN.store(num, Ordering::Relaxed);
}

/// TLS error combined with service error.
#[derive(Debug)]
pub enum TlsError<E1, E2> {
    Tls(E1),
    Service(E2),
}
