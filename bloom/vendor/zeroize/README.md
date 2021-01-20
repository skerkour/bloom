# [zeroize].rs 🄌 <a href="https://www.iqlusion.io"><img src="https://storage.googleapis.com/iqlusion-production-web/img/logo/iqlusion-rings-sm.png" alt="iqlusion" width="24" height="24"></a>

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Apache 2.0/MIT Licensed][license-image]
![MSRV][rustc-image]
[![Build Status][build-image]][build-link]
[![Gitter Chat][gitter-image]][gitter-link]

Securely zero memory while avoiding compiler optimizations.

This crate implements a portable approach to securely zeroing memory using
techniques which guarantee they won't be "optimized away" by the compiler.

The [`Zeroize` trait] is the crate's primary API.

[Documentation]

## About

[Zeroing memory securely is hard] - compilers optimize for performance, and
in doing so they love to "optimize away" unnecessary zeroing calls. There are
many documented "tricks" to attempt to avoid these optimizations and ensure
that a zeroing routine is performed reliably.

This crate isn't about tricks: it uses [core::ptr::write_volatile]
and [core::sync::atomic] memory fences to provide easy-to-use, portable
zeroing behavior which works on all of Rust's core number types and slices
thereof, implemented in pure Rust with no usage of FFI or assembly.

- No insecure fallbacks!
- No dependencies!
- No FFI or inline assembly! **WASM friendly** (and tested)!
- `#![no_std]` i.e. **embedded-friendly**!
- No functionality besides securely zeroing memory!
- (Optional) Custom derive support for zeroing complex structures

## Minimum Supported Rust Version

Requires Rust **1.44** or newer.

In the future, we reserve the right to change MSRV (i.e. MSRV is out-of-scope
for this crate's SemVer guarantees), however when we do it will be accompanied
with a minor version bump.

## License

**zeroize** is distributed under the terms of either the MIT license
or the Apache License (Version 2.0), at your option.

See [LICENSE] (Apache License, Version 2.0) file in the `iqlusioninc/crates`
toplevel directory of this repository or [LICENSE-MIT] for details.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you shall be dual licensed as above,
without any additional terms or conditions.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/zeroize.svg
[crate-link]: https://crates.io/crates/zeroize
[docs-image]: https://docs.rs/zeroize/badge.svg
[docs-link]: https://docs.rs/zeroize/
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.44+-blue.svg
[build-image]: https://github.com/iqlusioninc/crates/workflows/Rust/badge.svg?branch=develop&event=push
[build-link]: https://github.com/iqlusioninc/crates/actions
[gitter-image]: https://badges.gitter.im/iqlusioninc/community.svg
[gitter-link]: https://gitter.im/iqlusioninc/community

[//]: # (general links)

[zeroize]: https://en.wikipedia.org/wiki/Zeroisation
[`Zeroize` trait]: https://docs.rs/zeroize/latest/zeroize/trait.Zeroize.html
[Documentation]: https://docs.rs/zeroize/
[Zeroing memory securely is hard]: http://www.daemonology.net/blog/2014-09-04-how-to-zero-a-buffer.html
[core::ptr::write_volatile]: https://doc.rust-lang.org/core/ptr/fn.write_volatile.html
[core::sync::atomic]: https://doc.rust-lang.org/stable/core/sync/atomic/index.html
[good cryptographic hygiene]: https://github.com/veorq/cryptocoding#clean-memory-of-secret-data
[LICENSE]: https://github.com/iqlusioninc/crates/blob/develop/LICENSE
[LICENSE-MIT]: https://github.com/iqlusioninc/crates/blob/develop/zeroize/LICENSE-MIT
