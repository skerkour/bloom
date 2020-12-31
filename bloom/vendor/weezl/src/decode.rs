//! A module for all decoding needs.
#[cfg(feature = "std")]
use crate::error::StreamResult;
use crate::error::{BufferResult, LzwError, LzwStatus};
use crate::{BitOrder, Code, StreamBuf, MAX_CODESIZE, MAX_ENTRIES, STREAM_BUF_SIZE};

use crate::alloc::{boxed::Box, vec, vec::Vec};
#[cfg(feature = "std")]
use std::io::{self, BufRead, Write};

/// The state for decoding data with an LZW algorithm.
///
/// The same structure can be utilized with streams as well as your own buffers and driver logic.
/// It may even be possible to mix them if you are sufficiently careful not to lose or skip any
/// already decode data in the process.
pub struct Decoder {
    state: Box<dyn Stateful + Send + 'static>,
}

/// A decoding stream sink.
///
/// See [`Decoder::into_stream`] on how to create this type.
///
/// [`Decoder::into_stream`]: struct.Decoder.html#method.into_stream
#[cfg_attr(
    not(feature = "std"),
    deprecated = "This type is only useful with the `std` feature."
)]
#[cfg_attr(not(feature = "std"), allow(dead_code))]
pub struct IntoStream<'d, W> {
    decoder: &'d mut Decoder,
    writer: W,
    buffer: Option<StreamBuf<'d>>,
    default_size: usize,
}

trait Stateful {
    fn advance(&mut self, inp: &[u8], out: &mut [u8]) -> BufferResult;
    fn has_ended(&self) -> bool;
    /// Ignore an end code and continue decoding (no implied reset).
    fn restart(&mut self);
    /// Reset the decoder to the beginning, dropping all buffers etc.
    fn reset(&mut self);
}

#[derive(Clone)]
struct Link {
    prev: Code,
    byte: u8,
}

#[derive(Default)]
struct MsbBuffer {
    /// A buffer of individual bits. The oldest code is kept in the high-order bits.
    bit_buffer: u64,
    /// A precomputed mask for this code.
    code_mask: u16,
    /// The current code size.
    code_size: u8,
    /// The number of bits in the buffer.
    bits: u8,
}

#[derive(Default)]
struct LsbBuffer {
    /// A buffer of individual bits. The oldest code is kept in the high-order bits.
    bit_buffer: u64,
    /// A precomputed mask for this code.
    code_mask: u16,
    /// The current code size.
    code_size: u8,
    /// The number of bits in the buffer.
    bits: u8,
}

trait CodeBuffer {
    fn new(min_size: u8) -> Self;
    fn reset(&mut self, min_size: u8);
    fn bump_code_size(&mut self);
    /// Retrieve the next symbol, refilling if necessary.
    fn next_symbol(&mut self, inp: &mut &[u8]) -> Option<Code>;
    /// Refill the internal buffer.
    fn refill_bits(&mut self, inp: &mut &[u8]);
    /// Get the next buffered code word.
    fn get_bits(&mut self) -> Option<Code>;
    fn max_code(&self) -> Code;
    fn code_size(&self) -> u8;
}

struct DecodeState<CodeBuffer> {
    /// The original minimum code size.
    min_size: u8,
    /// The table of decoded codes.
    table: Table,
    /// The buffer of decoded data.
    buffer: Buffer,
    /// The link which we are still decoding and its original code.
    last: Option<(Code, Link)>,
    /// The next code entry.
    next_code: Code,
    /// Code to reset all tables.
    clear_code: Code,
    /// Code to signal the end of the stream.
    end_code: Code,
    /// A stored flag if the end code has already appeared.
    has_ended: bool,
    /// If tiff then bumps are a single code sooner.
    is_tiff: bool,
    /// The buffer for decoded words.
    code_buffer: CodeBuffer,
}

struct Buffer {
    bytes: Box<[u8]>,
    read_mark: usize,
    write_mark: usize,
}

struct Table {
    inner: Vec<Link>,
    depths: Vec<u16>,
}

impl Decoder {
    /// Create a new decoder with the specified bit order and symbol size.
    ///
    /// The algorithm for dynamically increasing the code symbol bit width is compatible with the
    /// original specification. In particular you will need to specify an `Lsb` bit oder to decode
    /// the data portion of a compressed `gif` image.
    ///
    /// # Panics
    ///
    /// The `size` needs to be in the interval `2..=12`.
    pub fn new(order: BitOrder, size: u8) -> Self {
        type Boxed = Box<dyn Stateful + Send + 'static>;
        super::assert_code_size(size);
        let state = match order {
            BitOrder::Lsb => Box::new(DecodeState::<LsbBuffer>::new(size)) as Boxed,
            BitOrder::Msb => Box::new(DecodeState::<MsbBuffer>::new(size)) as Boxed,
        };

        Decoder { state }
    }

