# Version 0.6.1

New features:
* Support for reading `u16` and ascii string tags.
* Added `Limits::unlimited` for disabling all limits.
* Added `ImageEncoder::rows_per_strip` to overwrite the default.

Changes:
* The default strip size for chunked encoding is now 1MB, up from 8KB. This
  should lead to more efficient decoding and compression.

Fixes:
* Fixed a bug where LZW compressed strips could not be decoded, instead
  returning an error `Inconsistent sizes encountered`.
* Reading a tag with a complex type and a single value returns the proper Value
  variant, instead of a vector with one entry.

# Version 0.6.0

New features:
* Support for decoding BigTIFF with 64-bit offsets
* The value types byte, `f32`, `f64` are now recognized
* Support for Modern JPEG encoded images

Improvements:
* Better support for adding auxiliary tags before encoding image data
* Switched to lzw decoder library `weezl` for performance
* The `ColorType` trait now supports `SAMPLE_ENCODING` hints

Fixes:
* Fixed decoding of inline ASCII in tags
* Fixed handling after null terminator in ASCII data
* Recognize tile and sample format tags

# Version 0.5.0

* Added support for 32-bit and 64-bit decoded values.
* Added CMYK(16|32|64) color type support.
* Check many internal integer conversions to increase stability. This should
  only lead to images being reported as faulty that would previously silently
  break platform limits. If there are any false positives, please report them.
* Remove an erroneous check of decoded length in lzw compressed images.

# Version 0.4.0

* Several enumerations are now non_exhaustive for future extensions.
  These are `Tag`, `Type`, `Value`, `PhotometricInterpretation`,
  `CompressionMethod`, `Predictor`.
* Enums gained a dedicated method to convert to their TIFF variant value with
  the specified type. Performing these conversions by casting the discriminant
  with `as` is not guaranteed to be stable, except where documented explicitly.
* Removed the num-derive and num dependencies.
* Added support for decoding `deflate` compressed images.
* Make the decoder `Limits` customizable by exposing members.
* Fixed multi-page TIFF encoding writing incorrect offsets.
