[![License:Zlib](https://img.shields.io/badge/License-Zlib-brightgreen.svg)](https://opensource.org/licenses/Zlib)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.34-green.svg)
[![crates.io](https://img.shields.io/crates/v/bytemuck.svg)](https://crates.io/crates/bytemuck)
[![docs.rs](https://docs.rs/bytemuck/badge.svg)](https://docs.rs/bytemuck/)

# bytemuck

A crate for mucking around with piles of bytes.

## Extensions

There is experimental support for the `Zeroable` trait being derived through a
proc-macro. I'm not the author of that crate, please file bugs with that crate
in the other repo.

* https://github.com/rodrimati1992/zeroable_crates

## Stability

The goal is to stay at 1.y.z until _at least_ the next edition of Rust.

I consider any increase of the Minimum Rust Version to be a semver breaking change,
so `rustc-1.34` will continue to be supported for at least the rest of the
`bytemuck-1.y.z` series of the crate.

(The secret goal is to get all of this functionality into the standard library
some day so that we don't even need to import a crate to do all this fun stuff.)
