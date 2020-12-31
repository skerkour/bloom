//! All IO functionality needed for TIFF decoding

use crate::bytecast;
use crate::error::{TiffError, TiffResult};
use miniz_oxide::inflate;
use std::io::{self, Read, Seek};

/// Byte order of the TIFF file.
#[derive(Clone, Copy, Debug)]
pub enum ByteOrder {
    /// little endian byte order
    LittleEndian,
    /// big endian byte order
    BigEndian,
}

/// Reader that is aware of the byte order.
pub trait EndianReader: Read {
    /// Byte order that should be adhered to
    fn byte_order(&self) -> ByteOrder;

    /// Reads an u16
    #[inline(always)]
    fn read_u16(&mut self) -> Result<u16, io::Error> {
        let mut n = [0u8; 2];
        self.read_exact(&mut n)?;
        Ok(match self.byte_order() {
            ByteOrder::LittleEndian => u16::from_le_bytes(n),
            ByteOrder::BigEndian => u16::from_be_bytes(n),
        })
    }

    #[inline(always)]
    fn read_u16_into(&mut self, buffer: &mut [u16]) -> Result<(), io::Error> {
        self.read_exact(bytecast::u16_as_ne_mut_bytes(buffer))?;
        match self.byte_order() {
            ByteOrder::LittleEndian => {
                for n in buffer {
                    *n = u16::from_le(*n);
                }
            }
            ByteOrder::BigEndian => {
                for n in buffer {
                    *n = u16::from_be(*n);
                }
            }
        }
        Ok(())
    }

    /// Reads an i16
    #[inline(always)]
    fn read_i16(&mut self) -> Result<i16, io::Error> {
        let mut n = [0u8; 2];
        self.read_exact(&mut n)?;
        Ok(match self.byte_order() {
            ByteOrder::LittleEndian => i16::from_le_bytes(n),
            ByteOrder::BigEndian => i16::from_be_bytes(n),
        })
    }

    /// Reads an u32
    #[inline(always)]
    fn read_u32(&mut self) -> Result<u32, io::Error> {
        let mut n = [0u8; 4];
        self.read_exact(&mut n)?;
        Ok(match self.byte_order() {
            ByteOrder::LittleEndian => u32::from_le_bytes(n),
            ByteOrder::BigEndian => u32::from_be_bytes(n),
        })
    }

    #[inline(always)]
    fn read_u32_into(&mut self, buffer: &mut [u32]) -> Result<(), io::Error> {
        self.read_exact(bytecast::u32_as_ne_mut_bytes(buffer))?;
        match self.byte_order() {
            ByteOrder::LittleEndian => {
                for n in buffer {
                    *n = u32::from_le(*n);
                }
            }
            ByteOrder::BigEndian => {
                for n in buffer {
                    *n = u32::from_be(*n);
                }
            }
        }
        Ok(())
    }

    /// Reads an i32
    #[inline(always)]
    fn read_i32(&mut self) -> Result<i32, io::Error> {
        let mut n = [0u8; 4];
        self.read_exact(&mut n)?;
        Ok(match self.byte_order() {
            ByteOrder::LittleEndian => i32::from_le_bytes(n),
            ByteOrder::BigEndian => i32::from_be_bytes(n),
        })
    }

    /// Reads an u64
    #[inline(always)]
    fn read_u64(&mut self) -> Result<u64, io::Error> {
        let mut n = [0u8; 8];
        self.read_exact(&mut n)?;
        Ok(match self.byte_order() {
            ByteOrder::LittleEndian => u64::from_le_bytes(n),
            ByteOrder::BigEndian => u64::from_be_bytes(n),
        })
    }

    #[inline(always)]
    fn read_u64_into(&mut self, buffer: &mut [u64]) -> Result<(), io::Error> {
        self.read_exact(bytecast::u64_as_ne_mut_bytes(buffer))?;
        match self.byte_order() {
            ByteOrder::LittleEndian => {
                for n in buffer {
                    *n = u64::from_le(*n);
                }
            }
            ByteOrder::BigEndian => {
                for n in buffer {
                    *n = u64::from_be(*n);
                }
            }
        }
        Ok(())
    }

