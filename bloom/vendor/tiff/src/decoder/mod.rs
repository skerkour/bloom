use std::cmp;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::{self, Read, Seek};

use crate::{ColorType, TiffError, TiffFormatError, TiffResult, TiffUnsupportedError};

use self::ifd::Directory;
use crate::tags::{
    CompressionMethod, PhotometricInterpretation, Predictor, SampleFormat, Tag, Type,
};

use self::stream::{
    ByteOrder, DeflateReader, EndianReader, JpegReader, LZWReader, PackBitsReader, SmartReader,
};

pub mod ifd;
mod stream;

/// Result of a decoding process
#[derive(Debug)]
pub enum DecodingResult {
    /// A vector of unsigned bytes
    U8(Vec<u8>),
    /// A vector of unsigned words
    U16(Vec<u16>),
    /// A vector of 32 bit unsigned ints
    U32(Vec<u32>),
    /// A vector of 64 bit unsigned ints
    U64(Vec<u64>),
    /// A vector of 32 bit IEEE floats
    F32(Vec<f32>),
    /// A vector of 64 bit IEEE floats
    F64(Vec<f64>),
}

impl DecodingResult {
    fn new_u8(size: usize, limits: &Limits) -> TiffResult<DecodingResult> {
        if size > limits.decoding_buffer_size {
            Err(TiffError::LimitsExceeded)
        } else {
            Ok(DecodingResult::U8(vec![0; size]))
        }
    }

    fn new_u16(size: usize, limits: &Limits) -> TiffResult<DecodingResult> {
        if size > limits.decoding_buffer_size / 2 {
            Err(TiffError::LimitsExceeded)
        } else {
            Ok(DecodingResult::U16(vec![0; size]))
        }
    }

    fn new_u32(size: usize, limits: &Limits) -> TiffResult<DecodingResult> {
        if size > limits.decoding_buffer_size / 4 {
            Err(TiffError::LimitsExceeded)
        } else {
            Ok(DecodingResult::U32(vec![0; size]))
        }
    }

    fn new_u64(size: usize, limits: &Limits) -> TiffResult<DecodingResult> {
        if size > limits.decoding_buffer_size / 8 {
            Err(TiffError::LimitsExceeded)
        } else {
            Ok(DecodingResult::U64(vec![0; size]))
        }
    }

    fn new_f32(size: usize, limits: &Limits) -> TiffResult<DecodingResult> {
        if size > limits.decoding_buffer_size / std::mem::size_of::<f32>() {
            Err(TiffError::LimitsExceeded)
        } else {
            Ok(DecodingResult::F32(vec![0.0; size]))
        }
    }

    fn new_f64(size: usize, limits: &Limits) -> TiffResult<DecodingResult> {
        if size > limits.decoding_buffer_size / std::mem::size_of::<f64>() {
            Err(TiffError::LimitsExceeded)
        } else {
            Ok(DecodingResult::F64(vec![0.0; size]))
        }
    }

    pub fn as_buffer(&mut self, start: usize) -> DecodingBuffer {
        match *self {
            DecodingResult::U8(ref mut buf) => DecodingBuffer::U8(&mut buf[start..]),
            DecodingResult::U16(ref mut buf) => DecodingBuffer::U16(&mut buf[start..]),
            DecodingResult::U32(ref mut buf) => DecodingBuffer::U32(&mut buf[start..]),
            DecodingResult::U64(ref mut buf) => DecodingBuffer::U64(&mut buf[start..]),
            DecodingResult::F32(ref mut buf) => DecodingBuffer::F32(&mut buf[start..]),
            DecodingResult::F64(ref mut buf) => DecodingBuffer::F64(&mut buf[start..]),
        }
    }
}