    /// Create a TIFF compatible decoder with the specified bit order and symbol size.
    ///
    /// The algorithm for dynamically increasing the code symbol bit width is compatible with the
    /// TIFF specification, which is a misinterpretation of the original algorithm for increasing
    /// the code size. It switches one symbol sooner.
    ///
    /// # Panics
    ///
    /// The `size` needs to be in the interval `2..=12`.
    pub fn with_tiff_size_switch(order: BitOrder, size: u8) -> Self {
        type Boxed = Box<dyn Stateful + Send + 'static>;
        super::assert_code_size(size);
        let state = match order {
            BitOrder::Lsb => {
                let mut state = Box::new(DecodeState::<LsbBuffer>::new(size));
                state.is_tiff = true;
                state as Boxed
            }
            BitOrder::Msb => {
                let mut state = Box::new(DecodeState::<MsbBuffer>::new(size));
                state.is_tiff = true;
                state as Boxed
            }
        };

        Decoder { state }
    }

    /// Decode some bytes from `inp` and write result to `out`.
    ///
    /// This will consume a prefix of the input buffer and write decoded output into a prefix of
    /// the output buffer. See the respective fields of the return value for the count of consumed
    /// and written bytes. For the next call You should have adjusted the inputs accordingly.
    ///
    /// The call will try to decode and write as many bytes of output as available. It will be
    /// much more optimized (and avoid intermediate buffering) if it is allowed to write a large
    /// contiguous chunk at once.
    ///
    /// See [`into_stream`] for high-level functions (that are only available with the `std`
    /// feature).
    ///
    /// [`into_stream`]: #method.into_stream
    pub fn decode_bytes(&mut self, inp: &[u8], out: &mut [u8]) -> BufferResult {
        self.state.advance(inp, out)
    }

    /// Construct a decoder into a writer.
    #[cfg(feature = "std")]
    pub fn into_stream<W: Write>(&mut self, writer: W) -> IntoStream<'_, W> {
        IntoStream {
            decoder: self,
            writer,
            buffer: None,
            default_size: STREAM_BUF_SIZE,
        }
    }

    /// Check if the decoding has finished.
    ///
    /// No more output is produced beyond the end code that marked the finish of the stream. The
    /// decoder may have read additional bytes, including padding bits beyond the last code word
    /// but also excess bytes provided.
    pub fn has_ended(&self) -> bool {
        self.state.has_ended()
    }

    /// Ignore an end code and continue.
    ///
    /// This will _not_ reset any of the inner code tables and not have the effect of a clear code.
    /// It will instead continue as if the end code had not been present. If no end code has
    /// occurred then this is a no-op.
    ///
    /// You can test if an end code has occurred with [`has_ended`](#method.has_ended).
    /// FIXME: clarify how this interacts with padding introduced after end code.
    #[allow(dead_code)]
    pub(crate) fn restart(&mut self) {
        self.state.restart();
    }

    /// Reset all internal state.
    ///
    /// This produce a decoder as if just constructed with `new` but taking slightly less work. In
    /// particular it will not deallocate any internal allocations. It will also avoid some
    /// duplicate setup work.
    pub fn reset(&mut self) {
        self.state.reset();
    }
}

#[cfg(feature = "std")]
impl<'d, W: Write> IntoStream<'d, W> {
    /// Decode data from a reader.
    ///
    /// This will read data until the stream is empty or an end marker is reached.
    pub fn decode(&mut self, read: impl BufRead) -> StreamResult {
        self.decode_part(read, false)
    }

    /// Decode data from a reader, requiring an end marker.
    pub fn decode_all(mut self, read: impl BufRead) -> StreamResult {
        self.decode_part(read, true)
    }

    /// Set the size of the intermediate decode buffer.
    ///
    /// A buffer of this size is allocated to hold one part of the decoded stream when no buffer is
    /// available and any decoding method is called. No buffer is allocated if `set_buffer` has
    /// been called. The buffer is reused.
    ///
    /// # Panics
    /// This method panics if `size` is `0`.
    pub fn set_buffer_size(&mut self, size: usize) {
        assert_ne!(size, 0, "Attempted to set empty buffer");
        self.default_size = size;
    }

