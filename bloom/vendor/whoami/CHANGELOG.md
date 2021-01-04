# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://github.com/AldaronLau/semver).

## [1.0.3] - 2020-12-31
### Fixed
 - Link to logo in documentation.

## [1.0.2] - 2020-12-31
### Changed
 - `distro()` on Windows now returns more detailed version.

## [1.0.1] - 2020-12-16
### Added
 - Official support for BSD and variants

### Fixed
 - `platform()` will now return `Platform::Bsd` when running BSD.
 - Misaligned address error on FreeBSD when calling `username()`.

## [1.0.0] - 2020-11-23
### Removed
 - `Platform::Dive` and `DesktopEnv::Dive`, as that was an OS idea not a real OS
 - Explicit support for `stdweb`, now built on `web-sys`/`wasm-bindgen`.

## [0.9.0] - 2020-06-24
### Added
 - `stdweb` and `wasm-bindgen` support
 - Versions of `-> String` functions that return `OsString`s:
   - `devicename_os()`
   - `distro_os()`
   - `hostname_os()`
   - `realname_os()`
   - `username_os()`

### Changed
 - Renamed `DesktopEnv::Mac` to `DesktopEnv::Aqua`
 - Renamed `DesktopEnv::Wasm` to `DesktopEnv::WebBrowser`
 - Renamed `DesktopEnv::Redox` to `DesktopEnv::Orbital`
 - Renamed `DesktopEnv::Fuchsia` to `DesktopEnv::Ermine`
 - Renamed `Platform::FreeBsd` to `Platform::Bsd`
 - Renamed `env()` to `desktop_env()`
 - Renamed `host()` to `devicename()`
 - Renamed `os()` to `distro()`
 - Renamed `user()` to `realname()`

### Fixed
 - Inconsistencies on Windows
 - MacOS running commands instead of using native APIs (this results in speed
   improvements on MacOS)
 - One of the Linux functions also using commands instead of native APIs (faster)

### Contributors
Thanks to everyone who contributed to make this version of whoami possible!

- [AldaronLau](https://github.com/AldaronLau)
- [Vlad-Shcherbina](https://github.com/Vlad-Shcherbina)

## [0.8.2] - 2020-06-11
### Changed
 - Windows `host()` and `hostname()` now behave like they do on Linux and MacOS

### Fixed
 - Windows FFI Undefined Behavior because of not checking for errors
 - Cross-compiling from Linux to Windows not working

## [0.8.1] - 2020-02-22
### Fixed
 - Remove unnecessary use of `to_mut()` on `Cow`s returned from
   `String::from_utf8_lossy()`.

## [0.8.0] - 2020-02-21
### Added
 - Detection for KDE desktop environment.

### Changed
 - Unknown desktop environment may now contain lowercase characters.

### Fixed
 - No longer unwraps in any place where bad data from the OS could cause
   a panic.

## [0.7.0] - 2019-12-21
### Removed
 - `stdweb` dependency when targetting web assembly.

### Changed
 - All public enums now have the attribute `#[non_exhaustive]` and derive
   `Debug`.

### Fixed
 - Some out-of-date documentation

## [0.6.0] - 2019-10-25
### Added
 - Web Assembly support.

### Removed
 - `Platform::Web` variant of enum, use `env()` if you need to.

### Changed
 - `platform()` is no longer a const fn (needed for wasm platform
   detection).

## [0.5.3] - 2019-07-18
### Changed
 - Now uses a more modern Rust coding style (replace `::std::` with `std::`).
 - Now uses a more modern Rust coding style with `mem::MaybeUninit`.
 - `impl Display` for desktop environment now uses proper capitalization.
 - Don't depend on `libc` anymore.
### Fixed
 - Fancy Names not working on Windows
   - `user()` now uses Windows Display Name on Windows rather than the username.
   - `host()` now uses Windows Name DNS Fully Qualified rather than the hostname.

## [0.5.2] - 2019-05-12
### Fixed
 - Fixed more broken links!

## [0.5.1] - 2019-05-12
### Fixed
 - Clippy lint warning: change `expect(&format!("…"))` to `expect("…")` for optimization in 2 cases.
 - Fixed broken links

## [0.5.0] - 2019-03-17
### Added
 - `Platform` enum with corresponding `platform()` function.
 - `Dive`, `Fuchsia`, and `Redox` to `DesktopEnv` enum.
### Changed
 - Started using `const fn` for some functions.

## [0.4.1] - 2019-01-12
### Fixed
 - Fixed README errors.

## [0.4.0] - 2019-01-12
### Added
 - MacOS support.
### Changed
 - `env` on Ubuntu now returns DesktopEnv::Ubuntu instead of DesktopEnv::Other("UBUNTU")
 - Split off the binary into `whome` (who me?) crate

## [0.3.0] - 2019-01-04
### Added
 - Added more fallbacks.
### Changed
 - Rename realname -> user
 - Rename computer -> host
### Fixed
 - Fix typo for uknown -> unknown.

## [0.2.4] - 2018-12-04
### Fixed
 - Works now on platforms that use u8 instead of i8 for chars (like ARM).

## [0.2.3] - 2018-11-26
### Fixed
 - Trailing newline on Windows.

## [0.2.2] - 2018-06-02
### Fixed
 - Typo in markdown.

## [0.2.1] - 2018-06-02
### Fixed
 - Undefined behavior on Linux.

## [0.2.0] - 2017-12-28
### Added
 - Windows support.

## [0.1.1] - 2017-08-04
### Fixed
 - Something in the markdown.

## [0.1.0] - 2017-08-04
### Added
 - Published to crates.io.