// A buffer for image decoding
pub enum DecodingBuffer<'a> {
    /// A slice of unsigned bytes
    U8(&'a mut [u8]),
    /// A slice of unsigned words
    U16(&'a mut [u16]),
    /// A slice of 32 bit unsigned ints
    U32(&'a mut [u32]),
    /// A slice of 64 bit unsigned ints
    U64(&'a mut [u64]),
    /// A slice of 32 bit IEEE floats
    F32(&'a mut [f32]),
    /// A slice of 64 bit IEEE floats
    F64(&'a mut [f64]),
}

impl<'a> DecodingBuffer<'a> {
    fn len(&self) -> usize {
        match *self {
            DecodingBuffer::U8(ref buf) => buf.len(),
            DecodingBuffer::U16(ref buf) => buf.len(),
            DecodingBuffer::U32(ref buf) => buf.len(),
            DecodingBuffer::U64(ref buf) => buf.len(),
            DecodingBuffer::F32(ref buf) => buf.len(),
            DecodingBuffer::F64(ref buf) => buf.len(),
        }
    }

    fn byte_len(&self) -> usize {
        match *self {
            DecodingBuffer::U8(_) => 1,
            DecodingBuffer::U16(_) => 2,
            DecodingBuffer::U32(_) => 4,
            DecodingBuffer::U64(_) => 8,
            DecodingBuffer::F32(_) => 4,
            DecodingBuffer::F64(_) => 8,
        }
    }

    fn copy<'b>(&'b mut self) -> DecodingBuffer<'b>
    where
        'a: 'b,
    {
        match *self {
            DecodingBuffer::U8(ref mut buf) => DecodingBuffer::U8(buf),
            DecodingBuffer::U16(ref mut buf) => DecodingBuffer::U16(buf),
            DecodingBuffer::U32(ref mut buf) => DecodingBuffer::U32(buf),
            DecodingBuffer::U64(ref mut buf) => DecodingBuffer::U64(buf),
            DecodingBuffer::F32(ref mut buf) => DecodingBuffer::F32(buf),
            DecodingBuffer::F64(ref mut buf) => DecodingBuffer::F64(buf),
        }
    }
}

#[derive(Debug)]
struct StripDecodeState {
    strip_index: usize,
    strip_offsets: Vec<u64>,
    strip_bytes: Vec<u64>,
}

/// Decoding limits
#[derive(Clone, Debug)]
pub struct Limits {
    /// The maximum size of any `DecodingResult` in bytes, the default is
    /// 256MiB. If the entire image is decoded at once, then this will
    /// be the maximum size of the image. If it is decoded one strip at a
    /// time, this will be the maximum size of a strip.
    pub decoding_buffer_size: usize,
    /// The maximum size of any ifd value in bytes, the default is
    /// 1MiB.
    pub ifd_value_size: usize,
    /// Maximum size for intermediate buffer which may be used to limit the amount of data read per
    /// segment even if the entire image is decoded at once.
    pub intermediate_buffer_size: usize,
    /// The purpose of this is to prevent all the fields of the struct from
    /// being public, as this would make adding new fields a major version
    /// bump.
    _non_exhaustive: (),
}

impl Limits {
    /// A configuration that does not impose any limits.
    ///
    /// This is a good start if the caller only wants to impose selective limits, contrary to the
    /// default limits which allows selectively disabling limits.
    ///
    /// Note that this configuration is likely to crash on excessively large images since,
    /// naturally, the machine running the program does not have infinite memory.
    pub fn unlimited() -> Limits {
        Limits {
            decoding_buffer_size: usize::max_value(),
            ifd_value_size: usize::max_value(),
            intermediate_buffer_size: usize::max_value(),
            _non_exhaustive: (),
        }
    }
}

impl Default for Limits {
    fn default() -> Limits {
        Limits {
            decoding_buffer_size: 256 * 1024 * 1024,
            intermediate_buffer_size: 128 * 1024 * 1024,
            ifd_value_size: 1024 * 1024,
            _non_exhaustive: (),
        }
    }
}

/// The representation of a TIFF decoder
///
/// Currently does not support decoding of interlaced images
#[derive(Debug)]
pub struct Decoder<R>
where
    R: Read + Seek,
{
    reader: SmartReader<R>,
    byte_order: ByteOrder,
    bigtiff: bool,
    limits: Limits,
    next_ifd: Option<u64>,
    ifd: Option<Directory>,
    width: u32,
    height: u32,
    bits_per_sample: Vec<u8>,
    samples: u8,
    sample_format: Vec<SampleFormat>,
    photometric_interpretation: PhotometricInterpretation,
    compression_method: CompressionMethod,
    strip_decoder: Option<StripDecodeState>,
}

trait Wrapping {
    fn wrapping_add(&self, other: Self) -> Self;
}

impl Wrapping for u8 {
    fn wrapping_add(&self, other: Self) -> Self {
        u8::wrapping_add(*self, other)
    }
}

impl Wrapping for u16 {
    fn wrapping_add(&self, other: Self) -> Self {
        u16::wrapping_add(*self, other)
    }
}

impl Wrapping for u32 {
    fn wrapping_add(&self, other: Self) -> Self {
        u32::wrapping_add(*self, other)
    }
}

impl Wrapping for u64 {
    fn wrapping_add(&self, other: Self) -> Self {
        u64::wrapping_add(*self, other)
    }
}

fn rev_hpredict_nsamp<T>(image: &mut [T], size: (u32, u32), samples: usize) -> TiffResult<()>
where
    T: Copy + Wrapping,
{
    let width = usize::try_from(size.0)?;
    let height = usize::try_from(size.1)?;
    for row in 0..height {
        for col in samples..width * samples {
            let prev_pixel = image[(row * width * samples + col - samples)];
            let pixel = &mut image[(row * width * samples + col)];
            *pixel = pixel.wrapping_add(prev_pixel);
        }
    }
    Ok(())
}

fn rev_hpredict(image: DecodingBuffer, size: (u32, u32), color_type: ColorType) -> TiffResult<()> {
    let samples = match color_type {
        ColorType::Gray(8) | ColorType::Gray(16) | ColorType::Gray(32) | ColorType::Gray(64) => 1,
        ColorType::RGB(8) | ColorType::RGB(16) | ColorType::RGB(32) | ColorType::RGB(64) => 3,
        ColorType::RGBA(8)
        | ColorType::RGBA(16)
        | ColorType::RGBA(32)
        | ColorType::RGBA(64)
        | ColorType::CMYK(8)
        | ColorType::CMYK(16)
        | ColorType::CMYK(32)
        | ColorType::CMYK(64) => 4,
        _ => {
            return Err(TiffError::UnsupportedError(
                TiffUnsupportedError::HorizontalPredictor(color_type),
            ))
        }
    };
    match image {
        DecodingBuffer::U8(buf) => {
            rev_hpredict_nsamp(buf, size, samples)?;
        }
        DecodingBuffer::U16(buf) => {
            rev_hpredict_nsamp(buf, size, samples)?;
        }
        DecodingBuffer::U32(buf) => {
            rev_hpredict_nsamp(buf, size, samples)?;
        }
        DecodingBuffer::U64(buf) => {
            rev_hpredict_nsamp(buf, size, samples)?;
        }
        DecodingBuffer::F32(_buf) => {
            // FIXME: check how this is defined.
            // See issue #89.
            // rev_hpredict_nsamp(buf, size, samples)?;
            return Err(TiffError::UnsupportedError(
                TiffUnsupportedError::HorizontalPredictor(color_type),
            ));
        }
        DecodingBuffer::F64(_buf) => {
            //FIXME: check how this is defined.
            // See issue #89.
            // rev_hpredict_nsamp(buf, size, samples)?;
            return Err(TiffError::UnsupportedError(
                TiffUnsupportedError::HorizontalPredictor(color_type),
            ));
        }
    }
    Ok(())
}

impl<R: Read + Seek> Decoder<R> {
    /// Create a new decoder that decodes from the stream ```r```
    pub fn new(r: R) -> TiffResult<Decoder<R>> {
        Decoder {
            reader: SmartReader::wrap(r, ByteOrder::LittleEndian),
            byte_order: ByteOrder::LittleEndian,
            bigtiff: false,
            limits: Default::default(),
            next_ifd: None,
            ifd: None,
            width: 0,
            height: 0,
            bits_per_sample: vec![1],
            samples: 1,
            sample_format: vec![SampleFormat::Uint],
            photometric_interpretation: PhotometricInterpretation::BlackIsZero,
            compression_method: CompressionMethod::None,
            strip_decoder: None,
        }
        .init()
    }

    pub fn with_limits(mut self, limits: Limits) -> Decoder<R> {
        self.limits = limits;
        self
    }

    pub fn dimensions(&mut self) -> TiffResult<(u32, u32)> {
        Ok((self.width, self.height))
    }

    pub fn colortype(&mut self) -> TiffResult<ColorType> {
        match self.photometric_interpretation {
            PhotometricInterpretation::RGB => match self.bits_per_sample[..] {
                [r, g, b] if [r, r] == [g, b] => Ok(ColorType::RGB(r)),
                [r, g, b, a] if [r, r, r] == [g, b, a] => Ok(ColorType::RGBA(r)),
                // FIXME: We should _ignore_ other components. In particular:
                // > Beware of extra components. Some TIFF files may have more components per pixel
                // than you think. A Baseline TIFF reader must skip over them gracefully,using the
                // values of the SamplesPerPixel and BitsPerSample fields.
                // > -- TIFF 6.0 Specification, Section 7, Additional Baseline requirements.
                _ => Err(TiffError::UnsupportedError(
                    TiffUnsupportedError::InterpretationWithBits(
                        self.photometric_interpretation,
                        self.bits_per_sample.clone(),
                    ),
                )),
            },
            PhotometricInterpretation::CMYK => match self.bits_per_sample[..] {
                [c, m, y, k] if [c, c, c] == [m, y, k] => Ok(ColorType::CMYK(c)),
                _ => Err(TiffError::UnsupportedError(
                    TiffUnsupportedError::InterpretationWithBits(
                        self.photometric_interpretation,
                        self.bits_per_sample.clone(),
                    ),
                )),
            },
            PhotometricInterpretation::BlackIsZero | PhotometricInterpretation::WhiteIsZero
                if self.bits_per_sample.len() == 1 =>
            {
                Ok(ColorType::Gray(self.bits_per_sample[0]))
            }

            // TODO: this is bad we should not fail at this point
            _ => Err(TiffError::UnsupportedError(
                TiffUnsupportedError::InterpretationWithBits(
                    self.photometric_interpretation,
                    self.bits_per_sample.clone(),
                ),
            )),
        }
    }

    fn read_header(&mut self) -> TiffResult<()> {
        let mut endianess = Vec::with_capacity(2);
        self.reader.by_ref().take(2).read_to_end(&mut endianess)?;
        match &*endianess {
            b"II" => {
                self.byte_order = ByteOrder::LittleEndian;
                self.reader.byte_order = ByteOrder::LittleEndian;
            }
            b"MM" => {
                self.byte_order = ByteOrder::BigEndian;
                self.reader.byte_order = ByteOrder::BigEndian;
            }
            _ => {
                return Err(TiffError::FormatError(
                    TiffFormatError::TiffSignatureNotFound,
                ))
            }
        }
        match self.read_short()? {
            42 => self.bigtiff = false,
            43 => {
                self.bigtiff = true;
                // Read bytesize of offsets (in bigtiff it's alway 8 but provide a way to move to 16 some day)
                if self.read_short()? != 8 {
                    return Err(TiffError::FormatError(
                        TiffFormatError::TiffSignatureNotFound,
                    ));
                }
                // This constant should always be 0
                if self.read_short()? != 0 {
                    return Err(TiffError::FormatError(
                        TiffFormatError::TiffSignatureNotFound,
                    ));
                }
            }
            _ => {
                return Err(TiffError::FormatError(
                    TiffFormatError::TiffSignatureInvalid,
                ))
            }
        }
        self.next_ifd = match self.read_ifd_offset()? {
            0 => None,
            n => Some(n),
        };
        Ok(())
    }

    /// Initializes the decoder.
    pub fn init(mut self) -> TiffResult<Decoder<R>> {
        self.read_header()?;
        self.next_image()?;
        Ok(self)
    }

    /// Reads in the next image.
    /// If there is no further image in the TIFF file a format error is returned.
    /// To determine whether there are more images call `TIFFDecoder::more_images` instead.
    pub fn next_image(&mut self) -> TiffResult<()> {
        self.ifd = Some(self.read_ifd()?);
        self.width = self.get_tag_u32(Tag::ImageWidth)?;
        self.height = self.get_tag_u32(Tag::ImageLength)?;
        self.strip_decoder = None;

        self.photometric_interpretation = self
            .find_tag_unsigned(Tag::PhotometricInterpretation)?
            .and_then(PhotometricInterpretation::from_u16)
            .ok_or(TiffUnsupportedError::UnknownInterpretation)?;

        if let Some(val) = self.find_tag_unsigned(Tag::Compression)? {
            self.compression_method = CompressionMethod::from_u16(val)
                .ok_or(TiffUnsupportedError::UnknownCompressionMethod)?;
        }
        if let Some(val) = self.find_tag_unsigned(Tag::SamplesPerPixel)? {
            self.samples = val;
        }
        if let Some(vals) = self.find_tag_unsigned_vec(Tag::SampleFormat)? {
            self.sample_format = vals
                .into_iter()
                .map(SampleFormat::from_u16_exhaustive)
                .collect();

            // TODO: for now, only homogenous formats across samples are supported.
            if !self.sample_format.windows(2).all(|s| s[0] == s[1]) {
                return Err(TiffUnsupportedError::UnsupportedSampleFormat(
                    self.sample_format.clone(),
                )
                .into());
            }
        }
        match self.samples {
            1 | 3 | 4 => {
                if let Some(val) = self.find_tag_unsigned_vec(Tag::BitsPerSample)? {
                    self.bits_per_sample = val;
                }
            }
            _ => return Err(TiffUnsupportedError::UnsupportedSampleDepth(self.samples).into()),
        }

        Ok(())
    }

    /// Returns `true` if there is at least one more image available.
    pub fn more_images(&self) -> bool {
        self.next_ifd.is_some()
    }

    /// Returns the byte_order
    pub fn byte_order(&self) -> ByteOrder {
        self.byte_order
    }

    #[inline]
    pub fn read_ifd_offset(&mut self) -> Result<u64, io::Error> {
        if self.bigtiff {
            self.read_long8()
        } else {
            self.read_long().map(u64::from)
        }
    }

    /// Reads a TIFF byte value
    #[inline]
    pub fn read_byte(&mut self) -> Result<u8, io::Error> {
        let mut buf = [0; 1];
        self.reader.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    /// Reads a TIFF short value
    #[inline]
    pub fn read_short(&mut self) -> Result<u16, io::Error> {
        self.reader.read_u16()
    }

    /// Reads a TIFF sshort value
    #[inline]
    pub fn read_sshort(&mut self) -> Result<i16, io::Error> {
        self.reader.read_i16()
    }

    /// Reads a TIFF long value
    #[inline]
    pub fn read_long(&mut self) -> Result<u32, io::Error> {
        self.reader.read_u32()
    }

    /// Reads a TIFF slong value
    #[inline]
    pub fn read_slong(&mut self) -> Result<i32, io::Error> {
        self.reader.read_i32()
    }

    /// Reads a TIFF float value
    #[inline]
    pub fn read_float(&mut self) -> Result<f32, io::Error> {
        self.reader.read_f32()
    }

    /// Reads a TIFF double value
    #[inline]
    pub fn read_double(&mut self) -> Result<f64, io::Error> {
        self.reader.read_f64()
    }

    #[inline]
    pub fn read_long8(&mut self) -> Result<u64, io::Error> {
        self.reader.read_u64()
    }

    /// Reads a string
    #[inline]
    pub fn read_string(&mut self, length: usize) -> TiffResult<String> {
        let mut out = vec![0; length];
        self.reader.read_exact(&mut out)?;
        // Strings may be null-terminated, so we trim anything downstream of the null byte
        if let Some(first) = out.iter().position(|&b| b == 0) {
            out.truncate(first);
        }
        Ok(String::from_utf8(out)?)
    }

    /// Reads a TIFF IFA offset/value field
    #[inline]
    pub fn read_offset(&mut self) -> TiffResult<[u8; 4]> {
        if self.bigtiff {
            return Err(TiffError::FormatError(
                TiffFormatError::InconsistentSizesEncountered,
            ));
        }
        let mut val = [0; 4];
        self.reader.read_exact(&mut val)?;
        Ok(val)
    }

    /// Reads a TIFF IFA offset/value field
    #[inline]
    pub fn read_offset_u64(&mut self) -> Result<[u8; 8], io::Error> {
        let mut val = [0; 8];
        self.reader.read_exact(&mut val)?;
        Ok(val)
    }

    /// Moves the cursor to the specified offset
    #[inline]
    pub fn goto_offset(&mut self, offset: u32) -> io::Result<()> {
        self.goto_offset_u64(offset.into())
    }

    #[inline]
    pub fn goto_offset_u64(&mut self, offset: u64) -> io::Result<()> {
        self.reader.seek(io::SeekFrom::Start(offset)).map(|_| ())
    }

    /// Reads a IFD entry.
    // An IFD entry has four fields:
    //
    // Tag   2 bytes
    // Type  2 bytes
    // Count 4 bytes
    // Value 4 bytes either a pointer the value itself
    fn read_entry(&mut self) -> TiffResult<Option<(Tag, ifd::Entry)>> {
        let tag = Tag::from_u16_exhaustive(self.read_short()?);
        let type_ = match Type::from_u16(self.read_short()?) {
            Some(t) => t,
            None => {
                // Unknown type. Skip this entry according to spec.
                self.read_long()?;
                self.read_long()?;
                return Ok(None);
            }
        };
        let entry = if self.bigtiff {
            ifd::Entry::new_u64(type_, self.read_long8()?, self.read_offset_u64()?)
        } else {
            ifd::Entry::new(type_, self.read_long()?, self.read_offset()?)
        };
        Ok(Some((tag, entry)))
    }

    /// Reads the next IFD
    fn read_ifd(&mut self) -> TiffResult<Directory> {
        let mut dir: Directory = HashMap::new();
        match self.next_ifd {
            None => {
                return Err(TiffError::FormatError(
                    TiffFormatError::ImageFileDirectoryNotFound,
                ))
            }
            Some(offset) => self.goto_offset_u64(offset)?,
        }
        let num_tags = if self.bigtiff {
            self.read_long8()?
        } else {
            self.read_short()?.into()
        };
        for _ in 0..num_tags {
            let (tag, entry) = match self.read_entry()? {
                Some(val) => val,
                None => {
                    continue;
                } // Unknown data type in tag, skip
            };
            dir.insert(tag, entry);
        }
        self.next_ifd = match self.read_ifd_offset()? {
            0 => None,
            n => Some(n),
        };
        Ok(dir)
    }

    /// Tries to retrieve a tag.
    /// Return `Ok(None)` if the tag is not present.
    pub fn find_tag(&mut self, tag: Tag) -> TiffResult<Option<ifd::Value>> {
        let entry = match self.ifd.as_ref().unwrap().get(&tag) {
            None => return Ok(None),
            Some(entry) => entry.clone(),
        };

        let limits = self.limits.clone();

        Ok(Some(entry.val(&limits, self)?))
    }

    /// Tries to retrieve a tag and convert it to the desired unsigned type.
    pub fn find_tag_unsigned<T: TryFrom<u64>>(&mut self, tag: Tag) -> TiffResult<Option<T>> {
        self.find_tag(tag)?
            .map(|v| v.into_u64())
            .transpose()?
            .map(|value| {
                T::try_from(value).map_err(|_| TiffFormatError::InvalidTagValueType(tag).into())
            })
            .transpose()
    }

    /// Tries to retrieve a vector of all a tag's values and convert them to
    /// the desired unsigned type.
    pub fn find_tag_unsigned_vec<T: TryFrom<u64>>(
        &mut self,
        tag: Tag,
    ) -> TiffResult<Option<Vec<T>>> {
        self.find_tag(tag)?
            .map(|v| v.into_u64_vec())
            .transpose()?
            .map(|v| {
                v.into_iter()
                    .map(|u| {
                        T::try_from(u).map_err(|_| TiffFormatError::InvalidTagValueType(tag).into())
                    })
                    .collect()
            })
            .transpose()
    }

    /// Tries to retrieve a tag and convert it to the desired unsigned type.
    /// Returns an error if the tag is not present.
    pub fn get_tag_unsigned<T: TryFrom<u64>>(&mut self, tag: Tag) -> TiffResult<T> {
        self.find_tag_unsigned(tag)?
            .ok_or_else(|| TiffFormatError::RequiredTagNotFound(tag).into())
    }

    /// Tries to retrieve a tag.
    /// Returns an error if the tag is not present
    pub fn get_tag(&mut self, tag: Tag) -> TiffResult<ifd::Value> {
        match self.find_tag(tag)? {
            Some(val) => Ok(val),
            None => Err(TiffError::FormatError(
                TiffFormatError::RequiredTagNotFound(tag),
            )),
        }
    }

    /// Tries to retrieve a tag and convert it to the desired type.
    pub fn get_tag_u32(&mut self, tag: Tag) -> TiffResult<u32> {
        self.get_tag(tag)?.into_u32()
    }
    pub fn get_tag_u64(&mut self, tag: Tag) -> TiffResult<u64> {
        self.get_tag(tag)?.into_u64()
    }

    /// Tries to retrieve a tag and convert it to the desired type.
    pub fn get_tag_f32(&mut self, tag: Tag) -> TiffResult<f32> {
        self.get_tag(tag)?.into_f32()
    }

    /// Tries to retrieve a tag and convert it to the desired type.
    pub fn get_tag_f64(&mut self, tag: Tag) -> TiffResult<f64> {
        self.get_tag(tag)?.into_f64()
    }

    /// Tries to retrieve a tag and convert it to the desired type.
    pub fn get_tag_u32_vec(&mut self, tag: Tag) -> TiffResult<Vec<u32>> {
        self.get_tag(tag)?.into_u32_vec()
    }

    pub fn get_tag_u16_vec(&mut self, tag: Tag) -> TiffResult<Vec<u16>> {
        self.get_tag(tag)?.into_u16_vec()
    }
    pub fn get_tag_u64_vec(&mut self, tag: Tag) -> TiffResult<Vec<u64>> {
        self.get_tag(tag)?.into_u64_vec()
    }

    /// Tries to retrieve a tag and convert it to the desired type.
    pub fn get_tag_f32_vec(&mut self, tag: Tag) -> TiffResult<Vec<f32>> {
        self.get_tag(tag)?.into_f32_vec()
    }

    /// Tries to retrieve a tag and convert it to the desired type.
    pub fn get_tag_f64_vec(&mut self, tag: Tag) -> TiffResult<Vec<f64>> {
        self.get_tag(tag)?.into_f64_vec()
    }

    /// Tries to retrieve a tag and convert it to a 8bit vector.
    pub fn get_tag_u8_vec(&mut self, tag: Tag) -> TiffResult<Vec<u8>> {
        self.get_tag(tag)?.into_u8_vec()
    }

    /// Tries to retrieve a tag and convert it to a ascii vector.
    pub fn get_tag_ascii_string(&mut self, tag: Tag) -> TiffResult<String> {
        self.get_tag(tag)?.into_string()
    }

    /// Decompresses the strip into the supplied buffer.
    /// Returns the number of bytes read.
    fn expand_strip<'a>(
        &mut self,
        buffer: DecodingBuffer<'a>,
        offset: u64,
        length: u64,
        strip_sample_count: usize,
    ) -> TiffResult<usize> {
        let color_type = self.colortype()?;
        self.goto_offset_u64(offset)?;
        let (bytes, mut reader): (usize, Box<dyn EndianReader>) = match self.compression_method {
            CompressionMethod::None => {
                let order = self.reader.byte_order;
                (
                    usize::try_from(length)?,
                    Box::new(SmartReader::wrap(&mut self.reader, order)),
                )
            }
            CompressionMethod::LZW => {
                let (bytes, reader) = LZWReader::new(
                    &mut self.reader,
                    usize::try_from(length)?,
                    strip_sample_count * buffer.byte_len(),
                )?;
                (bytes, Box::new(reader))
            }
            CompressionMethod::PackBits => {
                let order = self.reader.byte_order;
                let (bytes, reader) =
                    PackBitsReader::new(&mut self.reader, order, usize::try_from(length)?)?;
                (bytes, Box::new(reader))
            }
            CompressionMethod::OldDeflate => {
                let (bytes, reader) = DeflateReader::new(&mut self.reader, strip_sample_count)?;
                (bytes, Box::new(reader))
            }
            method => {
                return Err(TiffError::UnsupportedError(
                    TiffUnsupportedError::UnsupportedCompressionMethod(method),
                ))
            }
        };

        // FIXME: this might be suboptimal. We might default remaining bits to ´0`, which some
        // other decoders might do.
        if bytes / buffer.byte_len() > strip_sample_count {
            return Err(TiffError::FormatError(
                TiffFormatError::UnexpectedCompressedData {
                    actual_bytes: bytes,
                    required_bytes: strip_sample_count * buffer.byte_len(),
                },
            ));
        }

        Ok(match (color_type, buffer) {
            (ColorType::RGB(8), DecodingBuffer::U8(ref mut buffer))
            | (ColorType::RGBA(8), DecodingBuffer::U8(ref mut buffer))
            | (ColorType::CMYK(8), DecodingBuffer::U8(ref mut buffer)) => {
                reader.read_exact(&mut buffer[..bytes])?;
                bytes
            }
            (ColorType::RGBA(16), DecodingBuffer::U16(ref mut buffer))
            | (ColorType::RGB(16), DecodingBuffer::U16(ref mut buffer))
            | (ColorType::CMYK(16), DecodingBuffer::U16(ref mut buffer)) => {
                reader.read_u16_into(&mut buffer[..bytes / 2])?;
                bytes / 2
            }
            (ColorType::RGBA(32), DecodingBuffer::U32(ref mut buffer))
            | (ColorType::RGB(32), DecodingBuffer::U32(ref mut buffer))
            | (ColorType::CMYK(32), DecodingBuffer::U32(ref mut buffer)) => {
                reader.read_u32_into(&mut buffer[..bytes / 4])?;
                bytes / 4
            }
            (ColorType::RGBA(32), DecodingBuffer::F32(ref mut buffer))
            | (ColorType::RGB(32), DecodingBuffer::F32(ref mut buffer))
            | (ColorType::CMYK(32), DecodingBuffer::F32(ref mut buffer)) => {
                reader.read_f32_into(&mut buffer[..bytes / 4])?;
                bytes / 4
            }
            (ColorType::RGBA(64), DecodingBuffer::F64(ref mut buffer))
            | (ColorType::RGB(64), DecodingBuffer::F64(ref mut buffer))
            | (ColorType::CMYK(64), DecodingBuffer::F64(ref mut buffer)) => {
                reader.read_f64_into(&mut buffer[..bytes / 8])?;
                bytes / 8
            }
            (ColorType::RGBA(64), DecodingBuffer::U64(ref mut buffer))
            | (ColorType::RGB(64), DecodingBuffer::U64(ref mut buffer))
            | (ColorType::CMYK(64), DecodingBuffer::U64(ref mut buffer)) => {
                reader.read_u64_into(&mut buffer[..bytes / 8])?;
                bytes / 8
            }
            (ColorType::Gray(64), DecodingBuffer::U64(ref mut buffer)) => {
                reader.read_u64_into(&mut buffer[..bytes / 8])?;
                if self.photometric_interpretation == PhotometricInterpretation::WhiteIsZero {
                    for datum in buffer[..bytes / 8].iter_mut() {
                        *datum = 0xffff_ffff_ffff_ffff - *datum
                    }
                }
                bytes / 8
            }
            (ColorType::Gray(32), DecodingBuffer::U32(ref mut buffer)) => {
                reader.read_u32_into(&mut buffer[..bytes / 4])?;
                if self.photometric_interpretation == PhotometricInterpretation::WhiteIsZero {
                    for datum in buffer[..bytes / 4].iter_mut() {
                        *datum = 0xffff_ffff - *datum
                    }
                }
                bytes / 4
            }
            (ColorType::Gray(16), DecodingBuffer::U16(ref mut buffer)) => {
                reader.read_u16_into(&mut buffer[..bytes / 2])?;
                if self.photometric_interpretation == PhotometricInterpretation::WhiteIsZero {
                    for datum in buffer[..bytes / 2].iter_mut() {
                        *datum = 0xffff - *datum
                    }
                }
                bytes / 2
            }
            (ColorType::Gray(n), DecodingBuffer::U8(ref mut buffer)) if n <= 8 => {
                reader.read_exact(&mut buffer[..bytes])?;
                if self.photometric_interpretation == PhotometricInterpretation::WhiteIsZero {
                    for byte in buffer[..bytes].iter_mut() {
                        *byte = 0xff - *byte
                    }
                }
                bytes
            }
            (ColorType::Gray(32), DecodingBuffer::F32(ref mut buffer)) => {
                reader.read_f32_into(&mut buffer[..bytes / 4])?;
                if self.photometric_interpretation == PhotometricInterpretation::WhiteIsZero {
                    for datum in buffer[..bytes / 4].iter_mut() {
                        // FIXME: assumes [0, 1) range for floats
                        *datum = 1.0 - *datum
                    }
                }
                bytes / 4
            }
            (ColorType::Gray(64), DecodingBuffer::F64(ref mut buffer)) => {
                reader.read_f64_into(&mut buffer[..bytes / 8])?;
                if self.photometric_interpretation == PhotometricInterpretation::WhiteIsZero {
                    for datum in buffer[..bytes / 8].iter_mut() {
                        // FIXME: assumes [0, 1) range for floats
                        *datum = 1.0 - *datum
                    }
                }
                bytes / 8
            }
            (type_, _) => {
                return Err(TiffError::UnsupportedError(
                    TiffUnsupportedError::UnsupportedColorType(type_),
                ))
            }
        })
    }

    /// Number of strips in image
    pub fn strip_count(&mut self) -> TiffResult<u32> {
        let rows_per_strip = self.get_tag_u32(Tag::RowsPerStrip).unwrap_or(self.height);

        if rows_per_strip == 0 {
            return Ok(0);
        }

        Ok((self.height + rows_per_strip - 1) / rows_per_strip)
    }

    fn initialize_strip_decoder(&mut self) -> TiffResult<()> {
        if self.strip_decoder.is_none() {
            let strip_offsets = self.get_tag_u64_vec(Tag::StripOffsets)?;
            let strip_bytes = self.get_tag_u64_vec(Tag::StripByteCounts)?;

            self.strip_decoder = Some(StripDecodeState {
                strip_index: 0,
                strip_offsets,
                strip_bytes,
            });
        }
        Ok(())
    }

    pub fn read_jpeg(&mut self) -> TiffResult<DecodingResult> {
        let offsets = self.get_tag_u32_vec(Tag::StripOffsets)?;
        let bytes = self.get_tag_u32_vec(Tag::StripByteCounts)?;

        let jpeg_tables: Option<Vec<u8>> = match self.find_tag(Tag::JPEGTables) {
            Ok(None) => None,
            Ok(_) => Some(self.get_tag_u8_vec(Tag::JPEGTables)?),
            Err(e) => return Err(e),
        };

        if offsets.len() == 0 {
            return Err(TiffError::FormatError(TiffFormatError::RequiredTagEmpty(
                Tag::StripOffsets,
            )));
        }
        if offsets.len() != bytes.len() {
            return Err(TiffError::FormatError(
                TiffFormatError::InconsistentSizesEncountered,
            ));
        }

        let mut res_img = Vec::with_capacity(offsets[0] as usize);

        for (idx, offset) in offsets.iter().enumerate() {
            if bytes[idx] as usize > self.limits.intermediate_buffer_size {
                return Err(TiffError::LimitsExceeded);
            }

            self.goto_offset(*offset)?;
            let jpeg_reader = JpegReader::new(&mut self.reader, bytes[idx], &jpeg_tables)?;
            let mut decoder = jpeg::Decoder::new(jpeg_reader);

            match decoder.decode() {
                Ok(mut val) => res_img.append(&mut val),
                Err(e) => {
                    return match e {
                        jpeg::Error::Io(io_err) => Err(TiffError::IoError(io_err)),
                        jpeg::Error::Format(fmt_err) => {
                            Err(TiffError::FormatError(TiffFormatError::Format(fmt_err)))
                        }
                        jpeg::Error::Unsupported(_) => Err(TiffError::UnsupportedError(
                            TiffUnsupportedError::UnknownInterpretation,
                        )),
                        jpeg::Error::Internal(_) => Err(TiffError::UnsupportedError(
                            TiffUnsupportedError::UnknownInterpretation,
                        )),
                    }
                }
            }
        }

        Ok(DecodingResult::U8(res_img))
    }

    pub fn read_strip_to_buffer(&mut self, mut buffer: DecodingBuffer) -> TiffResult<()> {
        self.initialize_strip_decoder()?;

        let index = self.strip_decoder.as_ref().unwrap().strip_index;
        let offset = *self
            .strip_decoder
            .as_ref()
            .unwrap()
            .strip_offsets
            .get(index)
            .ok_or(TiffError::FormatError(
                TiffFormatError::InconsistentSizesEncountered,
            ))?;
        let byte_count = *self
            .strip_decoder
            .as_ref()
            .unwrap()
            .strip_bytes
            .get(index)
            .ok_or(TiffError::FormatError(
                TiffFormatError::InconsistentSizesEncountered,
            ))?;

        let tag_rows = self.get_tag_u32(Tag::RowsPerStrip).unwrap_or(self.height);
        let rows_per_strip = usize::try_from(tag_rows)?;

        let sized_width = usize::try_from(self.width)?;
        let sized_height = usize::try_from(self.height)?;

        let strip_height = cmp::min(rows_per_strip, sized_height - index * rows_per_strip);

        let buffer_size = sized_width * strip_height * self.bits_per_sample.len();

        if buffer.len() < buffer_size {
            return Err(TiffError::FormatError(
                TiffFormatError::InconsistentSizesEncountered,
            ));
        }

        let units_read = self.expand_strip(buffer.copy(), offset, byte_count, buffer_size)?;

        self.strip_decoder.as_mut().unwrap().strip_index += 1;

        if u32::try_from(index)? == self.strip_count()? {
            self.strip_decoder = None;
        }

        if units_read < buffer_size {
            return Err(TiffError::FormatError(
                TiffFormatError::InconsistentStripSamples {
                    actual_samples: units_read,
                    required_samples: buffer_size,
                },
            ));
        }
        if let Ok(predictor) = self.get_tag_unsigned(Tag::Predictor) {
            match Predictor::from_u16(predictor) {
                Some(Predictor::None) => (),
                Some(Predictor::Horizontal) => {
                    rev_hpredict(
                        buffer.copy(),
                        (self.width, u32::try_from(strip_height)?),
                        self.colortype()?,
                    )?;
                }
                None => {
                    return Err(TiffError::FormatError(TiffFormatError::UnknownPredictor(
                        predictor,
                    )))
                }
                Some(Predictor::__NonExhaustive) => unreachable!(),
            }
        }
        Ok(())
    }

    fn result_buffer(&self, height: usize) -> TiffResult<DecodingResult> {
        let buffer_size = usize::try_from(self.width)? * height * self.bits_per_sample.len();

        let max_sample_bits = self.bits_per_sample.iter().cloned().max().unwrap_or(8);
        match self.sample_format.first().unwrap_or(&SampleFormat::Uint) {
            SampleFormat::Uint => match max_sample_bits {
                n if n <= 8 => DecodingResult::new_u8(buffer_size, &self.limits),
                n if n <= 16 => DecodingResult::new_u16(buffer_size, &self.limits),
                n if n <= 32 => DecodingResult::new_u32(buffer_size, &self.limits),
                n if n <= 64 => DecodingResult::new_u64(buffer_size, &self.limits),
                n => Err(TiffError::UnsupportedError(
                    TiffUnsupportedError::UnsupportedBitsPerChannel(n),
                )),
            },
            SampleFormat::IEEEFP => match max_sample_bits {
                32 => DecodingResult::new_f32(buffer_size, &self.limits),
                64 => DecodingResult::new_f64(buffer_size, &self.limits),
                n => Err(TiffError::UnsupportedError(
                    TiffUnsupportedError::UnsupportedBitsPerChannel(n),
                )),
            },
            format => {
                Err(TiffUnsupportedError::UnsupportedSampleFormat(vec![format.clone()]).into())
            }
        }
    }

    /// Read a single strip from the image and return it as a Vector
    pub fn read_strip(&mut self) -> TiffResult<DecodingResult> {
        self.initialize_strip_decoder()?;
        let index = self.strip_decoder.as_ref().unwrap().strip_index;

        let rows_per_strip =
            usize::try_from(self.get_tag_u32(Tag::RowsPerStrip).unwrap_or(self.height))?;

        let strip_height = cmp::min(
            rows_per_strip,
            usize::try_from(self.height)? - index * rows_per_strip,
        );

        let mut result = self.result_buffer(strip_height)?;

        self.read_strip_to_buffer(result.as_buffer(0))?;

        Ok(result)
    }

    /// Decodes the entire image and return it as a Vector
    pub fn read_image(&mut self) -> TiffResult<DecodingResult> {
        if self.compression_method == CompressionMethod::ModernJPEG {
            return self.read_jpeg();
        }

        self.initialize_strip_decoder()?;
        let rows_per_strip =
            usize::try_from(self.get_tag_u32(Tag::RowsPerStrip).unwrap_or(self.height))?;

        let samples_per_strip =
            usize::try_from(self.width)? * rows_per_strip * self.bits_per_sample.len();

        let mut result = self.result_buffer(usize::try_from(self.height)?)?;

        for i in 0..usize::try_from(self.strip_count()?)? {
            self.read_strip_to_buffer(result.as_buffer(samples_per_strip * i))?;
        }
        Ok(result)
    }
}
