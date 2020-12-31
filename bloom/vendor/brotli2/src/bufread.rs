//! I/O streams for wrapping `BufRead` types as encoders/decoders

use std::io::prelude::*;
use std::io;

use super::CompressParams;
use raw::{self, Decompress, DeStatus, Compress, CompressOp, CoStatus};

#[derive(Clone, Copy, Eq, PartialEq)]
enum DoneStatus {
    Processing,
    Finishing,
    Done,
}

/// A brotli encoder, or compressor.
///
/// This structure implements a `BufRead` interface and will read uncompressed
/// data from an underlying stream and emit a stream of compressed data.
pub struct BrotliEncoder<R: BufRead> {
    obj: R,
    data: Compress,
    done: DoneStatus,
    err: Option<raw::Error>,
}

/// A brotli decoder, or decompressor.
///
/// This structure implements a `BufRead` interface and takes a stream of
/// compressed data as input, providing the decompressed data when read from.
pub struct BrotliDecoder<R: BufRead> {
    obj: R,
    data: Decompress,
    err: Option<raw::Error>,
}

impl<R: BufRead> BrotliEncoder<R> {
    /// Creates a new encoder which will read uncompressed data from the given
    /// stream and emit the compressed stream.
    ///
    /// The `level` argument here is typically 0-11.
    pub fn new(r: R, level: u32) -> BrotliEncoder<R> {
        let mut data = Compress::new();
        data.set_params(CompressParams::new().quality(level));
        BrotliEncoder {
            obj: r,
            data: data,
            done: DoneStatus::Processing,
            err: None,
        }
    }

    /// Creates a new encoder with a custom `CompressParams`.
    pub fn from_params(r: R, params: &CompressParams) -> BrotliEncoder<R> {
        let mut data = Compress::new();
        data.set_params(params);
        BrotliEncoder {
            obj: r,
            data: data,
            done: DoneStatus::Processing,
            err: None,
        }
    }

    /// Acquires a reference to the underlying stream
    pub fn get_ref(&self) -> &R {
        &self.obj
    }

    /// Acquires a mutable reference to the underlying stream
    ///
    /// Note that mutation of the stream may result in surprising results if
    /// this encoder is continued to be used.
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.obj
    }

    /// Consumes this encoder, returning the underlying reader.
    pub fn into_inner(self) -> R {
        self.obj
    }
}

impl<R: BufRead> Read for BrotliEncoder<R> {
    fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
        if buf.is_empty() { return Ok(0) }
        // If the compressor has failed at some point, this is set.
        // Unfortunately we have no idea what status is in the compressor
        // was in when it failed so we can't do anything except bail again.
        if let Some(ref err) = self.err {
            return Err(err.clone().into())
        }

        if let Some(data) = self.data.take_output(Some(buf.len())) {
            buf[..data.len()].copy_from_slice(data);
            return Ok(data.len())
        }

        match self.done {
            DoneStatus::Done => return Ok(0),
            DoneStatus::Finishing => return tryfinish(self, buf),
            DoneStatus::Processing => (),
        }

        loop {
            let amt_in;
            let amt_out;
            {
                let input = &mut try!(self.obj.fill_buf());
                let avail_in = input.len();
                if avail_in == 0 {
                    break
                }
                let output = &mut buf;
                let avail_out = output.len();
                if let Err(err) = self.data.compress(CompressOp::Process, input, output) {
                    self.err = Some(err.clone().into());
                    return Err(err.into())
                }
                amt_in = avail_in - input.len();
                amt_out = avail_out - output.len();
            }
            self.obj.consume(amt_in);

            if amt_out == 0 {
                assert!(amt_in != 0);
                continue
            }
            return Ok(amt_out)
        }
        self.done = DoneStatus::Finishing;
        return tryfinish(self, buf);

        fn tryfinish<R: BufRead>(enc: &mut BrotliEncoder<R>, mut buf: &mut [u8])
                -> io::Result<usize> {
            let output = &mut buf;
            let avail_out = output.len();
            let iscomplete = match enc.data.compress(CompressOp::Finish, &mut &[][..], output) {
                Ok(c) => c,
                Err(err) => {
                    enc.err = err.clone().into();
                    return Err(err.into())
                },
            };
            let written = avail_out - output.len();
            assert!(written != 0 || iscomplete == CoStatus::Finished);
            if iscomplete == CoStatus::Finished {
                enc.done = DoneStatus::Done
            }
            Ok(written)
        }
    }
}

impl<R: BufRead> BrotliDecoder<R> {
    /// Creates a new decoder which will decompress data read from the given
    /// stream.
    pub fn new(r: R) -> BrotliDecoder<R> {
        BrotliDecoder {
            data: Decompress::new(),
            obj: r,
            err: None,
        }
    }

    /// Acquires a reference to the underlying stream
    pub fn get_ref(&self) -> &R {
        &self.obj
    }

    /// Acquires a mutable reference to the underlying stream
    ///
    /// Note that mutation of the stream may result in surprising results if
    /// this encoder is continued to be used.
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.obj
    }

    /// Consumes this decoder, returning the underlying reader.
    pub fn into_inner(self) -> R {
        self.obj
    }
}

impl<R: BufRead> Read for BrotliDecoder<R> {
    fn read(&mut self, mut buf: &mut [u8]) -> io::Result<usize> {
        if buf.is_empty() { return Ok(0) }
        // If the decompressor has failed at some point, this is set.
        // Unfortunately we have no idea what status is in the compressor
        // was in when it failed so we can't do anything except bail again.
        if let Some(ref err) = self.err {
            return Err(err.clone().into())
        }

        loop {
            let amt_in;
            let amt_out;
            let status;
            {
                let mut input = try!(self.obj.fill_buf());
                let avail_in = input.len();
                let avail_out = buf.len();
                status = match self.data.decompress(&mut input, &mut buf) {
                    Ok(s) => s,
                    Err(err) => {
                        self.err = Some(err.clone().into());
                        return Err(err.into())
                    },
                };
                amt_in = avail_in - input.len();
                amt_out = avail_out - buf.len()
            }
            self.obj.consume(amt_in);

            if amt_in == 0 && status == DeStatus::NeedInput {
                return Err(io::Error::new(io::ErrorKind::Other,
                                          "corrupted brotli stream"))
            }
            if amt_out == 0 && status != DeStatus::Finished {
                assert!(amt_in != 0);
                continue
            }

            return Ok(amt_out)
        }
    }
}


