# Serde Querystrings [![badge-ci]][badge-ci-link] [![Latest Version]][crates.io] [![Documentation]][docs-rs] 


[badge-ci]: https://github.com/samscott89/serde_qs/workflows/Rust%20CI%20checks/badge.svg
[badge-ci-link]: https://github.com/samscott89/serde_qs/actions?query=workflow%3A%22Rust+CI+checks%22+branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/serde_qs.svg
[crates.io]: https://crates.io/crates/serde\_qs
[Documentation]: https://docs.rs/serde_qs/badge.svg
[docs-rs]: https://docs.rs/serde_qs/

This crate is a Rust library for serialising to and deserialising from
querystrings. This crate is designed to extend [`serde_urlencoded`][urlencoded]
when using nested parameters, similar to those used by [qs][qs] for Node, and
commonly used by Ruby on Rails via [Rack][Rack].

The core of the library was inspired by
[`serde_urlencoded`][urlencoded], which should be preferred
over this crate whenever non-nested query parameters are sufficient. It is built
upon [Serde], a high performance generic serialization framework and [rust-url],
a URL parser for Rust.

[rust-url]: https://github.com/servo/rust-url
[Serde]: https://github.com/serde-rs/serde
[urlencoded]: https://github.com/nox/serde_urlencoded
[qs]: https://www.npmjs.com/package/qs
[Rack]: http://www.rubydoc.info/github/rack/rack/Rack/Utils#parse_nested_query-class_method

Installation
============

This crate works with Cargo and can be found on
[crates.io] with a `Cargo.toml` like:

```toml
[dependencies]
serde_qs = "0.8"
```

Minimum supported Rust version is 1.36.

[crates.io]: https://crates.io/crates/serde_qs

## License

serde_qs is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in serde_qs by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
