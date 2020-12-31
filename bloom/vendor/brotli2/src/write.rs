//! Writer-based compression/decompression streams

use std::io::prelude::*;
use std::io;

use raw::{self, Decompress, DeStatus, Compress, CompressOp, CoStatus};

use super::CompressParams;

const BUF_SIZE: usize = 32 * 1024;

/// A compression stream which will have uncompressed data written to it and
/// will write compressed data to an output stream.
pub struct BrotliEncoder<W: Write> {
    data: Compress,
    obj: Option<W>,
    buf: Vec<u8>,
    cur: usize,
    err: Option<raw::Error>,
}

/// A compression stream which will have compressed data written to it and
/// will write uncompressed data to an output stream.
pub struct BrotliDecoder<W: Write> {
    data: Decompress,
    obj: Option<W>,
    buf: Vec<u8>,
    cur: usize,
    err: Option<raw::Error>,
}

impl<W: Write> BrotliEncoder<W> {
    /// Create a new compression stream which will compress at the given level
    /// to write compress output to the give output stream.
    pub fn new(obj: W, level: u32) -> BrotliEncoder<W> {
        let mut data = Compress::new();
        data.set_params(CompressParams::new().quality(level));
        BrotliEncoder {
            data: data,
            obj: Some(obj),
            buf: Vec::with_capacity(BUF_SIZE),
            cur: 0,
            err: None,
        }
    }

    /// Creates a new encoder with a custom `CompressParams`.
    pub fn from_params(obj: W, params: &CompressParams) -> BrotliEncoder<W> {
        let mut data = Compress::new();
        data.set_params(params);
        BrotliEncoder {
            data: data,
            obj: Some(obj),
            buf: Vec::with_capacity(BUF_SIZE),
            cur: 0,
            err: None,
        }
    }

    /// Acquires a reference to the underlying writer.
    pub fn get_ref(&self) -> &W {
        self.obj.as_ref().unwrap()
    }

    /// Acquires a mutable reference to the underlying writer.
    ///
    /// Note that mutating the output/input state of the stream may corrupt this
    /// object, so care must be taken when using this method.
    pub fn get_mut(&mut self) -> &mut W {
        self.obj.as_mut().unwrap()
    }

    fn dump(&mut self) -> io::Result<()> {
        loop {
            while !self.buf.is_empty() {
                let amt = try!(self.obj.as_mut().unwrap().write(&self.buf[self.cur..]));
                self.cur += amt;
                if self.cur == self.buf.len() {
                    self.buf.clear();
                    self.cur = 0
                }
            }
            // TODO: if we could peek, the buffer wouldn't be necessary
            if let Some(data) = self.data.take_output(Some(BUF_SIZE)) {
                match self.obj.as_mut().unwrap().write(data) {
                    Ok(n) => self.buf.extend_from_slice(&data[n..]),
                    Err(e) => {
                        self.buf.extend_from_slice(data);
                        return Err(e)
                    }
                }
            } else {
                break
            }
        }
        Ok(())
    }

    // Flush or finish stream, also flushing underlying stream
    fn do_flush_or_finish(&mut self, finish: bool) -> io::Result<()> {
        try!(self.dump());
        let op = if finish { CompressOp::Finish } else { CompressOp::Flush };
        loop {
            let status = match self.data.compress(op, &mut &[][..], &mut &mut [][..]) {
                Ok(s) => s,
                Err(err) => {
                    self.err = Some(err.clone());
                    return Err(err.into())
                },
            };
            let obj = self.obj.as_mut().unwrap();
            while let Some(data) = self.data.take_output(None) {
                try!(obj.write_all(data))
            }
            match status {
                CoStatus::Finished => {
                    try!(obj.flush());
                    return Ok(())
                },
                CoStatus::Unfinished => (),
            }
        }
    }

    /// Consumes this encoder, flushing the output stream.
    ///
    /// This will flush the underlying data stream and then return the contained
    /// writer if the flush succeeded.
    pub fn finish(mut self) -> io::Result<W> {
        try!(self.do_flush_or_finish(true));
        Ok(self.obj.take().unwrap())
    }
}

impl<W: Write> Write for BrotliEncoder<W> {
    fn write(&mut self, mut data: &[u8]) -> io::Result<usize> {
        if data.is_empty() { return Ok(0) }
        // If the decompressor has failed at some point, this is set.
        // Unfortunately we have no idea what status is in the compressor
        // was in when it failed so we can't do anything except bail again.
        if let Some(ref err) = self.err {
            return Err(err.clone().into())
        }
        try!(self.dump());
        // Zero-length output buf to keep it all inside the compressor buffer
        let avail_in = data.len();
        if let Err(err) = self.data.compress(CompressOp::Process, &mut data, &mut &mut [][..]) {
            self.err = Some(err.clone());
            return Err(err.into())
        }
        assert!(avail_in != data.len());
        Ok(avail_in - data.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.do_flush_or_finish(false)
    }
}

impl<W: Write> Drop for BrotliEncoder<W> {
    fn drop(&mut self) {
        if self.obj.is_some() {
            let _ = self.do_flush_or_finish(true);
        }
    }
}

impl<W: Write> BrotliDecoder<W> {
    /// Creates a new decoding stream which will decode all input written to it
    /// into `obj`.
    pub fn new(obj: W) -> BrotliDecoder<W> {
        BrotliDecoder {
            data: Decompress::new(),
            obj: Some(obj),
            buf: Vec::with_capacity(BUF_SIZE),
            cur: 0,
            err: None,
        }
    }

