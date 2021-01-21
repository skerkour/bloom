# hyper*x*

[![Rustdoc](https://docs.rs/hyperx/badge.svg)](https://docs.rs/hyperx)
[![Change Log](https://img.shields.io/crates/v/hyperx.svg?maxAge=3600&label=change%20log&color=9cf)](https://github.com/dekellum/hyperx/blob/master/CHANGELOG.md)
[![crates.io](https://img.shields.io/crates/v/hyperx.svg?maxAge=3600)](https://crates.io/crates/hyperx)
[![CI Status](https://github.com/dekellum/hyperx/workflows/CI/badge.svg?branch=master)](https://github.com/dekellum/hyperx/actions?query=workflow%3ACI)
[![deps status](https://deps.rs/repo/github/dekellum/hyperx/status.svg)](https://deps.rs/repo/github/dekellum/hyperx)

[Hyper] is the low-level HTTP implementation for Rust. Hyper*x* is an
e*x*traction of the hyper 0.11 typed header module, with minimized
dependencies, for continued use with hyper 0.12 or later,
where it was dropped.

[Hyper]: https://github.com/hyperium/hyper

## Minimum supported rust version

MSRV := 1.39.0

The crate will fail fast on any lower rustc (via a build.rs version
check) and is also CI tested on this version.

## License

The MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)
