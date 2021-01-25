//! This crate provides support for logging events and errors / panics to the
//! [Sentry](https://sentry.io/) error logging service.  It integrates with the standard panic
//! system in Rust as well as a few popular error handling setups.
//!
//! # Quickstart
//!
//! The most convenient way to use this library is the [`sentry::init`] function,
//! which starts a sentry client with a default set of integrations, and binds
//! it to the current [`Hub`].
//!
//! The [`sentry::init`] function returns a guard that when dropped will flush Events that were not
//! yet sent to the sentry service.  It has a two second deadline for this so shutdown of
//! applications might slightly delay as a result of this.  Keep the guard around or sending events
//! will not work.
//!
//! ```
//! let _guard = sentry::init("https://key@sentry.io/42");
//! sentry::capture_message("Hello World!", sentry::Level::Info);
//! // when the guard goes out of scope here, the client will wait up to two
//! // seconds to send remaining events to the service.
//! ```
//!
//! [`sentry::init`]: fn.init.html
//! [`Hub`]: struct.Hub.html
//!
//! # Integrations
//!
//! What makes this crate useful are the various integrations that exist.  Some of them are enabled
//! by default, some uncommon ones or for deprecated parts of the ecosystem a feature flag needs to
//! be enabled.  For the available integrations and how to use them see
//! [integrations](integrations/index.html) and [apply_defaults](fn.apply_defaults.html).
//!
//! # Minimal API
//!
//! This crate comes fully featured. If the goal is to instrument libraries for usage
//! with sentry, or to extend sentry with a custom [`Integration`] or a [`Transport`],
//! one should use the [`sentry-core`] crate instead.
//!
//! [`Integration`]: trait.Integration.html
//! [`Transport`]: trait.Transport.html
//! [`sentry-core`]: https://crates.io/crates/sentry-core
//!
//! # Features
//!
//! Functionality of the crate can be turned on and off by feature flags.  This is the current list
//! of feature flags:
//!
//! Default features:
//!
//! * `backtrace`: Enables backtrace support.
//! * `contexts`: Enables capturing device, os, and rust contexts.
//! * `panic`: Enables support for capturing panics.
//! * `transport`: Enables the default transport, which is currently `reqwest` with `native-tls`.
//!
//! Additional features:
//!
//! * `anyhow`: Enables support for the `anyhow` crate.
//! * `debug-images`: Attaches a list of loaded libraries to events (currently only supported on unix).
//! * `error-chain`: Enables support for the `error-chain` crate.
//! * `failure`: Enables support for the `failure` crate.
//! * `log`: Enables support for the `log` crate.
//! * `env_logger`: Enables support for the `log` crate with additional `env_logger` support.
//! * `slog`: Enables support for the `slog` crate.
//! * `test`: Enables testing support.
//! * `debug-logs`: Uses the `log` crate for internal logging.
//! * `reqwest`: Enables the `reqwest` transport, which is currently the default.
//! * `curl`: Enables the curl transport.
//! * `surf`: Enables the surf transport.
//! * `native-tls`: Uses the `native-tls` crate, which is currently the default.
//!   This only has an effect on the `reqwest` transport.
//! * `rustls`: Enables the `rustls` support of the `reqwest` transport.
//!   Please note that `native-tls` is a default feature, and one needs to use
//!   `default-features = false` to completely disable building `native-tls` dependencies.

#![doc(html_favicon_url = "https://sentry-brand.storage.googleapis.com/favicon.ico")]
#![doc(html_logo_url = "https://sentry-brand.storage.googleapis.com/sentry-glyph-black.png")]
#![warn(missing_docs)]

mod defaults;
mod init;
mod transport;

// re-export from core
#[doc(inline)]
pub use sentry_core::*;

// added public API
pub use crate::defaults::apply_defaults;
pub use crate::init::{init, ClientInitGuard};

/// Available Sentry Integrations.
///
/// See the [`apply_defaults`] function for more information on which integrations are
/// used by default.
///
/// [`apply_defaults`]: ../fn.apply_defaults.html
pub mod integrations {
    #[cfg(feature = "anyhow")]
    #[doc(inline)]
    pub use sentry_anyhow as anyhow;
    #[cfg(feature = "backtrace")]
    #[doc(inline)]
    pub use sentry_backtrace as backtrace;
    #[cfg(feature = "contexts")]
    #[doc(inline)]
    pub use sentry_contexts as contexts;
    #[cfg(feature = "debug-images")]
    #[doc(inline)]
    pub use sentry_debug_images as debug_images;
    #[cfg(feature = "error-chain")]
    #[doc(inline)]
    #[deprecated = "The `error_chain` integration is deprecated and will be removed in the future."]
    pub use sentry_error_chain as error_chain;
    #[cfg(feature = "failure")]
    #[doc(inline)]
    #[deprecated = "The `failure` integration is deprecated and will be removed in the future."]
    pub use sentry_failure as failure;
    #[cfg(feature = "log")]
    #[doc(inline)]
    pub use sentry_log as log;
    #[cfg(feature = "panic")]
    #[doc(inline)]
    pub use sentry_panic as panic;
    #[cfg(feature = "slog")]
    #[doc(inline)]
    pub use sentry_slog as slog;
}

#[doc(inline)]
pub use sentry_core::types::protocol::latest as protocol;

/// The provided transports.
///
/// This module exposes all transports that are compiled into the sentry
/// library.  The `reqwest`, `curl` and `surf` features turn on these transports.
pub mod transports {
    pub use crate::transport::DefaultTransportFactory;

    #[cfg(feature = "reqwest")]
    pub use crate::transport::ReqwestHttpTransport;

    #[cfg(feature = "curl")]
    pub use crate::transport::CurlHttpTransport;

    #[cfg(feature = "surf")]
    pub use crate::transport::SurfHttpTransport;

    #[cfg(any(feature = "reqwest", feature = "curl", feature = "surf"))]
    pub use crate::transport::HttpTransport;
}
