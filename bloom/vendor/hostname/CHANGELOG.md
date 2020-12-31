# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.1] - 2020-02-28

### Fixed

- Enabling `hostname::set` doctests only if "set" feature is enabled (#10)

## [0.3.0] - 2019-12-19

### Added

- Cargo feature `set` which enables the `hostname::set` function compilation (disabled by default)
- Note that `hostname::set` will fail the compilation for Android API < 23

### Changed

- `hostname::set` is available only with Cargo `set` feature enabled
- Fix compilation issue for FreeBSD, DragonFlyBSD and iOS targets (#9)
- Deprecated function `get_hostname` was removed, use `get` instead

## [0.2.0] - 2019-11-09

### Added

- MSRV policy, Rust 1.19 version is set as minimally supported
- `get` function which returns the current hostname (replaces `get_hostname` function)
- `set` function which allows to change the hostname

### Changed

- Windows implementation returns the DNS host name of local computer instead of the NetBIOS name
- Windows implementation works with the Unicode now instead of ANSI encoding

### Fixed

- Possible value truncation is handled for *nix implementation (#6)

### Deprecated

- `get_hostname` function is deprecated and marked to be removed in the upcoming `0.3.0` version