    /// Use a particular buffer as an intermediate decode buffer.
    ///
    /// Calling this sets or replaces the buffer. When a buffer has been set then it is used
    /// instead of dynamically allocating a buffer. Note that the size of the buffer is critical
    /// for efficient decoding. Some optimization techniques require the buffer to hold one or more
    /// previous decoded words. There is also additional overhead from `write` calls each time the
    /// buffer has been filled.
    ///
    /// # Panics
    /// This method panics if the `buffer` is empty.
    pub fn set_buffer(&mut self, buffer: &'d mut [u8]) {
        assert_ne!(buffer.len(), 0, "Attempted to set empty buffer");
        self.buffer = Some(StreamBuf::Borrowed(buffer));
    }

    fn decode_part(&mut self, mut read: impl BufRead, must_finish: bool) -> StreamResult {
        let IntoStream {
            decoder,
            writer,
            buffer,
            default_size,
        } = self;

        enum Progress {
            Ok,
            Done,
        }

        let mut bytes_read = 0;
        let mut bytes_written = 0;

        // Converting to mutable refs to move into the `once` closure.
        let read_bytes = &mut bytes_read;
        let write_bytes = &mut bytes_written;

        let outbuf: &mut [u8] =
            match { buffer.get_or_insert_with(|| StreamBuf::Owned(vec![0u8; *default_size])) } {
                StreamBuf::Borrowed(slice) => &mut *slice,
                StreamBuf::Owned(vec) => &mut *vec,
            };
        assert!(!outbuf.is_empty());

        let once = move || {
            // Try to grab one buffer of input data.
            let data = read.fill_buf()?;

            // Decode as much of the buffer as fits.
            let result = decoder.decode_bytes(data, &mut outbuf[..]);
            // Do the bookkeeping and consume the buffer.
            *read_bytes += result.consumed_in;
            *write_bytes += result.consumed_out;
            read.consume(result.consumed_in);

            // Handle the status in the result.
            let done = result.status.map_err(|err| {
                io::Error::new(io::ErrorKind::InvalidData, &*format!("{:?}", err))
            })?;

            // Check if we had any new data at all.
            if let LzwStatus::NoProgress = done {
                debug_assert_eq!(
                    result.consumed_out, 0,
                    "No progress means we have not decoded any data"
                );
                // In particular we did not finish decoding.
                if must_finish {
                    return Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "No more data but no end marker detected",
                    ));
                } else {
                    return Ok(Progress::Done);
                }
            }

            // And finish by writing our result.
            // TODO: we may lose data on error (also on status error above) which we might want to
            // deterministically handle so that we don't need to restart everything from scratch as
            // the only recovery strategy. Any changes welcome.
            writer.write_all(&outbuf[..result.consumed_out])?;

            Ok(if let LzwStatus::Done = done {
                Progress::Done
            } else {
                Progress::Ok
            })
        };

        // Decode chunks of input data until we're done.
        let status = core::iter::repeat_with(once)
            // scan+fuse can be replaced with map_while
            .scan((), |(), result| match result {
                Ok(Progress::Ok) => Some(Ok(())),
                Err(err) => Some(Err(err)),
                Ok(Progress::Done) => None,
            })
            .fuse()
            .collect();

        StreamResult {
            bytes_read,
            bytes_written,
            status,
        }
    }
}

impl<C: CodeBuffer> DecodeState<C> {
    fn new(min_size: u8) -> Self {
        DecodeState {
            min_size: min_size,
            table: Table::new(),
            buffer: Buffer::new(),
            last: None,
            clear_code: 1 << min_size,
            end_code: (1 << min_size) + 1,
            next_code: (1 << min_size) + 2,
            has_ended: false,
            is_tiff: false,
            code_buffer: CodeBuffer::new(min_size),
        }
    }

    fn init_tables(&mut self) {
        self.code_buffer.reset(self.min_size);
        self.next_code = (1 << self.min_size) + 2;
        self.table.init(self.min_size);
    }

    fn reset_tables(&mut self) {
        self.code_buffer.reset(self.min_size);
        self.next_code = (1 << self.min_size) + 2;
        self.table.clear(self.min_size);
    }
}

impl<C: CodeBuffer> Stateful for DecodeState<C> {
    fn has_ended(&self) -> bool {
        self.has_ended
    }

    fn restart(&mut self) {
        self.has_ended = false;
    }

    fn reset(&mut self) {
        self.table.init(self.min_size);
        self.buffer.read_mark = 0;
        self.buffer.write_mark = 0;
        self.last = None;
        self.restart();
        self.code_buffer = CodeBuffer::new(self.min_size);
    }

