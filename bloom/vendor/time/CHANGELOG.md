# Changelog

All notable changes to the time project will be documented in this file.

The format is based on [Keep a Changelog]. This project adheres to [Semantic
Versioning].

---

## 0.2.25 [2021-01-24]

### Fixed

- Fix #309, which can cause panics in certain situations.

## 0.2.24 [2021-01-08]

### Fixed

- The implementation of `OffsetDateTime::timestamp`, `OffsetDateTime::unix_timestamp`,
  `PrimitiveDatetime::timestamp`, and `OffsetDateTime::unix_timestamp` have been corrected. This
  affects all negative timestamps with a nonzero subsecond value.

## 0.2.23 [2020-11-17]

## Compatibility notes

Due to #293, any method that requires knowledge of the local offset will now
_fail_ on Linux. For `try_` methods, this means returning an error. For others,
it means assuming UTC.

### Deprecated

- `UtcOffset::timestamp` (moved to `UtcOffset::unix_timestamp`)
- `UtcOffset::timestamp_nanos` (moved to `UtcOffset::unix_timestamp_nanos`)
- `date` (moved to `macros::date`)
- `time` (moved to `macros::time`)
- `offset` (moved to `macros::offset`)
- `OffsetDateTime::now_local` (assumes UTC if unable to be determined)
- `UtcOffset::local_offset_at` (assumes UTC if unable to be determined)
- `UtcOffset::current_local_offset` (assumes UTC if unable to be determined)

## 0.2.22 [2020-09-25]

### Fixed

- Solaris & Illumos now successfully build.
- `Duration::new` could previously result in an inconsistent internal state.
  This led to some odd situations where a `Duration` could be both positive and
  negative. This has been fixed such that the internal state maintains its
  invariants.

## 0.2.21 [2020-09-20]

### Changed

- Implementation details of some error types have been exposed. This means that
  data about a component being out of range can be directly obtained, while an
  invalid offset or conversion error is guaranteed to be a zero-sized type.
- The following functions are `const fn` on rustc ≥ 1.46:
  - `Date::try_from_iso_ywd`
  - `Date::iso_year_week`
  - `Date::week`
  - `Date::sunday_based_week`
  - `Date::monday_based_week`
  - `Date::try_with_hms`
  - `Date::try_with_hms_milli`
  - `Date::try_with_hms_micro`
  - `Date::try_with_hms_nano`
  - `PrimitiveDateTime::iso_year_week`
  - `PrimitiveDateTime::week`
  - `PrimitiveDateTime::sunday_based_week`
  - `PrimitiveDateTime::monday_based_week`
  - `util::weeks_in_year`

## 0.2.20 [2020-09-16]

### Added

- `OffsetDateTime::timestamp_nanos`
- `OffsetDateTime::from_unix_timestamp_nanos`

### Fixed

A bug with far-reaching consequences has been fixed. See #276 for complete
details, but the gist is that the constructing a `Date` from a valid Julian day
may result in an invalid value or even panic. As a consequence of implementation
details, this affects nearly all arithmetic with `Date`s (and as a result also
`PrimitiveDateTime`s and `OffsetDateTime`s).

### Improvements

- Document how to construct an `OffsetDateTime` from a timestamp-nanosecond
  pair

## 0.2.19 [2020-09-12]

### Fixed

- The build script now declares a dependency on the `COMPILING_UNDER_CARGO_WEB`
  environment variable.
- Parsing the `%D` specifier no longer requires padding on the month.
  Previously, `Err(InvalidMonth)` was incorrectly returned.
- A `std::time::Duration` that is larger than `time::Duration::max_value()` now
  correctly returns `Ordering::Greater` when compared.
- Multiplying and assigning an integer by `Sign::Zero` now sets the integer to
  be zero. This previously left the integer unmodified.

## 0.2.18 [2020-09-08]

### Changed

- The following functions are `const fn` on rustc ≥ 1.46:
  - `Date::try_from_ymd`
  - `Date::try_from_yo`
  - `Time::try_from_hms`
  - `Time::try_from_hms_milli`
  - `Time::try_from_hms_micro`
  - `Time::try_from_hms_nano`
