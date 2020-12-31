#![allow(bad_style)]
#![doc(html_root_url = "https://docs.rs/brotli-sys/0.2")]

extern crate libc;

use libc::{c_void, size_t, c_int, c_char};

#[cfg(target_env = "msvc")]
#[doc(hidden)]
pub type __enum_ty = libc::c_int;
#[cfg(not(target_env = "msvc"))]
#[doc(hidden)]
pub type __enum_ty = libc::c_uint;

pub type __enum_ty_s = libc::c_int;

pub type brotli_alloc_func = Option<extern "C" fn(*mut c_void, size_t) -> *mut c_void>;
pub type brotli_free_func = Option<extern "C" fn(*mut c_void, *mut c_void)>;

// ========== Decoder functionality ==========

pub type BrotliDecoderResult = __enum_ty;
pub type BrotliDecoderErrorCode = __enum_ty_s;

pub enum BrotliDecoderState {}

pub const BROTLI_DECODER_RESULT_ERROR: BrotliDecoderResult = 0;
pub const BROTLI_DECODER_RESULT_SUCCESS: BrotliDecoderResult = 1;
pub const BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT: BrotliDecoderResult = 2;
pub const BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT: BrotliDecoderResult = 3;

pub const BROTLI_DECODER_NO_ERROR: BrotliDecoderErrorCode = 0;
pub const BROTLI_DECODER_SUCCESS: BrotliDecoderErrorCode = 1;
pub const BROTLI_DECODER_NEEDS_MORE_INPUT: BrotliDecoderErrorCode = 2;
pub const BROTLI_DECODER_NEEDS_MORE_OUTPUT: BrotliDecoderErrorCode = 3;
pub const BROTLI_DECODER_ERROR_FORMAT_EXUBERANT_NIBBLE: BrotliDecoderErrorCode = -1;
pub const BROTLI_DECODER_ERROR_FORMAT_RESERVED: BrotliDecoderErrorCode = -2;
pub const BROTLI_DECODER_ERROR_FORMAT_EXUBERANT_META_NIBBLE: BrotliDecoderErrorCode = -3;
pub const BROTLI_DECODER_ERROR_FORMAT_SIMPLE_HUFFMAN_ALPHABET: BrotliDecoderErrorCode = -4;
pub const BROTLI_DECODER_ERROR_FORMAT_SIMPLE_HUFFMAN_SAME: BrotliDecoderErrorCode = -5;
pub const BROTLI_DECODER_ERROR_FORMAT_CL_SPACE: BrotliDecoderErrorCode = -6;
pub const BROTLI_DECODER_ERROR_FORMAT_HUFFMAN_SPACE: BrotliDecoderErrorCode = -7;
pub const BROTLI_DECODER_ERROR_FORMAT_CONTEXT_MAP_REPEAT: BrotliDecoderErrorCode = -8;
pub const BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_1: BrotliDecoderErrorCode = -9;
pub const BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_2: BrotliDecoderErrorCode = -10;
pub const BROTLI_DECODER_ERROR_FORMAT_TRANSFORM: BrotliDecoderErrorCode = -11;
pub const BROTLI_DECODER_ERROR_FORMAT_DICTIONARY: BrotliDecoderErrorCode = -12;
pub const BROTLI_DECODER_ERROR_FORMAT_WINDOW_BITS: BrotliDecoderErrorCode = -13;
pub const BROTLI_DECODER_ERROR_FORMAT_PADDING_1: BrotliDecoderErrorCode = -14;
pub const BROTLI_DECODER_ERROR_FORMAT_PADDING_2: BrotliDecoderErrorCode = -15;
pub const BROTLI_DECODER_ERROR_INVALID_ARGUMENTS: BrotliDecoderErrorCode = -20;
pub const BROTLI_DECODER_ERROR_ALLOC_CONTEXT_MODES: BrotliDecoderErrorCode = -21;
pub const BROTLI_DECODER_ERROR_ALLOC_TREE_GROUPS: BrotliDecoderErrorCode = -22;
pub const BROTLI_DECODER_ERROR_ALLOC_CONTEXT_MAP: BrotliDecoderErrorCode = -25;
pub const BROTLI_DECODER_ERROR_ALLOC_RING_BUFFER_1: BrotliDecoderErrorCode = -26;
pub const BROTLI_DECODER_ERROR_ALLOC_RING_BUFFER_2: BrotliDecoderErrorCode = -27;
pub const BROTLI_DECODER_ERROR_ALLOC_BLOCK_TYPE_TREES: BrotliDecoderErrorCode = -30;
pub const BROTLI_DECODER_ERROR_UNREACHABLE: BrotliDecoderErrorCode = -31;

