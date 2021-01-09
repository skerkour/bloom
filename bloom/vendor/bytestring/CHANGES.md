# Changes

## Unreleased - 2020-xx-xx


## 1.0.0 - 2020-12-31
* Update `bytes` dependency to `1`.
* Add array and slice of `u8` impls of `TryFrom` up to 32 in length.
* Rename `get_ref` to `as_bytes` and rename `into_inner` to `into_bytes`.
* `ByteString::new` is now a `const fn`.
* Crate is now `#[no_std]` compatible.


## 0.1.5 - 2020-03-30
* Serde support


## 0.1.4 - 2020-01-14
* Fix `AsRef<str>` impl


## 0.1.3 - 2020-01-13
* Add `PartialEq<T: AsRef<str>>`, `AsRef<[u8]>` impls


## 0.1.2 - 2019-12-22
* Fix `new()` method
* Make `ByteString::from_static()` and `ByteString::from_bytes_unchecked()` methods const.


## 0.1.1 - 2019-12-07
* Fix hash impl


## 0.1.0 - 2019-12-07
* Initial release
