![Logo](https://raw.githubusercontent.com/ctz/rustls/master/admin/rustls-logo-web.png)

**rustls-native-certs** allows [rustls](https://github.com/ctz/rustls) to use the
platform's native certificate store when operating as a TLS client.

This is supported on Windows, macOS and Linux:

- On Windows, certificates are loaded from the system certificate store.
  The [`schannel`](https://github.com/steffengy/schannel-rs) crate is used to access
  the Windows certificate store APIs.
- On macOS, certificates are loaded from the keychain.
  The user, admin and system trust settings are merged together as documented
  by Apple.  The [`security-framework`](https://github.com/kornelski/rust-security-framework)
  crate is used to access the keystore APIs.
- On Linux and other UNIX-like operating systems, the
  [`openssl-probe`](https://github.com/alexcrichton/openssl-probe) crate is used to discover
  the filename of the system CA bundle.

# Status
rustls-native-certs is currently in development.

If you'd like to help out, please see [CONTRIBUTING.md](CONTRIBUTING.md).

[![Build Status](https://dev.azure.com/ctz99/ctz/_apis/build/status/ctz.rustls-native-certs?branchName=master)](https://dev.azure.com/ctz99/ctz/_build/latest?definitionId=5&branchName=master)
[![Documentation](https://docs.rs/rustls-native-certs/badge.svg)](https://docs.rs/rustls-native-certs/)

## Release history:

* 0.2.0 (2020-01-26):
  - Return valid certificates even in the presence of invalid ones.  This allows
    callers to opt-in to "best effort" behaviour.
* 0.1.0 (2019-11-04):
  - Initial release.

# API

This library exposes a single function with this signature:

```rust
pub fn load_native_certs() -> Result<rustls::RootCertStore, (Option<rustls::RootCertStore>, std::io::Error)>
```

On success, this returns a `rustls::RootCertStore` loaded with a
snapshop of the root certificates found on this platform.  This
function fails in a platform-specific way, expressed in a `std::io::Error`.

When an error is returned, optionally a `rustls::RootCertStore` is also
returned containing the certificates which *could* be loaded.  This means
callers can opt-in to "best-effort" behaviour even in the presence of invalid
certificates.

This function can be expensive: on some platforms it involves loading
and parsing a ~300KB disk file.  It's therefore prudent to call
this sparingly.

# Worked example

See [`examples/google.rs`](examples/google.rs).

# Should I use this or `webpki-roots`?

(Background: [webpki-roots](https://crates.io/crates/webpki-roots) is a crate that compiles-in Mozilla's set of root certificates.)

This crate is preferable in many ways to *webpki-roots*.
To sum up the pros and cons:

Pros:

- **This crate respects local configuration of root certificates**: both
  removal of roots that the user finds untrustworthy, and addition of locally-trusted roots.
  _The latter case is exceedingly important if your application is required to work in
  enterprise environments with "transparent" TLS-terminating middleboxes._
- **This crate instantaneously reflects underlying system configuration**.  _Since webpki-roots
  compiles in root certificates, getting an update to these requires taking regular updates
  to this crate, plus recompilation and redeployment of the application.  This is a long-winded
  process that may become a liability in the event of a severe misissuance._
- **This crate is compatible with developer aids** such as [mkcert](https://github.com/FiloSottile/mkcert).

Cons:

- **The OS certificate store is occasionally "attacked" by [malware](https://en.wikipedia.org/wiki/Superfish)**
  or just [bad software](https://sennheiser.zendesk.com/hc/en-us/articles/360011888254).
- **The OS update system may, in fact, be quite poor at keeping the root certificates up-to-date**
  if it is disabled or out-of-support.
- **The quality of the `ca-certificates` package on debian-based Linux distributions is poor**.
  At the time of writing, this ships many certificates not included in the Mozilla
  set, either because they [failed an audit and were withdrawn](https://bugzilla.mozilla.org/show_bug.cgi?id=1448506) or
  [were removed for mississuance](https://bugzilla.mozilla.org/show_bug.cgi?id=1552374).
- **You may prefer to insulate yourself against local configuration** for support or
  (perhaps inadvisable) security reasons.

# License

rustls-native-certs is distributed under the following three licenses:

- Apache License version 2.0.
- MIT license.
- ISC license.

These are included as LICENSE-APACHE, LICENSE-MIT and LICENSE-ISC
respectively.  You may use this software under the terms of any
of these licenses, at your option.