    /// Reads an f32
    #[inline(always)]
    fn read_f32(&mut self) -> Result<f32, io::Error> {
        let mut n = [0u8; 4];
        self.read_exact(&mut n)?;
        Ok(f32::from_bits(match self.byte_order() {
            ByteOrder::LittleEndian => u32::from_le_bytes(n),
            ByteOrder::BigEndian => u32::from_be_bytes(n),
        }))
    }

    #[inline(always)]
    fn read_f32_into(&mut self, buffer: &mut [f32]) -> Result<(), io::Error> {
        self.read_exact(bytecast::f32_as_ne_mut_bytes(buffer))?;
        match self.byte_order() {
            ByteOrder::LittleEndian => {
                for n in buffer {
                    *n = f32::from_bits(u32::from_le(n.to_bits()));
                }
            }
            ByteOrder::BigEndian => {
                for n in buffer {
                    *n = f32::from_bits(u32::from_be(n.to_bits()));
                }
            }
        }
        Ok(())
    }

    /// Reads an f64
    #[inline(always)]
    fn read_f64(&mut self) -> Result<f64, io::Error> {
        let mut n = [0u8; 8];
        self.read_exact(&mut n)?;
        Ok(f64::from_bits(match self.byte_order() {
            ByteOrder::LittleEndian => u64::from_le_bytes(n),
            ByteOrder::BigEndian => u64::from_be_bytes(n),
        }))
    }

    #[inline(always)]
    fn read_f64_into(&mut self, buffer: &mut [f64]) -> Result<(), io::Error> {
        self.read_exact(bytecast::f64_as_ne_mut_bytes(buffer))?;
        match self.byte_order() {
            ByteOrder::LittleEndian => {
                for n in buffer {
                    *n = f64::from_bits(u64::from_le(n.to_bits()));
                }
            }
            ByteOrder::BigEndian => {
                for n in buffer {
                    *n = f64::from_bits(u64::from_be(n.to_bits()));
                }
            }
        }
        Ok(())
    }
}

///
/// # READERS
///

///
/// ## Deflate Reader
///

/// Reader that decompresses DEFLATE streams
pub struct DeflateReader {
    buffer: io::Cursor<Vec<u8>>,
    byte_order: ByteOrder,
}

impl DeflateReader {
    pub fn new<R: Read + Seek>(
        reader: &mut SmartReader<R>,
        max_uncompressed_length: usize,
    ) -> TiffResult<(usize, Self)> {
        let byte_order = reader.byte_order;
        let mut compressed = Vec::new();
        reader.read_to_end(&mut compressed)?;

        // TODO: Implement streaming compression, and remove this (temporary) and somewhat
        // misleading workaround.
        if compressed.len() > max_uncompressed_length {
            return Err(TiffError::LimitsExceeded);
        }

        let uncompressed =
            inflate::decompress_to_vec_zlib(&compressed).map_err(TiffError::from_inflate_status)?;

        Ok((
            uncompressed.len(),
            Self {
                byte_order,
                buffer: io::Cursor::new(uncompressed),
            },
        ))
    }
}

impl Read for DeflateReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.buffer.read(buf)
    }
}

impl EndianReader for DeflateReader {
    fn byte_order(&self) -> ByteOrder {
        self.byte_order
    }
}

///
/// ## LZW Reader
///

/// Reader that decompresses LZW streams
pub struct LZWReader {
    buffer: io::Cursor<Vec<u8>>,
    byte_order: ByteOrder,
}