    fn advance(&mut self, mut inp: &[u8], mut out: &mut [u8]) -> BufferResult {
        // Skip everything if there is nothing to do.
        if self.has_ended {
            return BufferResult {
                consumed_in: 0,
                consumed_out: 0,
                status: Ok(LzwStatus::Done),
            };
        }

        // Rough description:
        // We will fill the output slice as much as possible until either there is no more symbols
        // to decode or an end code has been reached. This requires an internal buffer to hold a
        // potential tail of the word corresponding to the last symbol. This tail will then be
        // decoded first before continuing with the regular decoding. The same buffer is required
        // to persist some symbol state across calls.
        //
        // We store the words corresponding to code symbols in an index chain, bytewise, where we
        // push each decoded symbol. (TODO: wuffs shows some success with 8-byte units). This chain
        // is traversed for each symbol when it is decoded and bytes are placed directly into the
        // output slice. In the special case (new_code == next_code) we use an existing decoded
        // version that is present in either the out bytes of this call or in buffer to copy the
        // repeated prefix slice.
        // TODO: I played with a 'decoding cache' to remember the position of long symbols and
        // avoid traversing the chain, doing a copy of memory instead. It did however not lead to
        // a serious improvement. It's just unlikely to both have a long symbol and have that
        // repeated twice in the same output buffer.
        //
        // You will also find the (to my knowledge novel) concept of a _decoding burst_ which
        // gained some >~10% speedup in tests. This is motivated by wanting to use out-of-order
        // execution as much as possible and for this reason have the least possible stress on
        // branch prediction. Our decoding table already gives us a lookahead on symbol lengths but
        // only for re-used codes, not novel ones. This lookahead also makes the loop termination
        // when restoring each byte of the code word perfectly predictable! So a burst is a chunk
        // of code words which are all independent of each other, have known lengths _and_ are
        // guaranteed to fit into the out slice without requiring a buffer. One burst can be
        // decoded in an extremely tight loop.
        //
        // TODO: since words can be at most (1 << MAX_CODESIZE) = 4096 bytes long we could avoid
        // that intermediate buffer at the expense of not always filling the output buffer
        // completely. Alternatively we might follow its chain of precursor states twice. This may
        // be even cheaper if we store more than one byte per link so it really should be
        // evaluated.
        // TODO: if the caller was required to provide the previous last word we could also avoid
        // the buffer for cases where we need it to restore the next code! This could be built
        // backwards compatible by only doing it after an opt-in call that enables the behaviour.

        // Record initial lengths for the result that is returned.
        let o_in = inp.len();
        let o_out = out.len();

        // The code_link is the previously decoded symbol.
        // It's used to link the new code back to its predecessor.
        let mut code_link = None;
        // The status, which is written to on an invalid code.
        let mut status = Ok(LzwStatus::Ok);

        match self.last.take() {
            // No last state? This is the first code after a reset?
            None => {
                match self.next_symbol(&mut inp) {
                    // Plainly invalid code.
                    Some(code) if code > self.next_code => status = Err(LzwError::InvalidCode),
                    // next_code would require an actual predecessor.
                    Some(code) if code == self.next_code => status = Err(LzwError::InvalidCode),
                    // No more symbols available and nothing decoded yet.
                    // Assume that we didn't make progress, this may get reset to Done if we read
                    // some bytes from the input.
                    None => status = Ok(LzwStatus::NoProgress),
                    // Handle a valid code.
                    Some(init_code) => {
                        if init_code == self.clear_code {
                            self.init_tables();
                        } else if init_code == self.end_code {
                            self.has_ended = true;
                            status = Ok(LzwStatus::Done);
                        } else if self.table.is_empty() {
                            // We require an explicit reset.
                            // TODO: allow this to be configured and do the setup implicitly.
                            status = Err(LzwError::InvalidCode);
                        } else {
                            // Reconstruct the first code in the buffer.
                            self.buffer.fill_reconstruct(&self.table, init_code);
                            let link = self.table.at(init_code).clone();
                            code_link = Some((init_code, link));
                        }
                    }
                }
            }
            // Move the tracking state to the stack.
            Some(tup) => code_link = Some(tup),
        };

        // Track an empty `burst` (see below) means we made no progress.
        let mut burst_required_for_progress = false;
        // Restore the previous state, if any.
        if let Some((code, link)) = code_link.take() {
            code_link = Some((code, link));
            let remain = self.buffer.buffer();
            // Check if we can fully finish the buffer.
            if remain.len() > out.len() {
                if out.is_empty() {
                    status = Ok(LzwStatus::NoProgress);
                } else {
                    out.copy_from_slice(&remain[..out.len()]);
                    self.buffer.consume(out.len());
                    out = &mut [];
                }
            } else if remain.is_empty() {
                status = Ok(LzwStatus::NoProgress);
                burst_required_for_progress = true;
            } else {
                let consumed = remain.len();
                out[..consumed].copy_from_slice(remain);
                self.buffer.consume(consumed);
                out = &mut out[consumed..];
                burst_required_for_progress = false;
            }
        }

        // The tracking state for a burst.
        // These are actually initialized later but compiler wasn't smart enough to fully optimize
        // out the init code so that appears outside th loop.
        // TODO: maybe we can make it part of the state but it's dubious if that really gives a
        // benefit over stack usage? Also the slices stored here would need some treatment as we
        // can't infect the main struct with a lifetime.
        let mut burst = [0; 6];
        let mut bytes = [0u16; 6];
        let mut target: [&mut [u8]; 6] = Default::default();
        // A special reference to out slice which holds the last decoded symbol.
        let mut last_decoded: Option<&[u8]> = None;

        while let Some((mut code, mut link)) = code_link.take() {
            if out.is_empty() && !self.buffer.buffer().is_empty() {
                code_link = Some((code, link));
                break;
            }

            let mut burst_size = 0;
            // Ensure the code buffer is full, we're about to request some codes.
            // Note that this also ensures at least one code is in the buffer if any input is left.
            self.refill_bits(&mut inp);
            // A burst is a sequence of decodes that are completely independent of each other. This
            // is the case if neither is an end code, a clear code, or a next code, i.e. we have
            // all of them in the decoding table and thus known their depths, and additionally if
            // we can decode them directly into the output buffer.
            for b in &mut burst {
                // TODO: does it actually make a perf difference to avoid reading new bits here?
                *b = match self.get_bits() {
                    None => break,
                    Some(code) => code,
                };

                // We can commit the previous burst code, and will take a slice from the output
                // buffer. This also avoids the bounds check in the tight loop later.
                if burst_size > 0 {
                    let len = bytes[burst_size - 1];
                    let (into, tail) = out.split_at_mut(usize::from(len));
                    target[burst_size - 1] = into;
                    out = tail;
                }

                // Check that we don't overflow the code size with all codes we burst decode.
                let potential_code = self.next_code + burst_size as u16;
                burst_size += 1;
                if potential_code == self.code_buffer.max_code() - Code::from(self.is_tiff) {
                    break;
                }

                // A burst code can't be special.
                if *b == self.clear_code || *b == self.end_code || *b >= self.next_code {
                    break;
                }

                // Read the code length and check that we can decode directly into the out slice.
                let len = self.table.depths[usize::from(*b)];
                if out.len() < usize::from(len) {
                    break;
                }

                bytes[burst_size - 1] = len;
            }

            // No code left, and no more bytes to fill the buffer.
            if burst_size == 0 {
                if burst_required_for_progress {
                    status = Ok(LzwStatus::NoProgress);
                }
                code_link = Some((code, link));
                break;
            }

            burst_required_for_progress = false;
            // Note that the very last code in the burst buffer doesn't actually belong to the
            // burst itself. TODO: sometimes it could, we just don't differentiate between the
            // breaks and a loop end condition above. That may be a speed advantage?
            let (&new_code, burst) = burst[..burst_size].split_last().unwrap();

            // The very tight loop for restoring the actual burst.
            for (&burst, target) in burst.iter().zip(&mut target[..burst_size - 1]) {
                let cha = self.table.reconstruct(burst, target);
                // TODO: this pushes into a Vec, maybe we can make this cleaner.
                // Theoretically this has a branch and llvm tends to be flaky with code layout for
                // the case of requiring an allocation (which can't occur in practice).
                let new_link = self.table.derive(&link, cha, code);
                self.next_code += 1;
                code = burst;
                link = new_link;
            }

            // Update the slice holding the last decoded word.
            if let Some(new_last) = target[..burst_size - 1].last_mut() {
                let slice = core::mem::replace(new_last, &mut []);
                last_decoded = Some(&*slice);
            }

            // Now handle the special codes.
            if new_code == self.clear_code {
                self.reset_tables();
                last_decoded = None;
                continue;
            }

            if new_code == self.end_code {
                self.has_ended = true;
                status = Ok(LzwStatus::Done);
                last_decoded = None;
                break;
            }

            if new_code > self.next_code {
                status = Err(LzwError::InvalidCode);
                last_decoded = None;
                break;
            }

            let required_len = if new_code == self.next_code {
                self.table.depths[usize::from(code)] + 1
            } else {
                self.table.depths[usize::from(new_code)]
            };

            let cha;
            let is_in_buffer;
            // Check if we will need to store our current state into the buffer.
            if usize::from(required_len) > out.len() {
                is_in_buffer = true;
                if new_code == self.next_code {
                    // last_decoded will be Some if we have restored any code into the out slice.
                    // Otherwise it will still be present in the buffer.
                    if let Some(last) = last_decoded.take() {
                        self.buffer.bytes[..last.len()].copy_from_slice(last);
                        self.buffer.write_mark = last.len();
                        self.buffer.read_mark = last.len();
                    }

                    cha = self.buffer.fill_cscsc();
                } else {
                    // Restore the decoded word into the buffer.
                    last_decoded = None;
                    cha = self.buffer.fill_reconstruct(&self.table, new_code);
                }
            } else {
                is_in_buffer = false;
                let (target, tail) = out.split_at_mut(usize::from(required_len));
                out = tail;

                if new_code == self.next_code {
                    // Reconstruct high.
                    let source = match last_decoded.take() {
                        Some(last) => last,
                        None => &self.buffer.bytes[..self.buffer.write_mark],
                    };
                    cha = source[0];
                    target[..source.len()].copy_from_slice(source);
                    target[source.len()..][0] = source[0];
                } else {
                    cha = self.table.reconstruct(new_code, target);
                }

                // A new decoded word.
                last_decoded = Some(target);
            }

            let new_link;
            // Each newly read code creates one new code/link based on the preceding code if we
            // have enough space to put it there.
            if !self.table.is_full() {
                let link = self.table.derive(&link, cha, code);

                if self.next_code == self.code_buffer.max_code() - Code::from(self.is_tiff)
                    && self.code_buffer.code_size() < MAX_CODESIZE
                {
                    self.bump_code_size();
                }

                self.next_code += 1;
                new_link = link;
            } else {
                // It's actually quite likely that the next code will be a reset but just in case.
                // FIXME: this path hasn't been tested very well.
                new_link = link.clone();
            }

            // store the information on the decoded word.
            code_link = Some((new_code, new_link));

            // Can't make any more progress with decoding.
            if is_in_buffer {
                break;
            }
        }

        // We need to store the last word into the buffer in case the first code in the next
        // iteration is the next_code.
        if let Some(tail) = last_decoded {
            self.buffer.bytes[..tail.len()].copy_from_slice(tail);
            self.buffer.write_mark = tail.len();
            self.buffer.read_mark = tail.len();
        }

        // Ensure we don't indicate that no progress was made if we read some bytes from the input
        // (which is progress).
        if o_in > inp.len() {
            if let Ok(LzwStatus::NoProgress) = status {
                status = Ok(LzwStatus::Ok);
            }
        }

        // Store the code/link state.
        self.last = code_link;

        BufferResult {
            consumed_in: o_in.wrapping_sub(inp.len()),
            consumed_out: o_out.wrapping_sub(out.len()),
            status,
        }
    }
}

