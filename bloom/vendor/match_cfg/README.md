# match_cfg

[![Build Status](https://travis-ci.com/gnzlbg/match_cfg.svg?branch=master)](https://travis-ci.com/gnzlbg/match_cfg)

[Documentation](https://docs.rs/match_cfg)

**Minimum Supported Rust Version**: 1.13.0.

A convenience macro to ergonomically define an item depending on a large number
of `#[cfg]` parameters. Structured like match statement, the first matching
branch is the item that gets emitted.

```toml
[dependencies]
match_cfg = "0.1"
```

The `use_core` feature is enabled by default and builds the crate with `libcore`
as a dependency by using the `#![no_std]` attribute. When this feature is
disabled, this crate is built without libcore support by using the `#![no_core]`
attribute - this makes use of the `#![feature(no_core)]` and requires a nightly
version of Rust.

## Example

```rust
#[macro_use(match_cfg)]
extern crate match_cfg;

match_cfg! {
    #[cfg(unix)] => {
         fn foo() { /* unix specific functionality */ }
     }
     #[cfg(target_pointer_width = "32")] => {
         fn foo() { /* non-unix, 32-bit functionality */ }
     }
     _ => {
         fn foo() { /* fallback implementation */ }
     }
}

fn main() {
    foo();
}
```

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Serde by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
