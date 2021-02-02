//! # hyper-rustls
//!
//! A pure-Rust HTTPS connector for [hyper](https://hyper.rs), based on [Rustls](https://github.com/ctz/rustls).
//!
//! ## Example
//!
//! ```no_run
//! # #[cfg(all(any(feature = "rustls-native-certs", feature = "webpki-roots"), feature = "tokio-runtime"))]
//! # fn main() {
//! use hyper::{Body, Client, StatusCode, Uri};
//!
//! let mut rt = tokio::runtime::Runtime::new().unwrap();
//! let url = ("https://hyper.rs").parse().unwrap();
//! let https = hyper_rustls::HttpsConnector::new();
//!
//! let client: Client<_, hyper::Body> = Client::builder().build(https);
//!
//! let res = rt.block_on(client.get(url)).unwrap();
//! assert_eq!(res.status(), StatusCode::OK);
//! # }
//! # #[cfg(not(feature = "tokio-runtime"))]
//! # fn main() {}
//! ```

#[cfg(all(
    feature = "tokio-runtime",
    any(not(feature = "rustls-native-certs"), feature = "webpki-roots"),
    any(not(feature = "webpki-roots"), feature = "rustls-native-certs")
))]
compile_error!(
    "Must enable exactly one of rustls-native-certs (default) or webpki-roots with tokio-runtime! (note: use `default-features = false' in a binary crate for one or other)"
);

mod connector;
mod stream;

pub use crate::connector::HttpsConnector;
pub use crate::stream::MaybeHttpsStream;