impl LZWReader {
    /// Wraps a reader
    pub fn new<R>(
        reader: &mut SmartReader<R>,
        compressed_length: usize,
        max_uncompressed_length: usize,
    ) -> io::Result<(usize, LZWReader)>
    where
        R: Read + Seek,
    {
        let order = reader.byte_order;
        let mut compressed = vec![0; compressed_length as usize];
        reader.read_exact(&mut compressed[..])?;
        let mut uncompressed = Vec::with_capacity(max_uncompressed_length);
        let mut decoder = weezl::decode::Decoder::with_tiff_size_switch(weezl::BitOrder::Msb, 8);
        let mut bytes_read = 0;

        while uncompressed.len() < max_uncompressed_length {
            let bytes_written = uncompressed.len();
            uncompressed.reserve(1 << 12);
            let buffer_space = uncompressed.capacity().min(max_uncompressed_length);
            uncompressed.resize(buffer_space, 0u8);

            let result = decoder.decode_bytes(
                &compressed[bytes_read..],
                &mut uncompressed[bytes_written..],
            );
            bytes_read += result.consumed_in;
            uncompressed.truncate(bytes_written + result.consumed_out);

            match result.status {
                Ok(weezl::LzwStatus::Ok) => {}
                Ok(weezl::LzwStatus::Done) => break,
                Ok(weezl::LzwStatus::NoProgress) => {
                    return Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "no lzw end code found",
                    ))
                }
                Err(err) => return Err(io::Error::new(io::ErrorKind::InvalidData, err)),
            }
        }

        let bytes = uncompressed.len();
        Ok((
            bytes,
            LZWReader {
                buffer: io::Cursor::new(uncompressed),
                byte_order: order,
            },
        ))
    }
}

impl Read for LZWReader {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.buffer.read(buf)
    }
}

impl EndianReader for LZWReader {
    #[inline(always)]
    fn byte_order(&self) -> ByteOrder {
        self.byte_order
    }
}

///
/// ## JPEG Reader (for "new-style" JPEG format (TIFF compression tag 7))
///

pub(crate) struct JpegReader {
    buffer: io::Cursor<Vec<u8>>,
    byte_order: ByteOrder,
}
impl JpegReader {
    /// Constructs new JpegReader wrapping a SmartReader.
    /// Because JPEG compression in TIFF allows to save quantization and/or huffman tables in one
    /// central location, the constructor accepts this data as `jpeg_tables` here containing either
    /// or both.
    /// These `jpeg_tables` are simply prepended to the remaining jpeg image data.
    /// Because these `jpeg_tables` start with a `SOI` (HEX: `0xFFD8`) or __start of image__ marker
    /// which is also at the beginning of the remaining JPEG image data and would
    /// confuse the JPEG renderer, one of these has to be taken off. In this case the first two
    /// bytes of the remaining JPEG data is removed because it follows `jpeg_tables`.
    /// Similary, `jpeg_tables` ends with a `EOI` (HEX: `0xFFD9`) or __end of image__ marker,
    /// this has to be removed as well (last two bytes of `jpeg_tables`).

    pub fn new<R>(
        reader: &mut SmartReader<R>,
        length: u32,
        jpeg_tables: &Option<Vec<u8>>,
    ) -> io::Result<JpegReader>
    where
        R: Read + Seek,
    {
        let order = reader.byte_order;

        // Read jpeg image data
        let mut segment = vec![0; length as usize];
        reader.read_exact(&mut segment[..])?;

        match jpeg_tables {
            Some(tables) => {
                let mut jpeg_data = tables.clone();
                let truncated_length = jpeg_data.len() - 2;
                jpeg_data.truncate(truncated_length);
                jpeg_data.extend_from_slice(&mut segment[2..]);

                Ok(JpegReader {
                    buffer: io::Cursor::new(jpeg_data),
                    byte_order: order,
                })
            }
            None => Ok(JpegReader {
                buffer: io::Cursor::new(segment),
                byte_order: order,
            }),
        }
    }
}

impl Read for JpegReader {
    // #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.buffer.read(buf)
    }
}

