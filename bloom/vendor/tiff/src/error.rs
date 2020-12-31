use std::error::Error;
use std::fmt;
use std::io;
use std::string;

use crate::decoder::ifd::Value;
use crate::tags::{
    CompressionMethod, PhotometricInterpretation, PlanarConfiguration, SampleFormat, Tag,
};
use crate::ColorType;
use miniz_oxide::inflate::TINFLStatus;

/// Tiff error kinds.
#[derive(Debug)]
pub enum TiffError {
    /// The Image is not formatted properly.
    FormatError(TiffFormatError),

    /// The Decoder does not support features required by the image.
    UnsupportedError(TiffUnsupportedError),

    /// An I/O Error occurred while decoding the image.
    IoError(io::Error),

    /// The Limits of the Decoder is exceeded.
    LimitsExceeded,

    /// An integer conversion to or from a platform size failed, either due to
    /// limits of the platform size or limits of the format.
    IntSizeError,
}

/// The image is not formatted properly.
///
/// This indicates that the encoder producing the image might behave incorrectly or that the input
/// file has been corrupted.
///
/// The list of variants may grow to incorporate errors of future features. Matching against this
/// exhaustively is not covered by interface stability guarantees.
#[derive(Debug, Clone, PartialEq)]
pub enum TiffFormatError {
    TiffSignatureNotFound,
    TiffSignatureInvalid,
    ImageFileDirectoryNotFound,
    InconsistentSizesEncountered,
    UnexpectedCompressedData {
        actual_bytes: usize,
        required_bytes: usize,
    },
    InconsistentStripSamples {
        actual_samples: usize,
        required_samples: usize,
    },
    InvalidTag,
    InvalidTagValueType(Tag),
    RequiredTagNotFound(Tag),
    UnknownPredictor(u16),
    ByteExpected(Value),
    UnsignedIntegerExpected(Value),
    SignedIntegerExpected(Value),
    InflateError(InflateError),
    Format(String),
    RequiredTagEmpty(Tag),
    #[doc(hidden)]
    /// Do not match against this variant. It may get removed.
    __NonExhaustive,
}

impl fmt::Display for TiffFormatError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::TiffFormatError::*;
        match *self {
            TiffSignatureNotFound => write!(fmt, "TIFF signature not found."),
            TiffSignatureInvalid => write!(fmt, "TIFF signature invalid."),
            ImageFileDirectoryNotFound => write!(fmt, "Image file directory not found."),
            InconsistentSizesEncountered => write!(fmt, "Inconsistent sizes encountered."),
            UnexpectedCompressedData {
                actual_bytes,
                required_bytes,
            } => {
                write!(
                    fmt,
                    "Decompression returned different amount of bytes than expected: got {}, expected {}.",
                    actual_bytes, required_bytes
                )
            }
            InconsistentStripSamples {
                actual_samples,
                required_samples,
            } => {
                write!(
                    fmt,
                    "Inconsistent elements in strip: got {}, expected {}.",
                    actual_samples, required_samples
                )
            }
            InvalidTag => write!(fmt, "Image contains invalid tag."),
            InvalidTagValueType(ref tag) => {
                write!(fmt, "Tag `{:?}` did not have the expected value type.", tag)
            }
            RequiredTagNotFound(ref tag) => write!(fmt, "Required tag `{:?}` not found.", tag),
            UnknownPredictor(ref predictor) => {
                write!(fmt, "Unknown predictor “{}” encountered", predictor)
            }
            ByteExpected(ref val) => write!(fmt, "Expected byte, {:?} found.", val),
            UnsignedIntegerExpected(ref val) => {
                write!(fmt, "Expected unsigned integer, {:?} found.", val)
            }
            SignedIntegerExpected(ref val) => {
                write!(fmt, "Expected signed integer, {:?} found.", val)
            }
            InflateError(_) => write!(fmt, "Failed to decode inflate data."),
            Format(ref val) => write!(fmt, "Invalid format: {:?}.", val),
            RequiredTagEmpty(ref val) => write!(fmt, "Required tag {:?} was empty.", val),
            __NonExhaustive => unreachable!(),
        }
    }
}

/// Decompression failed due to faulty compressed data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InflateError {
    status: TINFLStatus,
}

impl InflateError {
    pub(crate) fn new(status: TINFLStatus) -> Self {
        Self { status }
    }
}

impl TiffError {
    pub(crate) fn from_inflate_status(status: TINFLStatus) -> Self {
        TiffError::FormatError(TiffFormatError::InflateError(InflateError::new(status)))
    }
}

