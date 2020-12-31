//! Decoding and Encoding of TIFF Images
//!
//! TIFF (Tagged Image File Format) is a versatile image format that supports
//! lossless and lossy compression.
//!
//! # Related Links
//! * <https://www.adobe.io/open/standards/TIFF.html> - The TIFF specification

extern crate jpeg;
extern crate miniz_oxide;
extern crate weezl;

mod bytecast;
pub mod decoder;
pub mod encoder;
mod error;
pub mod tags;

pub use self::error::{InflateError, TiffError, TiffFormatError, TiffResult, TiffUnsupportedError};

/// An enumeration over supported color types and their bit depths
#[derive(Copy, PartialEq, Eq, Debug, Clone, Hash)]
pub enum ColorType {
    /// Pixel is grayscale
    Gray(u8),

    /// Pixel contains R, G and B channels
    RGB(u8),

    /// Pixel is an index into a color palette
    Palette(u8),

    /// Pixel is grayscale with an alpha channel
    GrayA(u8),

    /// Pixel is RGB with an alpha channel
    RGBA(u8),

    /// Pixel is CMYK
    CMYK(u8),
}
