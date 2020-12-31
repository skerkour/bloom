//! Brotli Compression/Decompression for Rust
//!
//! This crate is a binding to the [official brotli implementation][brotli] and
//! provides in-memory and I/O streams for Rust wrappers.
//!
//! [brotli]: https://github.com/google/brotli
//!
//! # Examples
//!
//! ```
//! use std::io::prelude::*;
//! use brotli2::read::{BrotliEncoder, BrotliDecoder};
//!
//! // Round trip some bytes from a byte source, into a compressor, into a
//! // decompressor, and finally into a vector.
//! let data = "Hello, World!".as_bytes();
//! let compressor = BrotliEncoder::new(data, 9);
//! let mut decompressor = BrotliDecoder::new(compressor);
//!
//! let mut contents = String::new();
//! decompressor.read_to_string(&mut contents).unwrap();
//! assert_eq!(contents, "Hello, World!");
//! ```

#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/brotli2/0.2")]

extern crate brotli_sys;
extern crate libc;

#[cfg(test)]
extern crate rand;
#[cfg(test)]
extern crate quickcheck;

pub mod raw;
pub mod bufread;
pub mod read;
pub mod write;

/// Possible choices for modes of compression
#[repr(isize)]
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum CompressMode {
    /// Default compression mode, the compressor does not know anything in
    /// advance about the properties of the input.
    Generic = brotli_sys::BROTLI_MODE_GENERIC as isize,
    /// Compression mode for utf-8 formatted text input.
    Text = brotli_sys::BROTLI_MODE_TEXT as isize,
    /// Compression mode in WOFF 2.0.
    Font = brotli_sys::BROTLI_MODE_FONT as isize,
}

/// Parameters passed to various compression routines.
#[derive(Clone,Debug)]
pub struct CompressParams {
    /// Compression mode.
    mode: u32,
    /// Controls the compression-speed vs compression-density tradeoffs. The higher the `quality`,
    /// the slower the compression. Range is 0 to 11.
    quality: u32,
    /// Base 2 logarithm of the sliding window size. Range is 10 to 24.
    lgwin: u32,
    /// Base 2 logarithm of the maximum input block size. Range is 16 to 24. If set to 0, the value
    /// will be set based on the quality.
    lgblock: u32,
}

impl CompressParams {
    /// Creates a new default set of compression parameters.
    pub fn new() -> CompressParams {
        CompressParams {
            mode: brotli_sys::BROTLI_DEFAULT_MODE,
            quality: brotli_sys::BROTLI_DEFAULT_QUALITY,
            lgwin: brotli_sys::BROTLI_DEFAULT_WINDOW,
            lgblock: 0,
        }
    }

    /// Set the mode of this compression.
    pub fn mode(&mut self, mode: CompressMode) -> &mut CompressParams {
        self.mode = mode as u32;
        self
    }

    /// Controls the compression-speed vs compression-density tradeoffs.
    ///
    /// The higher the quality, the slower the compression. Currently the range
    /// for the quality is 0 to 11.
    pub fn quality(&mut self, quality: u32) -> &mut CompressParams {
        self.quality = quality;
        self
    }

    /// Sets the base 2 logarithm of the sliding window size.
    ///
    /// Currently the range is 10 to 24.
    pub fn lgwin(&mut self, lgwin: u32) -> &mut CompressParams {
        self.lgwin = lgwin;
        self
    }

    /// Sets the base 2 logarithm of the maximum input block size.
    ///
    /// Currently the range is 16 to 24, and if set to 0 the value will be set
    /// based on the quality.
    pub fn lgblock(&mut self, lgblock: u32) -> &mut CompressParams {
        self.lgblock = lgblock;
        self
    }

    /// Get the current block size
    #[inline]
    pub fn get_lgblock_readable(&self) -> usize {
        1usize << self.lgblock
    }

    /// Get the native lgblock size
    #[inline]
    pub fn get_lgblock(&self) -> u32 {
        self.lgblock.clone()
    }
    /// Get the current window size
    #[inline]
    pub fn get_lgwin_readable(&self) -> usize {
        1usize << self.lgwin
    }
    /// Get the native lgwin value
    #[inline]
    pub fn get_lgwin(&self) -> u32 {
        self.lgwin.clone()
    }
}
