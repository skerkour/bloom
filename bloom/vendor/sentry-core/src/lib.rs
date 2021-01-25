//! This crate provides the core of the [Sentry](https://sentry.io/) SDK, which
//! can be used to log events and errors.
//!
//! This crate is meant for integration authors and third party library authors
//! that want to instrument their code for sentry.
//!
//! Regular users who wish to integrate sentry into their applications should
//! rather use the [`sentry`] crate, which comes with a default transport, and
//! a large set of integrations for various third-party libraries.
//!
//! # Core Concepts
//!
//! This crate follows the [Unified API] guidelines and is centered around
//! the concepts of [`Client`], [`Hub`] and [`Scope`], as well as the extension
//! points via the [`Integration`], [`Transport`] and [`TransportFactory`] traits.
//!
//! # Minimal API
//!
//! By default, this crate comes with a so-called "minimal" mode. This mode will
//! provide all the APIs needed to instrument code with sentry, and to write
//! sentry integrations, but it will blackhole a lot of operations.
//!
//! In minimal mode some types are restricted in functionality. For instance
//! the [`Client`] is not available and the [`Hub`] does not retain all API
//! functionality.
//!
//! # Features
//!
//! * `feature = "client"`: Activates the [`Client`] type and certain
//!   [`Hub`] functionality.
//! * `feature = "test"`: Activates the [`test`] module, which can be used to
//!   write integration tests. It comes with a test transport which can capture
//!   all sent events for inspection.
//! * `feature = "debug-logs"`: Uses the `log` crate for debug output, instead
//!   of printing to `stderr`. This feature is **deprecated** and will be
//!   replaced by a dedicated log callback in the future.
//!
//! [`sentry`]: https://crates.io/crates/sentry
//! [Unified API]: https://develop.sentry.dev/sdk/unified-api/
//! [`Client`]: struct.Client.html
//! [`Hub`]: struct.Hub.html
//! [`Scope`]: struct.Scope.html
//! [`Integration`]: trait.Integration.html
//! [`Transport`]: trait.Transport.html
//! [`TransportFactory`]: trait.TransportFactory.html
//! [`test`]: test/index.html

#![doc(html_favicon_url = "https://sentry-brand.storage.googleapis.com/favicon.ico")]
#![doc(html_logo_url = "https://sentry-brand.storage.googleapis.com/sentry-glyph-black.png")]
#![warn(missing_docs)]

// macros; these need to be first to be used by other modules
#[macro_use]
mod macros;

mod api;
mod breadcrumbs;
mod clientoptions;
mod constants;
mod error;
mod futures;
mod hub;
mod integration;
mod intodsn;
mod scope;
mod transport;

// public api or exports from this crate
pub use crate::api::*;
pub use crate::breadcrumbs::IntoBreadcrumbs;
pub use crate::clientoptions::ClientOptions;
pub use crate::error::{capture_error, event_from_error, parse_type_from_debug};
pub use crate::futures::{SentryFuture, SentryFutureExt};
pub use crate::hub::Hub;
pub use crate::integration::Integration;
pub use crate::intodsn::IntoDsn;
pub use crate::scope::{Scope, ScopeGuard};
pub use crate::transport::{Transport, TransportFactory};

// client feature
#[cfg(feature = "client")]
mod client;
#[cfg(feature = "client")]
mod session;
#[cfg(feature = "client")]
pub use crate::client::Client;

// test utilities
#[cfg(feature = "test")]
pub mod test;

// public api from other crates
#[doc(inline)]
pub use sentry_types as types;
pub use sentry_types::protocol::v7 as protocol;
pub use sentry_types::protocol::v7::{Breadcrumb, Envelope, Level, User};