impl EndianReader for JpegReader {
    #[inline(always)]
    fn byte_order(&self) -> ByteOrder {
        self.byte_order
    }
}

///
/// ## PackBits Reader
///

/// Reader that unpacks Apple's `PackBits` format
pub struct PackBitsReader {
    buffer: io::Cursor<Vec<u8>>,
    byte_order: ByteOrder,
}

impl PackBitsReader {
    /// Wraps a reader
    pub fn new<R: Read + Seek>(
        mut reader: R,
        byte_order: ByteOrder,
        length: usize,
    ) -> io::Result<(usize, PackBitsReader)> {
        let mut buffer = Vec::new();
        let mut read: usize = 0;
        while read < length {
            let lread = read_packbits_run(&mut reader, &mut buffer)?;
            if lread == 0 {
                return Err(io::ErrorKind::UnexpectedEof.into());
            }
            read += lread;
        }
        Ok((
            buffer.len(),
            PackBitsReader {
                buffer: io::Cursor::new(buffer),
                byte_order,
            },
        ))
    }
}

fn read_packbits_run<R: Read + Seek>(reader: &mut R, buffer: &mut Vec<u8>) -> io::Result<usize> {
    let mut header: [u8; 1] = [0];

    let bytes = reader.read(&mut header)?;

    match bytes {
        0 => Ok(0),
        _ => match header[0] as i8 {
            -128 => Ok(1),
            h if h >= -127 && h <= -1 => {
                let new_len = buffer.len() + (1 - h as isize) as usize;
                reader.read_exact(&mut header)?;
                buffer.resize(new_len, header[0]);
                Ok(2)
            }
            h => {
                let num_vals = h as usize + 1;
                let start = buffer.len();
                buffer.resize(start + num_vals, 0);
                reader.read_exact(&mut buffer[start..])?;
                Ok(num_vals + 1)
            }
        },
    }
}

impl Read for PackBitsReader {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.buffer.read(buf)
    }
}

impl EndianReader for PackBitsReader {
    #[inline(always)]
    fn byte_order(&self) -> ByteOrder {
        self.byte_order
    }
}

///
/// ## SmartReader Reader
///

/// Reader that is aware of the byte order.
#[derive(Debug)]
pub struct SmartReader<R>
where
    R: Read + Seek,
{
    reader: R,
    pub byte_order: ByteOrder,
}

impl<R> SmartReader<R>
where
    R: Read + Seek,
{
    /// Wraps a reader
    pub fn wrap(reader: R, byte_order: ByteOrder) -> SmartReader<R> {
        SmartReader { reader, byte_order }
    }
}

impl<R> EndianReader for SmartReader<R>
where
    R: Read + Seek,
{
    #[inline(always)]
    fn byte_order(&self) -> ByteOrder {
        self.byte_order
    }
}

impl<R: Read + Seek> Read for SmartReader<R> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.reader.read(buf)
    }
}

impl<R: Read + Seek> Seek for SmartReader<R> {
    #[inline]
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        self.reader.seek(pos)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_packbits() {
        let encoded = vec![
            0xFE, 0xAA, 0x02, 0x80, 0x00, 0x2A, 0xFD, 0xAA, 0x03, 0x80, 0x00, 0x2A, 0x22, 0xF7,
            0xAA,
        ];
        let encoded_len = encoded.len();

        let buff = io::Cursor::new(encoded);
        let (_, mut decoder) =
            PackBitsReader::new(buff, ByteOrder::LittleEndian, encoded_len).unwrap();

        let mut decoded = Vec::new();
        decoder.read_to_end(&mut decoded).unwrap();

        let expected = vec![
            0xAA, 0xAA, 0xAA, 0x80, 0x00, 0x2A, 0xAA, 0xAA, 0xAA, 0xAA, 0x80, 0x00, 0x2A, 0x22,
            0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA, 0xAA,
        ];
        assert_eq!(decoded, expected);
    }
}