impl<C: CodeBuffer> DecodeState<C> {
    fn next_symbol(&mut self, inp: &mut &[u8]) -> Option<Code> {
        self.code_buffer.next_symbol(inp)
    }

    fn bump_code_size(&mut self) {
        self.code_buffer.bump_code_size()
    }

    fn refill_bits(&mut self, inp: &mut &[u8]) {
        self.code_buffer.refill_bits(inp)
    }

    fn get_bits(&mut self) -> Option<Code> {
        self.code_buffer.get_bits()
    }
}

impl CodeBuffer for MsbBuffer {
    fn new(min_size: u8) -> Self {
        MsbBuffer {
            code_size: min_size + 1,
            code_mask: (1u16 << (min_size + 1)) - 1,
            bit_buffer: 0,
            bits: 0,
        }
    }

    fn reset(&mut self, min_size: u8) {
        self.code_size = min_size + 1;
        self.code_mask = (1 << self.code_size) - 1;
    }

    fn next_symbol(&mut self, inp: &mut &[u8]) -> Option<Code> {
        if self.bits < self.code_size {
            self.refill_bits(inp);
        }

        self.get_bits()
    }

    fn bump_code_size(&mut self) {
        self.code_size += 1;
        self.code_mask = (self.code_mask << 1) | 1;
    }

