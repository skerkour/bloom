//! This module exposes the types for the Sentry protocol in different versions.

#[cfg(feature = "protocol")]
pub mod v7;

/// The latest version of the protocol.
pub const LATEST: u16 = 7;

#[cfg(feature = "protocol")]
pub use v7 as latest;

mod envelope;
mod session;
