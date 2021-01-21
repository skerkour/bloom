## 1.3.0 (2021-1-8)

* Remove _log_ dependency, which was only used sparsely as a poor workaround
  for unspecific `Error::Header` errors. ([#25])

* Upgrade to _bytes_ 1.0.0 (paolobarbolini [#31])

* Lift prior constraint on _http_ dependency (optimistically hoping for no
  further 0.2.z anomalous releases).

[#25]: https://github.com/dekellum/hyperx/pull/25
[#31]: https://github.com/dekellum/hyperx/pull/31

## 1.2.1 (2021-1-7)

* Constrain _http_ dependency to <0.2.3 for remaining 1.2 series due to dubious
  release practices, e.g. forcing duplicates in patch releases. This will be
  lifted in 1.3.0, see this [github comment][461].

[461]: https://github.com/hyperium/http/pull/461#issuecomment-756298944

## 1.2.0 (2020-10-6)

* Replace use of time crate with httpdate crate for date/time typed
  headers (paolobarbolini #24)

* Broaden _base64_ dependency to include 0.13.0 (dvermd #23)

## 1.1.0 (2020-8-29)

* Fix various compile warnings with rustc 1.44 and later.

* Broaden _base64_ dependency to include 0.12.0 (paolobarbolini #20)

## 1.0.0 (2020-1-3)

* The default _compat_ feature is no longer optional, as its unlikely that
  anyone would be using recent versions without this feature. The feature gate
  name is retained for now, but has no effect.

* Place the legacy `Headers` struct under a new non-default _headers_ feature
  gate. Note that use of this type is no longer required nor recommended for
  parsing and serialization of typed headers. See the rewritten typed header
  doc examples.  Consider replacing with the _http_ crate `HeaderMap` and the
  `TypedHeaders` extension trait introduced here in 0.15.0. (#18)

* Upgrade to _http_ 0.2.0 (\w API changes) and _bytes_ 0.5.2 (MSRV 1.39.0)

* Upgrade (unconstrain) _cfg-if_ dependency to 0.1.10 (MSRV 1.31.0)

* Upgrade to _unicase_ 2.6.0

* Upgrade to _percent-encoding_ 2.1.0 (\w API changes, MSRV 1.33.0) (#15)

* Upgrade to _time_ 0.1.39 to avoid minimal version build failure

* Broaden _base64_ dependency to include 0.11.0 (MSRV 1.34.0)

* MSRV is now 1.39.0, based on above upgrades.

## 0.15.2 (2019-10-1)

* Constrain transitive _cfg-if_ dependency to <0.1.10 to preserve MSRV 1.27.2.

* Narrow various other dependencies for future reliability.  We may
  subsequently make PATCH releases which _broaden_ private or public
  dependencies to include new releases found compatible.

## 0.15.1 (2019-6-3)

* Fix build.rs for `rustc --version` not including git metadata (alyssais #14)

## 0.15.0 (2019-5-8)

* Add a `TypedHeaders` extension trait providing more convenient generic
  encode/decode methods to `http::HeaderMap` for _hyperx_ typed headers,
  implemented using a new `StandardHeader` trait and `standard_header!` macro,
  with an associate function for the `HeaderName` constants of the _http_
  crate. (#13)

* Add reference based `impl From<&'a Headers> for http::HeaderMap` for symmetry
  and performance, e.g. avoiding a `clone`. (#13)

* Increase MSRV to 1.27.2, which enables us to revert a CI workaround for the
  fact that base64 0.10.1 was released with this same MSRV. (#10 #12)

* Add a build.rs to check MSRV and fail fast with a clear error when older
  rustc versions are used. (#12)

## 0.14.0 (2019-1-4)

* Update the signature of `Header::parse_header` to be generic over types
  implementing a new `RawLike` trait, which includes the existing local `Raw`
  type as well as _http_ crate types `HeaderValue` and (`HeaderMap::get_all`)
  `GetAll`. This avoids an allocation when directly parsing from these later
  types.

  _Expected Breakage_: Any 3rd-party custom headers directly implementing
  `parse_header` will need to change accordingly on upgrade. Also `Into`
  conversions to `Raw` now frequently need to be type annotated. (#8)

* Improve header module rustdoc, including with parsing usage for the above.

## 0.13.2 (2019-1-2)

* Remove un-exported, and unused as of 0.13.1, `uri` module and related code.

* Broaden base64 dependency to include 0.10.0, passing tests.

* Silence a deprecation warning for `str::trim_right_matches` until the minimum
  rust version is updated to 1.30.0.

## 0.13.1 (2018-6-26)

* Remove `error::UriError` re-export and `error::Canceled` which are unused
  internally and where not exported from this crate. (#5)

## 0.13.0 (2018-6-18)

* Remove variants from `hyperx::Error` which are unused by the header
  module. Exhaustive matching has been discouraged for this enum, but this is
  still a potential breaking change. (dekellum #2)

* Add an alternative, by reference `From<&http::HeaderMap>` for `Headers`.
  (DarrenTsung #3)

## 0.12.0 (2018-6-1)

Forked from hyper 0.11.27, e*x*tracting the typed *header* module
from [hyperium/hyper@76fdbcf2], 0.11.x branch, preserved here as
[76fdbcf2].

## Prior Releases

See [hyper's CHANGELOG] for prior updates pertaining to the headers
sub-module.

[hyper's CHANGELOG]: https://github.com/hyperium/hyper/blob/0.11.x/CHANGELOG.md
[hyperium/hyper@76fdbcf2]: https://github.com/hyperium/hyper/commit/76fdbcf2
[76fdbcf2]: https://github.com/dekellum/hyperx/commit/76fdbcf23cd35cebb03bf4c0e3025b671578bd75
