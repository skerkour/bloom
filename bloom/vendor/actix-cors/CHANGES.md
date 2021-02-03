# Changes

## Unreleased - 2021-xx-xx



## 0.5.4 - 2020-12-31
* Fix `expose_any_header` method, now set the correct field. [#143]

[#143]: https://github.com/actix/actix-extras/pull/143


## 0.5.3 - 2020-11-19
* Fix version spec for `derive_more` dependency.


## 0.5.2 - 2020-11-15
* Ensure `tinyvec` is using the correct features.
* Bump `futures-util` minimum version to `0.3.7` to avoid `RUSTSEC-2020-0059`.


## 0.5.1 - 2020-11-05
* Fix `allow_any_header` method, now set the correct field. [#121]

[#121]: https://github.com/actix/actix-extras/pull/121


## 0.5.0 - 2020-10-19
* Disallow `*` in `Cors::allowed_origin`. [#114].
* Hide `CorsMiddleware` from docs. [#118].
* `CorsFactory` is removed. [#119]
* The `impl Default` constructor is now overly-restrictive. [#119]
* Added `Cors::permissive()` constructor that allows anything. [#119]
* Adds methods for each property to reset to a permissive state. (`allow_any_origin`,
  `expose_any_header`, etc.) [#119]
* Errors are now propagated with `Transform::InitError` instead of panicking. [#119]
* Fixes bug where allowed origin functions are not called if `allowed_origins` is All. [#119]
* `AllOrSome` is no longer public. [#119]
* Functions used for `allowed_origin_fn` now receive the Origin HeaderValue as the
  first parameter. [#120]

[#114]: https://github.com/actix/actix-extras/pull/114
[#118]: https://github.com/actix/actix-extras/pull/118
[#119]: https://github.com/actix/actix-extras/pull/119
[#120]: https://github.com/actix/actix-extras/pull/120


## 0.4.1 - 2020-10-07
* Allow closures to be used with `allowed_origin_fn`. [#110]

[#110]: https://github.com/actix/actix-extras/pull/110


## 0.4.0 - 2020-09-27
* Implement `allowed_origin_fn` builder method. [#93]
* Use `TryInto` instead of `TryFrom` where applicable. [#106]

[#93]: https://github.com/actix/actix-extras/pull/93
[#106]: https://github.com/actix/actix-extras/pull/106


## 0.3.0 - 2020-09-11
* Update `actix-web` dependency to 3.0.0.
* Minimum supported Rust version (MSRV) is now 1.42.0.
* Implement the Debug trait on all public types.


## 0.3.0-alpha.1 - 2020-03-11
* Minimize `futures-*` dependencies
* Update `actix-web` dependency to 3.0.0-alpha.1


## 0.2.0 - 2019-12-20
* Release


## 0.2.0-alpha.3 - 2019-12-07
* Migrate to actix-web 2.0.0
* Bump `derive_more` crate version to 0.99.0


## 0.1.0 - 2019-06-15
* Move cors middleware to separate crate
