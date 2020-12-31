# Changes

## Unreleased


## 2.0.0-alpha.4 - 2020-08-17

### Changed

* Update `rustls` dependency to 0.18
* Update `tokio-rustls` dependency to 0.14


## [2.0.0-alpha.3] - 2020-05-08

### Fixed

* Corrected spelling of `ConnectError::Unresolverd` to `ConnectError::Unresolved`

## [2.0.0-alpha.2] - 2020-03-08

### Changed

* Update `trust-dns-proto` dependency to 0.19. [#116]
* Update `trust-dns-resolver` dependency to 0.19. [#116]
* `Address` trait is now required to have static lifetime. [#116]
* `start_resolver` and `start_default_resolver` are now `async` and may return a `ConnectError`. [#116]

[#116]: https://github.com/actix/actix-net/pull/116

## [2.0.0-alpha.1] - 2020-03-03

### Changed

* Update `rustls` dependency to 0.17
* Update `tokio-rustls` dependency to 0.13

## [1.0.2] - 2020-01-15

* Fix actix-service 1.0.3 compatibility

## [1.0.1] - 2019-12-15

* Fix trust-dns-resolver compilation

## [1.0.0] - 2019-12-11

* Release

## [1.0.0-alpha.3] - 2019-12-07

### Changed

* Migrate to tokio 0.2


## [1.0.0-alpha.2] - 2019-12-02

### Changed

* Migrated to `std::future`


## [0.3.0] - 2019-10-03

### Changed

* Update `rustls` to 0.16
* Minimum required Rust version upped to 1.37.0

## [0.2.5] - 2019-09-05

* Add `TcpConnectService`

## [0.2.4] - 2019-09-02

* Use arbiter's storage for default async resolver

## [0.2.3] - 2019-08-05

* Add `ConnectService` and `OpensslConnectService`

## [0.2.2] - 2019-07-24

* Add `rustls` support

## [0.2.1] - 2019-07-17

### Added

* Expose Connect addrs #30

### Changed

* Update `derive_more` to 0.15


## [0.2.0] - 2019-05-12

### Changed

* Upgrade to actix-service 0.4


## [0.1.5] - 2019-04-19

### Added

* `Connect::set_addr()`

### Changed

* Use trust-dns-resolver 0.11.0


## [0.1.4] - 2019-04-12

### Changed

* Do not start default resolver immediately for default connector.


## [0.1.3] - 2019-04-11

### Changed

* Start trust-dns default resolver on first use

## [0.1.2] - 2019-04-04

### Added

* Log error if dns system config could not be loaded.

### Changed

* Rename connect Connector to TcpConnector #10


## [0.1.1] - 2019-03-15

### Fixed

* Fix error handling for single address


## [0.1.0] - 2019-03-14

* Refactor resolver and connector services

* Rename crate
