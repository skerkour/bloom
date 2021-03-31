# Changes

## Unreleased - 2021-xx-xx


## 0.2.7 - 2021-02-06
* Add `Router::recognize_checked` [#247]

[#247]: https://github.com/actix/actix-net/pull/247


## 0.2.6 - 2021-01-09
* Use `bytestring` version range compatible with Bytes v1.0. [#246]

[#246]: https://github.com/actix/actix-net/pull/246


## 0.2.5 - 2020-09-20
* Fix `from_hex()` method


## 0.2.4 - 2019-12-31
* Add `ResourceDef::resource_path_named()` path generation method


## 0.2.3 - 2019-12-25
* Add impl `IntoPattern` for `&String`


## 0.2.2 - 2019-12-25
* Use `IntoPattern` for `RouterBuilder::path()`


## 0.2.1 - 2019-12-25
* Add `IntoPattern` trait
* Add multi-pattern resources


## 0.2.0 - 2019-12-07
* Update http to 0.2
* Update regex to 1.3
* Use bytestring instead of string


## 0.1.5 - 2019-05-15
* Remove debug prints


## 0.1.4 - 2019-05-15
* Fix checked resource match


## 0.1.3 - 2019-04-22
* Added support for `remainder match` (i.e "/path/{tail}*")


## 0.1.2 - 2019-04-07
* Export `Quoter` type
* Allow to reset `Path` instance


## 0.1.1 - 2019-04-03
* Get dynamic segment by name instead of iterator.


## 0.1.0 - 2019-03-09
* Initial release
