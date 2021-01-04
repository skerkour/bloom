# Changelog

All notable changes to the standback project will be documented in this file.

The format is based on [Keep a Changelog]. This project adheres to [Semantic
Versioning].

---

## 0.2.14 [2020-12-30]

- Support for Rust 1.49.0 APIs has been added.

## 0.2.13 [2020-11-16]

- Support for Rust 1.48.0 APIs have been added.

## 0.2.12 [2020-11-16] [YANKED]

This version does not build, and has been yanked.

## 0.2.11 [2020-10-07]

- Support for Rust 1.47.0 APIs has been added.

## 0.2.10 [2020-08-23]

- Support for Rust 1.45.0 and 1.46.0 APIs has been added.
- `float::to_int_unchecked` has been added (stabilized in Rust 1.44.0).

## 0.2.9 [2020-06-04]

- Support for Rust 1.44.0 APIs has been added.
- Non-stable releases are now supported. All releases are treated as equivalent
  to the stable that was out at the time.

## 0.2.8 [2020-04-27]

Removed the `primitive` module, as it caused internal compiler errors in older
compilers.

## 0.2.7 [2020-04-25]

_This version has been yanked from crates.io._

Additional modules and constants were added for Rust 1.43.0.

## 0.2.6 [2020-04-21]

Support for Rust 1.43.0 APIs has been added.

## 0.2.5 [2020-04-20]

- `TryFrom`, `TryInto` and correctly re-exported.
- `TryFromIntError` has been moved to `mod num`.

## 0.2.4 [2020-04-20]

- `TryFrom` identity is implemented for some primitives.

## 0.2.3 [2020-04-18]

Embedded and `#![no_std]` now have full support and are checked in CI.

## 0.2.2 [2020-04-02]

The version of rustc being used will now respect the $RUSTC environment
variable.

## 0.2.1 [2020-03-13]

Support for Rust 1.42.0 APIs has been added.

## 0.2.0 [2020-03-10]

- `MaybeUninit` is restricted to `Copy` types, eliminating undefined behavior.
- `todo!` has been moved to the prelude.

## 0.1.0 [2020-03-05]

Initial release.