/// The Decoder does not support features required by the image.
///
/// This only captures known failures for which the standard either does not require support or an
/// implementation has been planned but not yet completed. Some variants may become unused over
/// time and will then get deprecated before being removed.
///
/// The list of variants may grow. Matching against this exhaustively is not covered by interface
/// stability guarantees.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TiffUnsupportedError {
    HorizontalPredictor(ColorType),
    InterpretationWithBits(PhotometricInterpretation, Vec<u8>),
    UnknownInterpretation,
    UnknownCompressionMethod,
    UnsupportedCompressionMethod(CompressionMethod),
    UnsupportedSampleDepth(u8),
    UnsupportedSampleFormat(Vec<SampleFormat>),
    UnsupportedColorType(ColorType),
    UnsupportedBitsPerChannel(u8),
    UnsupportedPlanarConfig(Option<PlanarConfiguration>),
    UnsupportedDataType,
    #[doc(hidden)]
    /// Do not match against this variant. It may get removed.
    __NonExhaustive,
}

impl fmt::Display for TiffUnsupportedError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::TiffUnsupportedError::*;
        match *self {
            HorizontalPredictor(color_type) => write!(
                fmt,
                "Horizontal predictor for {:?} is unsupported.",
                color_type
            ),
            InterpretationWithBits(ref photometric_interpretation, ref bits_per_sample) => write!(
                fmt,
                "{:?} with {:?} bits per sample is unsupported",
                photometric_interpretation, bits_per_sample
            ),
            UnknownInterpretation => write!(
                fmt,
                "The image is using an unknown photometric interpretation."
            ),
            UnknownCompressionMethod => write!(fmt, "Unknown compression method."),
            UnsupportedCompressionMethod(method) => {
                write!(fmt, "Compression method {:?} is unsupported", method)
            }
            UnsupportedSampleDepth(samples) => {
                write!(fmt, "{} samples per pixel is unsupported.", samples)
            }
            UnsupportedSampleFormat(ref formats) => {
                write!(fmt, "Sample format {:?} is unsupported.", formats)
            }
            UnsupportedColorType(color_type) => {
                write!(fmt, "Color type {:?} is unsupported", color_type)
            }
            UnsupportedBitsPerChannel(bits) => {
                write!(fmt, "{} bits per channel not supported", bits)
            }
            UnsupportedPlanarConfig(config) => {
                write!(fmt, "Unsupported planar configuration “{:?}”.", config)
            }
            UnsupportedDataType => write!(fmt, "Unsupported data type."),
            __NonExhaustive => unreachable!(),
        }
    }
}

impl fmt::Display for TiffError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            TiffError::FormatError(ref e) => write!(fmt, "Format error: {}", e),
            TiffError::UnsupportedError(ref f) => write!(
                fmt,
                "The Decoder does not support the \
                 image format `{}`",
                f
            ),
            TiffError::IoError(ref e) => e.fmt(fmt),
            TiffError::LimitsExceeded => write!(fmt, "The Decoder limits are exceeded"),
            TiffError::IntSizeError => write!(fmt, "Platform or format size limits exceeded"),
        }
    }
}

impl Error for TiffError {
    fn description(&self) -> &str {
        match *self {
            TiffError::FormatError(..) => "Format error",
            TiffError::UnsupportedError(..) => "Unsupported error",
            TiffError::IoError(..) => "IO error",
            TiffError::LimitsExceeded => "Decoder limits exceeded",
            TiffError::IntSizeError => "Platform or format size limits exceeded",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            TiffError::IoError(ref e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for TiffError {
    fn from(err: io::Error) -> TiffError {
        TiffError::IoError(err)
    }
}

impl From<string::FromUtf8Error> for TiffError {
    fn from(_err: string::FromUtf8Error) -> TiffError {
        TiffError::FormatError(TiffFormatError::InvalidTag)
    }
}

impl From<TiffFormatError> for TiffError {
    fn from(err: TiffFormatError) -> TiffError {
        TiffError::FormatError(err)
    }
}

impl From<TiffUnsupportedError> for TiffError {
    fn from(err: TiffUnsupportedError) -> TiffError {
        TiffError::UnsupportedError(err)
    }
}

impl From<std::num::TryFromIntError> for TiffError {
    fn from(_err: std::num::TryFromIntError) -> TiffError {
        TiffError::IntSizeError
    }
}

/// Result of an image decoding/encoding process
pub type TiffResult<T> = Result<T, TiffError>;
