# widestring
[![Crates.io](https://img.shields.io/crates/v/widestring.svg)](https://crates.io/crates/widestring/) [![docs.rs](https://docs.rs/widestring/badge.svg)](https://docs.rs/widestring/) [![Build status](https://ci.appveyor.com/api/projects/status/97pmbv6kk79bicww?svg=true)](https://ci.appveyor.com/project/starkat99/widestring-rs) [![Build Status](https://travis-ci.org/starkat99/widestring-rs.svg?branch=master)](https://travis-ci.org/starkat99/widestring-rs)

A wide string Rust FFI library for converting to and from wide strings, such as
those often used in Windows API or other FFI libaries. Both UTF-16 and UTF-32 types are provided, including support for malformed encoding.

## Documentation

- [Crate API Reference](https://docs.rs/widestring/)
- [Latest Changes](CHANGELOG.md)

### Optional Features

- **`alloc`** - Enabled by default. Enable use of the [`alloc`](https://doc.rust-lang.org/alloc/)
  crate when not using the `std` library.

  This enables the `U16String`, `U32String`, `U16CString`, `U32CString` types and alises.

- **`std`** - Enabled by default. Enable features that depend on the Rust `std` library, including
  everything in the `alloc` feature.

## License

This library is distributed under the terms of either of:

* MIT license ([LICENSE-MIT](LICENSE-MIT) or
[http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
[http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.