    fn refill_bits(&mut self, inp: &mut &[u8]) {
        let wish_count = (64 - self.bits) / 8;
        let mut buffer = [0u8; 8];
        let new_bits = match inp.get(..usize::from(wish_count)) {
            Some(bytes) => {
                buffer[..usize::from(wish_count)].copy_from_slice(bytes);
                *inp = &inp[usize::from(wish_count)..];
                wish_count * 8
            }
            None => {
                let new_bits = inp.len() * 8;
                buffer[..inp.len()].copy_from_slice(inp);
                *inp = &[];
                new_bits as u8
            }
        };
        self.bit_buffer |= u64::from_be_bytes(buffer) >> self.bits;
        self.bits += new_bits;
    }

    fn get_bits(&mut self) -> Option<Code> {
        if self.bits < self.code_size {
            return None;
        }

        let mask = u64::from(self.code_mask);
        let rotbuf = self.bit_buffer.rotate_left(self.code_size.into());
        self.bit_buffer = rotbuf & !mask;
        self.bits -= self.code_size;
        Some((rotbuf & mask) as u16)
    }

    fn max_code(&self) -> Code {
        self.code_mask
    }

    fn code_size(&self) -> u8 {
        self.code_size
    }
}

impl CodeBuffer for LsbBuffer {
    fn new(min_size: u8) -> Self {
        LsbBuffer {
            code_size: min_size + 1,
            code_mask: (1u16 << (min_size + 1)) - 1,
            bit_buffer: 0,
            bits: 0,
        }
    }

