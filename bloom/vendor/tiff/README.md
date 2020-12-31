# image-tiff
[![Build Status](https://travis-ci.org/image-rs/image-tiff.svg?branch=master)](https://travis-ci.org/image-rs/image-tiff)
[![Documentation](https://docs.rs/tiff/badge.svg)](https://docs.rs/tiff)
[![Further crate info](https://img.shields.io/crates/v/tiff.svg)](https://crates.io/crates/tiff)

TIFF decoding and encoding library in pure Rust

## Supported

### Features
- Baseline spec (other than formats and tags listed below as not supported)
- Multipage
- BigTIFF
- Incremental decoding

### Formats
This table lists photometric interpretations and sample formats which are supported for encoding and decoding. The entries are `ColorType` variants for which sample bit depths are supported. Only samples where all bit depths are equal are currently supported. For example, `RGB(8)` means that the bit depth [8, 8, 8] is supported and will be interpreted as an 8 bit per channel RGB color type.

| `PhotometricInterpretation` | UINT Format | IEEEFP Format |
|-|-|-|
| `WhiteIsZero` | Gray(8\|16\|32\|64) | Gray(32\|64) |
| `BlackIsZero` | Gray(8\|16\|32\|64) | Gray(32\|64) |
| `RGB` | RGB(8\|16\|32\|64), RGBA(8\|16\|32\|64) | RGB(32\|64), RGBA(32\|64) |
| `RGBPalette` | | |
| `Mask` | | |
| `CMYK` | CMYK(8\|16\|32\|64) | CMYK(32\|64) |
| `YCbCr` | | |
| `CIELab` | | |

### Compressions

| | Decoding | Encoding |
|-|-|-|
| None | ✓ | ✓ |
| LZW | ✓ | |
| Deflate | ✓ | |
| PackBits | ✓ | |


## Not yet supported

Formats and interpretations not listed above or with empty entries are unsupported.

- Baseline tags
  - `ExtraSamples`
- Extension tags