    /// Acquires a reference to the underlying writer.
    pub fn get_ref(&self) -> &W {
        self.obj.as_ref().unwrap()
    }

    /// Acquires a mutable reference to the underlying writer.
    ///
    /// Note that mutating the output/input state of the stream may corrupt this
    /// object, so care must be taken when using this method.
    pub fn get_mut(&mut self) -> &mut W {
        self.obj.as_mut().unwrap()
    }

    fn dump(&mut self) -> io::Result<()> {
        loop {
            while !self.buf.is_empty() {
                let amt = try!(self.obj.as_mut().unwrap().write(&self.buf[self.cur..]));
                self.cur += amt;
                if self.cur == self.buf.len() {
                    self.buf.clear();
                    self.cur = 0
                }
            }
            // TODO: if we could peek, the buffer wouldn't be necessary
            if let Some(data) = self.data.take_output(Some(BUF_SIZE)) {
                self.buf.extend_from_slice(data)
            } else {
                break
            }
        }
        Ok(())
    }

    fn do_finish(&mut self) -> io::Result<()> {
        try!(self.dump());
        loop {
            let status = match self.data.decompress(&mut &[][..], &mut &mut [][..]) {
                Ok(s) => s,
                Err(err) => {
                    self.err = Some(err.clone());
                    return Err(err.into())
                },
            };
            let obj = self.obj.as_mut().unwrap();
            while let Some(data) = self.data.take_output(None) {
                try!(obj.write_all(data))
            }
            match status {
                DeStatus::Finished => {
                    try!(obj.flush());
                    return Ok(())
                },
                // When decoding a truncated file, brotli returns DeStatus::NeedInput.
                // Since we're finishing, we cannot provide more data so this is an
                // error.
                DeStatus::NeedInput => {
                    let msg = "brotli compressed stream is truncated or otherwise corrupt";
                    return Err(io::Error::new(io::ErrorKind::UnexpectedEof, msg))
                },
                DeStatus::NeedOutput => (),
            }
        }
    }

    /// Unwrap the underlying writer, finishing the compression stream.
    pub fn finish(&mut self) -> io::Result<W> {
        try!(self.do_finish());
        Ok(self.obj.take().unwrap())
    }
}

impl<W: Write> Write for BrotliDecoder<W> {
    fn write(&mut self, mut data: &[u8]) -> io::Result<usize> {
        if data.is_empty() { return Ok(0) }
        // If the decompressor has failed at some point, this is set.
        // Unfortunately we have no idea what status is in the compressor
        // was in when it failed so we can't do anything except bail again.
        if let Some(ref err) = self.err {
            return Err(err.clone().into())
        }
        try!(self.dump());
        // Zero-length output buf to keep it all inside the decompressor buffer
        let avail_in = data.len();
        let status = match self.data.decompress(&mut data, &mut &mut [][..]) {
            Ok(s) => s,
            Err(err) => {
                self.err = Some(err.clone());
                return Err(err.into())
            },
        };
        assert!(avail_in != data.len() || status == DeStatus::Finished);
        Ok(avail_in - data.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        try!(self.dump());
        self.obj.as_mut().unwrap().flush()
    }
}

impl<W: Write> Drop for BrotliDecoder<W> {
    fn drop(&mut self) {
        if self.obj.is_some() {
            let _ = self.do_finish();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::prelude::*;
    use std::iter::repeat;
    use super::{BrotliEncoder, BrotliDecoder};

    #[test]
    fn smoke() {
        let d = BrotliDecoder::new(Vec::new());
        let mut c = BrotliEncoder::new(d, 6);
        c.write_all(b"12834").unwrap();
        let s = repeat("12345").take(100000).collect::<String>();
        c.write_all(s.as_bytes()).unwrap();
        let data = c.finish().unwrap().finish().unwrap();
        assert_eq!(&data[0..5], b"12834");
        assert_eq!(data.len(), 500005);
        assert!(format!("12834{}", s).as_bytes() == &*data);
    }

    #[test]
    fn write_empty() {
        let d = BrotliDecoder::new(Vec::new());
        let mut c = BrotliEncoder::new(d, 6);
        c.write(b"").unwrap();
        let data = c.finish().unwrap().finish().unwrap();
        assert_eq!(&data[..], b"");
    }

    #[test]
    fn qc() {
        ::quickcheck::quickcheck(test as fn(_) -> _);

        fn test(v: Vec<u8>) -> bool {
            let w = BrotliDecoder::new(Vec::new());
            let mut w = BrotliEncoder::new(w, 6);
            w.write_all(&v).unwrap();
            v == w.finish().unwrap().finish().unwrap()
        }
    }
}