    fn reset(&mut self, min_size: u8) {
        self.code_size = min_size + 1;
        self.code_mask = (1 << self.code_size) - 1;
    }

    fn next_symbol(&mut self, inp: &mut &[u8]) -> Option<Code> {
        if self.bits < self.code_size {
            self.refill_bits(inp);
        }

        self.get_bits()
    }

    fn bump_code_size(&mut self) {
        self.code_size += 1;
        self.code_mask = (self.code_mask << 1) | 1;
    }

    fn refill_bits(&mut self, inp: &mut &[u8]) {
        let wish_count = (64 - self.bits) / 8;
        let mut buffer = [0u8; 8];
        let new_bits = match inp.get(..usize::from(wish_count)) {
            Some(bytes) => {
                buffer[..usize::from(wish_count)].copy_from_slice(bytes);
                *inp = &inp[usize::from(wish_count)..];
                wish_count * 8
            }
            None => {
                let new_bits = inp.len() * 8;
                buffer[..inp.len()].copy_from_slice(inp);
                *inp = &[];
                new_bits as u8
            }
        };
        self.bit_buffer |= u64::from_be_bytes(buffer).swap_bytes() << self.bits;
        self.bits += new_bits;
    }

    fn get_bits(&mut self) -> Option<Code> {
        if self.bits < self.code_size {
            return None;
        }

        let mask = u64::from(self.code_mask);
        let code = self.bit_buffer & mask;
        self.bit_buffer >>= self.code_size;
        self.bits -= self.code_size;
        Some(code as u16)
    }

    fn max_code(&self) -> Code {
        self.code_mask
    }

    fn code_size(&self) -> u8 {
        self.code_size
    }
}

impl Buffer {
    fn new() -> Self {
        Buffer {
            bytes: vec![0; MAX_ENTRIES].into_boxed_slice(),
            read_mark: 0,
            write_mark: 0,
        }
    }

    /// When encoding a sequence `cScSc` where `c` is any character and `S` is any string
    /// this results in two codes `AB`, `A` encoding `cS` and `B` encoding `cSc`. Supposing
    /// the buffer is already filled with the reconstruction of `A`, we can easily fill it
    /// with the reconstruction of `B`.
    fn fill_cscsc(&mut self) -> u8 {
        self.bytes[self.write_mark] = self.bytes[0];
        self.write_mark += 1;
        self.read_mark = 0;
        self.bytes[0]
    }

    // Fill the buffer by decoding from the table
    fn fill_reconstruct(&mut self, table: &Table, code: Code) -> u8 {
        self.write_mark = 0;
        self.read_mark = 0;
        let depth = table.depths[usize::from(code)];
        let mut memory = core::mem::replace(&mut self.bytes, Box::default());

        let out = &mut memory[..usize::from(depth)];
        let last = table.reconstruct(code, out);

        self.bytes = memory;
        self.write_mark = usize::from(depth);
        last
    }

    fn buffer(&self) -> &[u8] {
        &self.bytes[self.read_mark..self.write_mark]
    }

    fn consume(&mut self, amt: usize) {
        self.read_mark += amt;
    }
}

impl Table {
    fn new() -> Self {
        Table {
            inner: Vec::with_capacity(MAX_ENTRIES),
            depths: Vec::with_capacity(MAX_ENTRIES),
        }
    }

    fn clear(&mut self, min_size: u8) {
        let static_count = usize::from(1u16 << u16::from(min_size)) + 2;
        self.inner.truncate(static_count);
        self.depths.truncate(static_count);
    }

    fn init(&mut self, min_size: u8) {
        self.inner.clear();
        self.depths.clear();
        for i in 0..(1u16 << u16::from(min_size)) {
            self.inner.push(Link::base(i as u8));
            self.depths.push(1);
        }
        // Clear code.
        self.inner.push(Link::base(0));
        self.depths.push(0);
        // End code.
        self.inner.push(Link::base(0));
        self.depths.push(0);
    }

