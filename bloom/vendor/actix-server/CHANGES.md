# Changes

## Unreleased - 2020-xx-xx


## 1.0.4 - 2020-09-12
* Update actix-codec to 0.3.0.
* Workers must be greater than 0. [#167]

[#167]: https://github.com/actix/actix-net/pull/167


## 1.0.3 - 2020-05-19
* Replace deprecated `net2` crate with `socket2` [#140]

[#140]: https://github.com/actix/actix-net/pull/140


## 1.0.2 - 2020-02-26
* Avoid error by calling `reregister()` on Windows [#103]

[#103]: https://github.com/actix/actix-net/pull/103


## 1.0.1 - 2019-12-29
* Rename `.start()` method to `.run()`


## 1.0.0 - 2019-12-11
* Use actix-net releases


## 1.0.0-alpha.4 - 2019-12-08
* Use actix-service 1.0.0-alpha.4


## 1.0.0-alpha.3 - 2019-12-07
* Migrate to tokio 0.2
* Fix compilation on non-unix platforms
* Better handling server configuration


## 1.0.0-alpha.2 - 2019-12-02
* Simplify server service (remove actix-server-config)
* Allow to wait on `Server` until server stops


## 0.8.0-alpha.1 - 2019-11-22
* Migrate to `std::future`


## 0.7.0 - 2019-10-04
* Update `rustls` to 0.16
* Minimum required Rust version upped to 1.37.0


## 0.6.1 - 2019-09-25
* Add UDS listening support to `ServerBuilder`


## 0.6.0 - 2019-07-18
* Support Unix domain sockets #3


## 0.5.1 - 2019-05-18
* ServerBuilder::shutdown_timeout() accepts u64


## 0.5.0 - 2019-05-12
* Add `Debug` impl for `SslError`
* Derive debug for `Server` and `ServerCommand`
* Upgrade to actix-service 0.4


## 0.4.3 - 2019-04-16
* Re-export `IoStream` trait
* Depend on `ssl` and `rust-tls` features from actix-server-config


## 0.4.2 - 2019-03-30
* Fix SIGINT force shutdown


## 0.4.1 - 2019-03-14
* `SystemRuntime::on_start()` - allow to run future before server service initialization


## 0.4.0 - 2019-03-12
* Use `ServerConfig` for service factory
* Wrap tcp socket to `Io` type
* Upgrade actix-service


## 0.3.1 - 2019-03-04
* Add `ServerBuilder::maxconnrate` sets the maximum per-worker number of concurrent connections
* Add helper ssl error `SslError`
* Rename `StreamServiceFactory` to `ServiceFactory`
* Deprecate `StreamServiceFactory`


## 0.3.0 - 2019-03-02
* Use new `NewService` trait


## 0.2.1 - 2019-02-09
* Drop service response


## 0.2.0 - 2019-02-01
* Migrate to actix-service 0.2
* Updated rustls dependency


## 0.1.3 - 2018-12-21
* Fix max concurrent connections handling


## 0.1.2 - 2018-12-12
* rename ServiceConfig::rt() to ServiceConfig::apply()
* Fix back-pressure for concurrent ssl handshakes


## 0.1.1 - 2018-12-11
* Fix signal handling on windows


## 0.1.0 - 2018-12-09
* Move server to separate crate
