# `bytemuck` changelog

## 1.5

* Added `pod_collect_to_vec`, which will gather a slice into a vec,
allowing you to change the pod type while also safely ignoring alignment.
[PR 50](https://github.com/Lokathor/bytemuck/pull/50)

## 1.4.2

* [Kimundi](https://github.com/Kimundi) fixed an issue that could make `try_zeroed_box`
stack overflow for large values at low optimization levels.
[PR 43](https://github.com/Lokathor/bytemuck/pull/43)

## 1.4.1

* [thomcc](https://github.com/thomcc) fixed up the CI and patched over a soundness hole in `offset_of!`.
[PR 38](https://github.com/Lokathor/bytemuck/pull/38)

## 1.4

* [icewind1991](https://github.com/icewind1991) has contributed the proc-macros
  for deriving impls of `Pod`, `TransparentWrapper`, `Zeroable`!! Everyone has
  been waiting for this one folks! It's a big deal. Just enable the `derive`
  cargo feature and then you'll be able to derive the traits on your types. It
  generates all the appropriate tests for you.
* The `zeroable_maybe_uninit` feature now adds a `Zeroable` impl to the
  `MaybeUninit` type. This is only behind a feature flag because `MaybeUninit`
  didn't exist back in `1.34.0` (the minimum rust version of `bytemuck`).

## 1.3.1

* The entire crate is now available under the `Apache-2.0 OR MIT` license as
  well as the previous `Zlib` license
  [#24](https://github.com/Lokathor/bytemuck/pull/24).
* [HeroicKatora](https://github.com/HeroicKatora) added the
  `try_zeroed_slice_box` function
  [#10](https://github.com/Lokathor/bytemuck/pull/17). `zeroed_slice_box` is
  also available.
* The `offset_of!` macro now supports a 2-arg version. For types that impl
  Default, it'll just make an instance using `default` and then call over to the
  3-arg version.
* The `PodCastError` type now supports `Hash` and `Display`. Also if you enable
  the `extern_crate_std` feature then it will support `std::error::Error`.
* We now provide a `TransparentWrapper<T>` impl for `core::num::Wrapper<T>`.
* The error type of `try_from_bytes` and `try_from_bytes_mut` when the input
  isn't aligned has been corrected from being `AlignmentMismatch` (intended for
  allocation casting only) to `TargetAlignmentGreaterAndInputNotAligned`.

## 1.3.0

* Had a bug because the CI was messed up! It wasn't soundness related, because
  it prevented the crate from building entirely if the `extern_crate_alloc`
  feature was used. Still, this is yanked, sorry.

## 1.2.0

* [thomcc](https://github.com/thomcc) added many things:
  * A fully sound `offset_of!` macro
    [#10](https://github.com/Lokathor/bytemuck/pull/10)
  * A `Contiguous` trait for when you've got enums with declared values
    all in a row [#12](https://github.com/Lokathor/bytemuck/pull/12)
  * A `TransparentWrapper` marker trait for when you want to more clearly
    enable adding and removing a wrapper struct to its inner value
    [#15](https://github.com/Lokathor/bytemuck/pull/15)
  * Now MIRI is run on CI in every single push!
    [#16](https://github.com/Lokathor/bytemuck/pull/16)

## 1.1.0

* [SimonSapin](https://github.com/SimonSapin) added `from_bytes`,
  `from_bytes_mut`, `try_from_bytes`, and `try_from_bytes_mut` ([PR
  Link](https://github.com/Lokathor/bytemuck/pull/8))

## 1.0.1

* Changed to the [zlib](https://opensource.org/licenses/Zlib) license.
* Added much more proper documentation.
* Reduced the minimum Rust version to 1.34