    fn at(&self, code: Code) -> &Link {
        &self.inner[usize::from(code)]
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn is_full(&self) -> bool {
        self.inner.len() >= MAX_ENTRIES
    }

    fn derive(&mut self, from: &Link, byte: u8, prev: Code) -> Link {
        let link = from.derive(byte, prev);
        let depth = self.depths[usize::from(prev)] + 1;
        self.inner.push(link.clone());
        self.depths.push(depth);
        link
    }

    fn reconstruct(&self, code: Code, out: &mut [u8]) -> u8 {
        let mut code_iter = code;
        let table = &self.inner[..=usize::from(code)];
        let len = code_iter;
        for ch in out.iter_mut().rev() {
            //(code, cha) = self.table[k as usize];
            // Note: This could possibly be replaced with an unchecked array access if
            //  - value is asserted to be < self.next_code() in push
            //  - min_size is asserted to be < MAX_CODESIZE
            let entry = &table[usize::from(code_iter)];
            code_iter = core::cmp::min(len, entry.prev);
            *ch = entry.byte;
        }
        out[0]
    }
}

impl Link {
    fn base(byte: u8) -> Self {
        Link { prev: 0, byte }
    }

    // TODO: this has self type to make it clear we might depend on the old in a future
    // optimization. However, that has no practical purpose right now.
    fn derive(&self, byte: u8, prev: Code) -> Self {
        Link { prev, byte }
    }
}

#[cfg(test)]
mod tests {
    use crate::alloc::vec::Vec;
    #[cfg(feature = "std")]
    use crate::StreamBuf;
    use crate::{decode::Decoder, BitOrder};

    #[test]
    #[should_panic]
    fn invalid_code_size_low() {
        let _ = Decoder::new(BitOrder::Msb, 1);
    }

    #[test]
    #[should_panic]
    fn invalid_code_size_high() {
        let _ = Decoder::new(BitOrder::Msb, 14);
    }

    fn make_encoded() -> Vec<u8> {
        const FILE: &'static [u8] = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/benches/binary-8-msb.lzw"
        ));
        return Vec::from(FILE);
    }

    #[test]
    #[cfg(feature = "std")]
    fn into_stream_buffer_no_alloc() {
        let encoded = make_encoded();
        let mut decoder = Decoder::new(BitOrder::Msb, 8);

        let mut output = vec![];
        let mut buffer = [0; 512];
        let mut istream = decoder.into_stream(&mut output);
        istream.set_buffer(&mut buffer[..]);
        istream.decode(&encoded[..]).status.unwrap();

        match istream.buffer {
            Some(StreamBuf::Borrowed(_)) => {}
            None => panic!("Decoded without buffer??"),
            Some(StreamBuf::Owned(_)) => panic!("Unexpected buffer allocation"),
        }
    }

    #[test]
    #[cfg(feature = "std")]
    fn into_stream_buffer_small_alloc() {
        struct WriteTap<W: std::io::Write>(W);
        const BUF_SIZE: usize = 512;

        impl<W: std::io::Write> std::io::Write for WriteTap<W> {
            fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                assert!(buf.len() <= BUF_SIZE);
                self.0.write(buf)
            }
            fn flush(&mut self) -> std::io::Result<()> {
                self.0.flush()
            }
        }

        let encoded = make_encoded();
        let mut decoder = Decoder::new(BitOrder::Msb, 8);

        let mut output = vec![];
        let mut istream = decoder.into_stream(WriteTap(&mut output));
        istream.set_buffer_size(512);
        istream.decode(&encoded[..]).status.unwrap();

        match istream.buffer {
            Some(StreamBuf::Owned(vec)) => assert!(vec.len() <= BUF_SIZE),
            Some(StreamBuf::Borrowed(_)) => panic!("Unexpected borrowed buffer, where from?"),
            None => panic!("Decoded without buffer??"),
        }
    }

    #[test]
    #[cfg(feature = "std")]
    fn reset() {
        let encoded = make_encoded();
        let mut decoder = Decoder::new(BitOrder::Msb, 8);
        let mut reference = None;

        for _ in 0..2 {
            let mut output = vec![];
            let mut buffer = [0; 512];
            let mut istream = decoder.into_stream(&mut output);
            istream.set_buffer(&mut buffer[..]);
            istream.decode_all(&encoded[..]).status.unwrap();

            decoder.reset();
            if let Some(reference) = &reference {
                assert_eq!(output, *reference);
            } else {
                reference = Some(output);
            }
        }
    }
}
