# Standback

![build status](https://github.com/jhpratt/standback/workflows/Build/badge.svg?branch=master&event=push)
![license](https://img.shields.io/badge/license-MIT%20or%20Apache--2-brightgreen)
![version](https://img.shields.io/crates/v/standback)
![rustc 1.31.0](https://img.shields.io/badge/rustc-1.31.0-blue)

[Documentation](https://docs.rs/standback)

Standback exists to allow the usage of various APIs that have been stabilized
since rustc 1.31.0 _without_ having to require users to upgrade their compiler.
The best part? Only old features are built from scratch; anything stable on the
compiler in use will just be re-exported.

Note that it is sometimes the case that newly stabilized methods would require
internal methods, direct access to fields, or nightly features to work. As such,
not every feature is backported. Found a neat way to implement a method or type
that is possible on stable? Pull requests are accepted!

## License

A majority of this code comes directly from the Rust standard library, where its
license is the following. All new code is also released under this license.

This project is licensed under either of

- [Apache License, Version 2.0](https://github.com/jhpratt/standback/blob/master/LICENSE-Apache)
- [MIT license](https://github.com/jhpratt/standback/blob/master/LICENSE-MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in time by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
