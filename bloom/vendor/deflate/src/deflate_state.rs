use std::io::Write;
use std::{cmp, io, mem};

use crate::compress::Flush;
use crate::compression_options::{CompressionOptions, MAX_HASH_CHECKS};
use crate::encoder_state::EncoderState;
pub use crate::huffman_table::MAX_MATCH;
use crate::huffman_table::NUM_LITERALS_AND_LENGTHS;
use crate::input_buffer::InputBuffer;
use crate::length_encode::{EncodedLength, LeafVec};
use crate::lz77::LZ77State;
use crate::output_writer::DynamicWriter;

/// A counter used for checking values in debug mode.
/// Does nothing when debug assertions are disabled.
#[derive(Default)]
pub struct DebugCounter {
    #[cfg(debug_assertions)]
    count: u64,
}

impl DebugCounter {
    #[cfg(debug_assertions)]
    pub fn get(&self) -> u64 {
        self.count
    }

    #[cfg(not(debug_assertions))]
    pub fn get(&self) -> u64 {
        0
    }

    #[cfg(debug_assertions)]
    pub fn reset(&mut self) {
        self.count = 0;
    }

    #[cfg(not(debug_assertions))]
    pub fn reset(&self) {}

    #[cfg(debug_assertions)]
    pub fn add(&mut self, val: u64) {
        self.count += val;
    }

    #[cfg(not(debug_assertions))]
    pub fn add(&self, _: u64) {}
}

pub struct LengthBuffers {
    pub leaf_buf: LeafVec,
    pub length_buf: Vec<EncodedLength>,
}

impl LengthBuffers {
    #[inline]
    fn new() -> LengthBuffers {
        LengthBuffers {
            leaf_buf: Vec::with_capacity(NUM_LITERALS_AND_LENGTHS),
            length_buf: Vec::with_capacity(19),
        }
    }
}

/// A struct containing all the stored state used for the encoder.
pub struct DeflateState<W: Write> {
    /// State of lz77 compression.
    pub lz77_state: LZ77State,
    pub input_buffer: InputBuffer,
    pub compression_options: CompressionOptions,
    /// State the Huffman part of the compression and the output buffer.
    pub encoder_state: EncoderState,
    /// The buffer containing the raw output of the lz77-encoding.
    pub lz77_writer: DynamicWriter,
    /// Buffers used when generating Huffman code lengths.
    pub length_buffers: LengthBuffers,
    /// Total number of bytes consumed/written to the input buffer.
    pub bytes_written: u64,
    /// Wrapped writer.
    /// Option is used to allow us to implement `Drop` and `finish()` at the same time for the
    /// writer structs.
    pub inner: Option<W>,
    /// The position in the output buffer where data should be flushed from, to keep track of
    /// what data has been output in case not all data is output when writing to the wrapped
    /// writer.
    pub output_buf_pos: usize,
    pub flush_mode: Flush,
    /// Whether we need to flush everything before continuing.
    /// Currently only used after having output a sync flush.
    /// This is implemented in a somewhat clunky manner at the moment,
    /// ideally it should be done in a more fail-safe way to avoid
    /// further bugs.
    pub needs_flush: bool,
    /// Number of bytes written as calculated by sum of block input lengths.
    /// Used to check that they are correct when `debug_assertions` are enabled.
    pub bytes_written_control: DebugCounter,
}

impl<W: Write> DeflateState<W> {
    pub fn new(compression_options: CompressionOptions, writer: W) -> DeflateState<W> {
        DeflateState {
            input_buffer: InputBuffer::empty(),
            lz77_state: LZ77State::new(
                compression_options.max_hash_checks,
                cmp::min(compression_options.lazy_if_less_than, MAX_HASH_CHECKS),
                compression_options.matching_type,
            ),
            encoder_state: EncoderState::new(Vec::with_capacity(1024 * 32)),
            lz77_writer: DynamicWriter::new(),
            length_buffers: LengthBuffers::new(),
            compression_options,
            bytes_written: 0,
            inner: Some(writer),
            output_buf_pos: 0,
            flush_mode: Flush::None,
            needs_flush: false,
            bytes_written_control: DebugCounter::default(),
        }
    }

    #[inline]
    pub fn output_buf(&mut self) -> &mut Vec<u8> {
        self.encoder_state.inner_vec()
    }

    /// Resets the status of the decoder, leaving the compression options intact
    ///
    /// If flushing the current writer succeeds, it is replaced with the provided one,
    /// buffers and status (except compression options) is reset and the old writer
    /// is returned.
    ///
    /// If flushing fails, the rest of the writer is not cleared.
    pub fn reset(&mut self, writer: W) -> io::Result<W> {
        self.encoder_state.flush();
        self.inner
            .as_mut()
            .expect("Missing writer!")
            .write_all(self.encoder_state.inner_vec())?;
        self.encoder_state.inner_vec().clear();
        self.input_buffer = InputBuffer::empty();
        self.lz77_writer.clear();
        self.lz77_state.reset();
        self.bytes_written = 0;
        self.output_buf_pos = 0;
        self.flush_mode = Flush::None;
        self.needs_flush = false;
        if cfg!(debug_assertions) {
            self.bytes_written_control.reset();
        }
        mem::replace(&mut self.inner, Some(writer))
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Missing writer"))
    }
}
