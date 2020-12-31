# Changes

## Unreleased - 2020-xx-xx

## 0.3.0 - 2020-08-23
* No changes from beta 2.

## 0.3.0-beta.2 - 2020-08-19
* Remove unused type parameter from `Framed::replace_codec`.

## 0.3.0-beta.1 - 2020-08-19
* Use `.advance()` instead of `.split_to()`.
* Upgrade `tokio-util` to `0.3`.
* Improve `BytesCodec` `.encode()` performance
* Simplify `BytesCodec` `.decode()` 
* Rename methods on `Framed` to better describe their use.
* Add method on `Framed` to get a pinned reference to the underlying I/O.
* Add method on `Framed` check emptiness of read buffer.

## [0.2.0] - 2019-12-10

* Use specific futures dependencies

## [0.2.0-alpha.4]

* Fix buffer remaining capacity calculation

## [0.2.0-alpha.3]

* Use tokio 0.2

* Fix low/high watermark for write/read buffers

## [0.2.0-alpha.2]

* Migrated to `std::future`

## [0.1.2] - 2019-03-27

* Added `Framed::map_io()` method.

## [0.1.1] - 2019-03-06

* Added `FramedParts::with_read_buffer()` method.

## [0.1.0] - 2018-12-09

* Move codec to separate crate