- An `error` module has been created where all existing error types are
  contained. The `Error` suffix has been dropped from these types.
- An `ext` module has been created where extension traits are contained.
- A `util` module has been created where utility functions are contained.
- `error::ComponentRange` now implements `Copy`.

For back-compatibility, all items that were moved to newly-contained modules
have been re-exported from their previous locations (and in the case of the
`error` module, with their previous name).

### Fixes

Parsing `format::Rfc3339` now correctly handles the UTC offset (#274).

## 0.2.17 [2020-09-01]

### Changed

The following functions are `const fn` on rustc ≥ 1.46:

- `Date::year`
- `Date::month`
- `Date::day`
- `Date::month_day`
- `Date::ordinal`
- `Date::as_ymd`
- `Date::as_yo`
- `Date::julian_day`
- `Duration::checked_div`
- `PrimitiveDateTime::year`
- `PrimitiveDateTime::month`
- `PrimitiveDateTime::day`
- `PrimitiveDateTime::month_day`
- `PrimitiveDateTime::ordinal`
- `Weekday::previous`
- `Weekday::next`

### Improvements

- `size_of::<Date>()` has been reduced from 8 to 4. As a consequence,
  `size_of::<PrimitiveDatetime>()` went from 16 to 12 and
  `size_of::<OffsetDateTime>()` from 20 to 16. This change also results in a
  performance improvement of approximately 30% on the `Date::year` and
  `Date::ordinal` methods.
- `cfg-if` has been removed as a dependency.

### Fixed

- `cfg` flags passed to rustc will no longer collide with other crates (at least
  unless they're doing something very stupid).
- The crate will successfully compile with any combination of feature flags.
  Previously, some combinations would fail.

## 0.2.16 [2020-05-12]

### Added

`OffsetDateTime`s can now be represented as Unix timestamps with serde. To do
this, you can use the `time::serde::timestamp` and
`time::serde::timestamp::option` modules.

## 0.2.15 [2020-05-04]

### Fixed

`cargo-web` support works, and is now explicitly checked in CI. A previous
change was made that made a method call ambiguous.

## 0.2.14 [2020-05-02]

### Fixed

Adding/subtracting a `core::time::Duration` now correctly takes subsecond
values into account. This also affects `PrimitiveDateTime` and `OffsetDateTime`.

## 0.2.13 [2020-05-01]

### Fixed

Panicking APIs are re-exposed.

## 0.2.12 [2020-04-30]

### Fixed

Subtracting `Instant`s can correctly result in a negative duration, rather than
resulting in the absolute value of it.

## 0.2.11 [2020-04-27]

### Added

- `OffsetDateTime::now_utc`

### Deprecated

- `OffsetDateTime::now` due to the offset not being clear from the method name
  alone.

### Fixed

`Date`s are now uniformly random when using the `rand` crate. Previously, both
the year and day within the year were uniform, but this meant that any given day
in a leap year was slightly less likely to be chosen than a day in a non-leap
year.

### Changed

- MSRV is lowered to 1.32.0.

## 0.2.10 [2020-04-19]

### Added

- Support for formatting and parsing `OffsetDateTime`s as RFC3339.
- Lazy formatting. To avoid exposing implementation details, we're just
  returning `impl Display`, rather than a concrete type.
- Add support for Illumos.

### Fixed

- Deprecated APIs from time v0.1 are public again. They were previously hidden
  by accident in 0.2.9.

## 0.2.9 [2020-03-13]

### Fixed

`cfg-if` now has a mandatory minimum of 0.1.10, rather than just 0.1. This is
because compilation fails when using 0.1.9.

## 0.2.8 [2020-03-12]

### Added

- `cargo_web` support has been added for getting a local offset. A general
  catch-all defaulting to UTC has also been added.
- `Error::source` has been implemented for the wrapper `time::Error`.
- `UtcOffset::try_local_offset`, `UtcOffset::try_current_local_offset`,
  `OffsetDateTime::try_now_local()` provide fallible alternatives when the
  default of UTC is not desired. To facilitate this change,
  `IndeterminateOffsetError` has been added.
- Support for parsing and formatting subsecond nanoseconds.

### Changed

- `#[non_exhaustive]` is simulated on compilers prior to 1.40.0.

## 0.2.7 [2020-02-22]

### Added

- `Display` has been implemented for `Date`, `OffsetDateTime`,
  `PrimitiveDateTime`, `Time`, `UtcOffset`, and `Weekday`.
- `Hash` is now derived for `Duration`.
- `SystemTime` can be converted to and from `OffsetDateTime`. The following
  trait implementations have been made for interoperability:
  - `impl Sub<SystemTime> for OffsetDateTime`
  - `impl Sub<OffsetDateTime> for SystemTime`
  - `impl PartialEq<SystemTime> for OffsetDateTime`
  - `impl PartialEq<OffsetDateTime> for SystemTime`
  - `impl PartialOrd<SystemTime> for OffsetDateTime`
  - `impl PartialOrd<OffsetDateTime> for SystemTime`
  - `impl From<SystemTime> for OffsetDateTime`
  - `impl From<OffsetDateTime> for SystemTime`
- All structs now `impl Duration<T> for Standard`, allowing usage with the
  `rand` crate. This is gated behind the `rand` feature flag.

- Documentation can now be built on stable. Some annotations will be missing if
  you do this.
- `NumericalDuration` has been implemented for `f32` and `f64`.
  `NumericalStdDuration` and `NumericalStdDurationShort` have been implemented
  for `f64` only.
- `UtcOffset::local_offset_at(OffsetDateTime)`, which will obtain the system's
  local offset at the provided moment in time.
  - `OffsetDateTime::now_local()` is equivalent to calling
    `OffsetDateTime::now().to_offset(UtcOffset::local_offset_at(OffsetDateTime::now()))`
    (but more efficient).
  - `UtcOffset::current_local_offset()` will return the equivalent of
    `OffsetDateTime::now_local().offset()`.

### Changed

- All formatting and parsing methods now accept `impl AsRef<str>` as parameters,
  rather than just `&str`. `time::validate_format_string` does this as well.
- The requirement of a `Date` being between the years -100,000 and +100,000
  (inclusive) is now strictly enforced.
- Overflow checks for `Duration` are now enabled by default. This behavior is
  the identical to what the standard library does.
- The `time`, `date`, and `offset` macros have been added to the prelude.

### Deprecated

- `Sign` has been deprecated in its entirety, along with `Duration::sign`.

  To obtain the sign of a `Duration`, you can use the `Sign::is_positive`,
  `Sign::is_negative`, and `Sign::is_zero` methods.

- A number of functions and trait implementations that implicitly assumed a
  timezone (generally UTC) have been deprecated. These are:
  - `Date::today`
  - `Time::now`
  - `PrimitiveDateTime::now`
  - `PrimitiveDateTime::unix_epoch`
  - `PrimitiveDateTime::from_unix_timestamp`
  - `PrimitiveDateTime::timestamp`
  - `impl Sub<SystemTime> for PrimitiveDateTime`
  - `impl Sub<PrimitiveDateTime> for SystemTime`
  - `impl PartialEq<SystemTime> for PrimitiveDateTime`
  - `impl PartialEq<PrimitiveDateTime> for SystemTime>`
  - `impl PartialOrd<SystemTime> for PrimitiveDateTime`
  - `impl PartialOrd<PrimitiveDateTime> for SystemTime>`
  - `impl From<SystemTime> for PrimitiveDateTime`
  - `impl From<PrimitiveDateTime> for SystemTime`

### Fixed

- Avoid panics when parsing an empty string (#215).
- The nanoseconds component of a `Duration` is now always in range. Previously,
  it was possible (via addition and/or subtraction) to obtain a value that was
  not internally consistent.
- `Time::parse` erroneously returned an `InvalidMinute` error when it was
  actually the second that was invalid.
- `Date::parse("0000-01-01", "%Y-%m-%d")` incorrectly returned an `Err` (#221).

## Pre-0.2.7

Prior to v0.2.7, changes were listed in GitHub releases.

[keep a changelog]: https://keepachangelog.com/en/1.0.0/
[semantic versioning]: https://semver.org/spec/v2.0.0.html
