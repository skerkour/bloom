# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 1.2.0 (2020-12-09)
### Added
- `Zeroize` support for x86(_64) SIMD registers ([#577])

### Changed
- Simplify `String::zeroize` ([#563])
- MSRV 1.44+ ([#515])

[#577]: https://github.com/iqlusioninc/crates/pull/577
[#563]: https://github.com/iqlusioninc/crates/pull/563
[#515]: https://github.com/iqlusioninc/crates/pull/515

## 1.1.1 (2020-09-15)

- Add `doc_cfg`([#505])
- zeroize entire capacity of `String`([#352])
- zeroize entire capacity of `Vec` ([#341])

[#505]: https://github.com/iqlusioninc/crates/pull/505
[#352]: https://github.com/iqlusioninc/crates/pull/352
[#341]: https://github.com/iqlusioninc/crates/pull/341

## 1.1.0 (2019-12-02)

- Add `TryZeroize` trait ([#307])
- Add `From<Z: Zeroize>` impl for `Zeroizing<Z>` ([#304])
- Remove `bytes-preview` feature ([#301])

[#307]: https://github.com/iqlusioninc/crates/pull/307
[#304]: https://github.com/iqlusioninc/crates/pull/304
[#301]: https://github.com/iqlusioninc/crates/pull/301

## 1.0.0 (2019-10-13)

- Initial 1.0 release 🎉
- zeroize_derive: Remove legacy `no_drop` attribute support ([#278])
- Rename `bytes` feature to `bytes-preview` ([#277])
- Further relax `Zeroize` trait bounds for `Vec` ([#276])
- Derive `Clone`, `Debug`, and `Eq` for `Zeroizing` ([#275])

[#278]: https://github.com/iqlusioninc/crates/pull/278
[#277]: https://github.com/iqlusioninc/crates/pull/277
[#276]: https://github.com/iqlusioninc/crates/pull/276
[#275]: https://github.com/iqlusioninc/crates/pull/275

## 1.0.0-pre (2019-09-30)

- Loosen `Vec` trait bounds for `Zeroize` ([#267])

[#267]: https://github.com/iqlusioninc/crates/pull/267

## 0.10.1 (2019-09-03)

- (Optionally) Impl `Zeroize` for `Bytes` and `BytesMut` ([#258], [#259])

[#259]: https://github.com/iqlusioninc/crates/pull/259
[#258]: https://github.com/iqlusioninc/crates/pull/258

## 0.10.0 (2019-08-19)

Barring unforeseen circumstances, this release aims to be the last `0.x`
release prior to a `zeroize` 1.0 release.

- Disable `zeroize_derive` Cargo feature by default ([#247])
- Remove `std` feature in favor of `alloc`; MSRV 1.36+ ([#246])
- Deprecate `#[zeroize(no_drop)]` attribute ([#244])
- Use 1.0 `proc-macro2`, `quote`, and `syn` crates ([#242])

[#247]: https://github.com/iqlusioninc/crates/pull/247
[#246]: https://github.com/iqlusioninc/crates/pull/246
[#244]: https://github.com/iqlusioninc/crates/pull/244
[#242]: https://github.com/iqlusioninc/crates/pull/242

## 0.9.3 (2019-07-27)

- Improved attribute parser; fixes nightly build ([#238])

[#238]: https://github.com/iqlusioninc/crates/pull/238

## 0.9.2 (2019-06-28)

- README.md: add Gitter badges; update image links ([#221])

[#221]: https://github.com/iqlusioninc/crates/pull/221

## 0.9.1 (2019-06-04)

- Impl `Zeroize` for `Option<Z: Zeroize>` ([#219])

[#219]: https://github.com/iqlusioninc/crates/pull/219

## 0.9.0 (2019-06-04)

**NOTICE**: This release changes the default behavior of `derive(Zeroize)`
to no longer derive a `Drop` impl. If you wish to derive `Drop`, you must
now explicitly add a `#[zeroize(drop)]` attribute on the type for which you
are deriving `Zeroize`.

- Remove CPU fences ([#216])
- Remove scary language about undefined behavior ([#214])
- Bound blanket array impls on `Zeroize` instead of `DefaultIsZeroes` ([#213])
- Require `zeroize(drop)` or `zeroize(no_drop)` attributes when deriving
  `Zeroize` ([#212]).
- Support stablized 'alloc' crate ([#192])

[#216]: https://github.com/iqlusioninc/crates/pull/216
[#214]: https://github.com/iqlusioninc/crates/pull/214
[#213]: https://github.com/iqlusioninc/crates/pull/213
[#212]: https://github.com/iqlusioninc/crates/pull/212
[#192]: https://github.com/iqlusioninc/crates/pull/192

## 0.8.0 (2019-05-20)

- Impl `Drop` by default when deriving `Zeroize` ([#188])

[#188]: https://github.com/iqlusioninc/crates/pull/188

## 0.7.0 (2019-05-19)

- Use synstructure for custom derive ([#185])
- Add explicit array impls for `DefaultIsZeroes` ([#184])
- Remove `nightly` feature ([#183])
- Add `Zeroizing<Z>` to zeroize values on drop ([#182])

[#185]: https://github.com/iqlusioninc/crates/pull/185
[#184]: https://github.com/iqlusioninc/crates/pull/184
[#183]: https://github.com/iqlusioninc/crates/pull/183
[#182]: https://github.com/iqlusioninc/crates/pull/182

## 0.6.0 (2019-03-23)

- Add ZeroizeOnDrop marker trait + custom derive ([#168])
- Custom derive support for `Zeroize` ([#167])
- Rename `ZeroizeWithDefault` to `DefaultIsZeroes` ([#166])

[#168]: https://github.com/iqlusioninc/crates/pull/168
[#167]: https://github.com/iqlusioninc/crates/pull/167
[#166]: https://github.com/iqlusioninc/crates/pull/166

## 0.5.2 (2018-12-25)

- Add `debug_assert!` to ensure string interiors are zeroized ([#156])

[#156]: https://github.com/iqlusioninc/crates/pull/156

## 0.5.1 (2018-12-24)

- Avoid re-exporting the whole prelude ([#150])

[#150]: https://github.com/iqlusioninc/crates/pull/150

## 0.5.0 (2018-12-24)

This release is a rewrite which replaces FFI bindings to OS-specific APIs with
a pure Rust solution.

- Use `core::sync::atomic` fences ([#146])
- Test wasm target ([#143])
- Rewrite using `core::ptr::write_volatile` ([#142])

[#146]: https://github.com/iqlusioninc/crates/pull/146
[#143]: https://github.com/iqlusioninc/crates/pull/143
[#142]: https://github.com/iqlusioninc/crates/pull/142

## 0.4.2 (2018-10-12)

- Fix ldd scraper for older glibc versions ([#134])

[#134]: https://github.com/iqlusioninc/crates/pull/134

## 0.4.1 (2018-10-12)

- Support musl-libc ([#131])

[#131]: https://github.com/iqlusioninc/crates/pull/131
  
## 0.4.0 (2018-10-12)

- Impl `Zeroize` trait on concrete types ([#108])

[#108]: https://github.com/iqlusioninc/crates/pull/108

## 0.3.0 (2018-10-11)

- Replace `secure_zero_memory` with `Zeroize` ([#104])

[#104]: https://github.com/iqlusioninc/crates/pull/104

## 0.2.0 (2018-10-11)

- Add `Zeroize` trait ([#101])

[#101]: https://github.com/iqlusioninc/crates/pull/101

## 0.1.2 (2018-10-03)

- README.md: Fix intrinsic links ([#86])

[#86]: https://github.com/iqlusioninc/crates/pull/86

## 0.1.1 (2018-10-03)

- Documentation improvements ([#83])

[#83]: https://github.com/iqlusioninc/crates/pull/83

## 0.1.0 (2018-10-03)

- Initial release