extern "C" {
    pub fn BrotliDecoderCreateInstance(alloc_func: brotli_alloc_func,
                                       free_func: brotli_free_func,
                                       opaque: *mut c_void)
                                       -> *mut BrotliDecoderState;
    pub fn BrotliDecoderDestroyInstance(state: *mut BrotliDecoderState);
    pub fn BrotliDecoderDecompress(encoded_size: size_t,
                                   encoded_buffer: *const u8,
                                   decoded_size: *mut size_t,
                                   decoded_buffer: *mut u8) ->
                                   BrotliDecoderResult;
    pub fn BrotliDecoderDecompressStream(state: *mut BrotliDecoderState,
                                         available_in: *mut size_t,
                                         next_in: *mut *const u8,
                                         available_out: *mut size_t,
                                         next_out: *mut *mut u8,
                                         total_out: *mut size_t)
                                         -> BrotliDecoderResult;
    pub fn BrotliDecoderSetCustomDictionary(state: *mut BrotliDecoderState,
                                            size: size_t,
                                            dict: *const u8);
    pub fn BrotliDecoderHasMoreOutput(state: *const BrotliDecoderState) -> c_int;
    pub fn BrotliDecoderTakeOutput(state: *mut BrotliDecoderState,
                                   size: *mut size_t)
                                   -> *const u8;
    pub fn BrotliDecoderIsUsed(state: *const BrotliDecoderState) -> c_int;
    pub fn BrotliDecoderIsFinished(state: *const BrotliDecoderState) -> c_int;
    pub fn BrotliDecoderGetErrorCode(state: *const BrotliDecoderState) -> BrotliDecoderErrorCode;
    pub fn BrotliDecoderErrorString(c: BrotliDecoderErrorCode) -> *const c_char;
    pub fn BrotliDecoderVersion() -> u32;
}



// ========== Encoder functionality ==========

pub type BrotliEncoderMode = __enum_ty;
pub type BrotliEncoderParameter = __enum_ty;
pub type BrotliEncoderOperation = __enum_ty;

pub const BROTLI_MODE_GENERIC: BrotliEncoderMode = 0;
pub const BROTLI_MODE_TEXT: BrotliEncoderMode = 1;
pub const BROTLI_MODE_FONT: BrotliEncoderMode = 2;

pub const BROTLI_PARAM_MODE: BrotliEncoderParameter = 0;
pub const BROTLI_PARAM_QUALITY: BrotliEncoderParameter = 1;
pub const BROTLI_PARAM_LGWIN: BrotliEncoderParameter = 2;
pub const BROTLI_PARAM_LGBLOCK: BrotliEncoderParameter = 3;
pub const BROTLI_PARAM_DISABLE_LITERAL_CONTEXT_MODELING: BrotliEncoderParameter = 4;
pub const BROTLI_PARAM_SIZE_HINT: BrotliEncoderParameter = 5;

pub const BROTLI_OPERATION_PROCESS: BrotliEncoderOperation = 0;
pub const BROTLI_OPERATION_FLUSH: BrotliEncoderOperation = 1;
pub const BROTLI_OPERATION_FINISH: BrotliEncoderOperation = 2;
pub const BROTLI_OPERATION_EMIT_METADATA: BrotliEncoderOperation = 3;

pub const BROTLI_DEFAULT_QUALITY: u32 = 11;
pub const BROTLI_DEFAULT_WINDOW: u32 = 22;
pub const BROTLI_DEFAULT_MODE: u32 = 0;

pub enum BrotliEncoderState {}

extern "C" {
    pub fn BrotliEncoderSetParameter(state: *mut BrotliEncoderState,
                                     param: BrotliEncoderParameter,
                                     value: u32)
                                     -> c_int;
    pub fn BrotliEncoderCreateInstance(alloc_func: brotli_alloc_func,
                                       free_func: brotli_free_func,
                                       opaque: *mut c_void)
                                       -> *mut BrotliEncoderState;
    pub fn BrotliEncoderDestroyInstance(state: *mut BrotliEncoderState);
    // These three are deprecated
    //pub fn BrotliEncoderInputBlockSize(state: *mut BrotliEncoderState) -> size_t;
    //pub fn BrotliEncoderCopyInputToRingBuffer(state: *mut BrotliEncoderState,
    //                                          input_size: size_t,
    //                                          input_buffer: *const u8);
    //pub fn BrotliEncoderWriteData(state: *mut BrotliEncoderState,
    //                              is_last: c_int,
    //                              force_flush: c_int,
    //                              out_size: *mut size_t,
    //                              output: *mut *mut u8)
    //                              -> c_int;
    pub fn BrotliEncoderSetCustomDictionary(state: *mut BrotliEncoderState,
                                            size: size_t,
                                            dict: *const u8);
    pub fn BrotliEncoderMaxCompressedSize(input_size: size_t) -> size_t;
    pub fn BrotliEncoderCompress(quality: c_int,
                                 lgwin: c_int,
                                 mode: BrotliEncoderMode,
                                 input_size: size_t,
                                 input_buffer: *const u8,
                                 encoded_size: *mut size_t,
                                 encoded_buffer: *mut u8)
                                 -> c_int;
    pub fn BrotliEncoderCompressStream(state: *mut BrotliEncoderState,
                                       op: BrotliEncoderOperation,
                                       available_in: *mut size_t,
                                       next_in: *mut *const u8,
                                       available_out: *mut size_t,
                                       next_out: *mut *mut u8,
                                       total_out: *mut size_t)
                                       -> c_int;
    pub fn BrotliEncoderIsFinished(state: *mut BrotliEncoderState) -> c_int;
    pub fn BrotliEncoderHasMoreOutput(state: *mut BrotliEncoderState) -> c_int;
    pub fn BrotliEncoderTakeOutput(state: *mut BrotliEncoderState,
                                   size: *mut usize)
                                   -> *const u8;
    pub fn BrotliEncoderVersion() -> u32;
}
