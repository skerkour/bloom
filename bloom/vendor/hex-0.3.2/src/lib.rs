#![cfg_attr(feature = "benchmarks", feature(test))]

// Copyright (c) 2013-2014 The Rust Project Developers.
// Copyright (c) 2015-2018 The rust-hex Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//! Encoding and decoding hex strings.
//!
//! For most cases, you can simply use the `decode()`, `encode()` and
//! `encode_upper()` functions. If you need a bit more control, use the traits
//! `ToHex` and `FromHex` instead.
//!
//! # Example
//!
//! ```
//! extern crate hex;
//!
//! fn main() {
//!     let hex_string = hex::encode("Hello world!");
//!     println!("{}", hex_string); // Prints '48656c6c6f20776f726c6421'
//! }
//! ```

use std::error;
use std::fmt;

/// Encoding values as hex string.
///
/// This trait is implemented for all `T` which implement `AsRef<[u8]>`. This
/// includes `String`, `str`, `Vec<u8>` and `[u8]`.
///
/// # Example
///
/// ```
/// use hex::ToHex;
///
/// let mut s = String::new();
/// "Hello world!".write_hex(&mut s).unwrap();
/// println!("{}", s);
/// ```
///
/// *Note*: instead of using this trait, you might want to use `encode()`.
pub trait ToHex {
    /// Writes the hex string representing `self` into `w`. Lower case letters
    /// are used (e.g. `f9b4ca`).
    fn write_hex<W: fmt::Write>(&self, w: &mut W) -> fmt::Result;

    /// Writes the hex string representing `self` into `w`. Upper case letters
    /// are used (e.g. `F9B4CA`).
    fn write_hex_upper<W: fmt::Write>(&self, w: &mut W) -> fmt::Result;
}

fn hex_write<W: fmt::Write>(table: &[u8; 16], src: &[u8], w: &mut W) -> fmt::Result {
    let hex = |byte: u8| table[byte as usize];

    for &b in src.iter() {
        w.write_char(hex((b >> 4) & 0xf) as char)?;
        w.write_char(hex(b & 0xf) as char)?;
    }

    Ok(())
}

impl<T: AsRef<[u8]>> ToHex for T {
    fn write_hex<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        static CHARS: &'static [u8; 16] = b"0123456789abcdef";

        hex_write(&CHARS, self.as_ref(), w)
    }

    fn write_hex_upper<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        static CHARS: &'static [u8; 16] = b"0123456789ABCDEF";

        hex_write(&CHARS, self.as_ref(), w)
    }
}

/// The error type for decoding a hex string into `Vec<u8>` or `[u8; N]`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FromHexError {
    /// An invalid character was found. Valid ones are: `0...9`, `a...f`
    /// or `A...F`.
    InvalidHexCharacter {
        c: char,
        index: usize,
    },

    /// A hex string's length needs to be even, as two digits correspond to
    /// one byte.
    OddLength,

    /// If the hex string is decoded into a fixed sized container, such as an
    /// array, the hex string's length * 2 has to match the container's
    /// length.
    InvalidStringLength,
}

impl error::Error for FromHexError {
    fn description(&self) -> &str {
        match *self {
            FromHexError::InvalidHexCharacter { .. } => "invalid character",
            FromHexError::OddLength => "odd number of digits",
            FromHexError::InvalidStringLength => "invalid string length",

        }
    }
}

impl fmt::Display for FromHexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FromHexError::InvalidHexCharacter { c, index } =>
                write!(f, "Invalid character '{}' at position {}", c, index),
            FromHexError::OddLength =>
                write!(f, "Odd number of digits"),
            FromHexError::InvalidStringLength =>
                write!(f, "Invalid string length"),
        }
    }
}

/// Types that can be decoded from a hex string.
///
/// This trait is implemented for `Vec<u8>` and small `u8`-arrays.
///
/// # Example
///
/// ```
/// use hex::FromHex;
///
/// match Vec::from_hex("48656c6c6f20776f726c6421") {
///     Ok(vec) => {
///         for b in vec {
///             println!("{}", b as char);
///         }
///     }
///     Err(e) => {
///         // Deal with the error ...
///     }
/// }
/// ```
pub trait FromHex: Sized {
    type Error;

    /// Creates an instance of type `Self` from the given hex string, or fails
    /// with a custom error type.
    ///
    /// Both, upper and lower case characters are valid and can even be
    /// mixed (e.g. `f9b4ca`, `F9B4CA` and `f9B4Ca` are all valid strings).
    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error>;
}

fn val(c: u8, idx: usize) -> Result<u8, FromHexError> {
    match c {
        b'A'...b'F' => Ok(c - b'A' + 10),
        b'a'...b'f' => Ok(c - b'a' + 10),
        b'0'...b'9' => Ok(c - b'0'),
        _ => {
            Err(FromHexError::InvalidHexCharacter {
                c: c as char,
                index: idx,
            })
        }
    }
}

impl FromHex for Vec<u8> {
    type Error = FromHexError;

    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
        let hex = hex.as_ref();
        if hex.len() % 2 != 0 {
            return Err(FromHexError::OddLength);
        }

        hex.chunks(2).enumerate().map(|(i, pair)| {
            Ok(val(pair[0], 2 * i)? << 4 | val(pair[1], 2 * i + 1)?)
        }).collect()
    }
}

// Helper macro to implement the trait for a few fixed sized arrays. Once Rust
// has type level integers, this should be removed.
macro_rules! from_hex_array_impl {
    ($($len:expr)+) => {$(
        impl FromHex for [u8; $len] {
            type Error = FromHexError;

            fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
                let hex = hex.as_ref();
                if hex.len() % 2 != 0 {
                    return Err(FromHexError::OddLength);
                }
                if hex.len() / 2 != $len {
                    return Err(FromHexError::InvalidStringLength);
                }

                let mut out = [0; $len];
                for (i, byte) in out.iter_mut().enumerate() {
                    *byte = val(hex[2 * i], 2 * i)? << 4
                        | val(hex[2 * i + 1], 2 * i + 1)?;
                }

                Ok(out)
            }
        }
    )+}
}

from_hex_array_impl! {
    1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16
    17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32
    33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48
    49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64
    65 66 67 68 69 70 71 72 73 74 75 76 77 78 79 80
    81 82 83 84 85 86 87 88 89 90 91 92 93 94 95 96
    97 98 99 100 101 102 103 104 105 106 107 108 109 110 111 112
    113 114 115 116 117 118 119 120 121 122 123 124 125 126 127 128
    160 192 200 224 256 384 512 768 1024 2048 4096 8192 16384 32768
}

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
from_hex_array_impl! {
    65536 131072 262144 524288 1048576 2097152 4194304 8388608
    16777216 33554432 67108864 134217728 268435456 536870912
    1073741824 2147483648
}

#[cfg(target_pointer_width = "64")]
from_hex_array_impl! {
    4294967296
}

/// Encodes `data` as hex string using lowercase characters.
///
/// Lowercase characters are used (e.g. `f9b4ca`). The resulting string's
/// length is always even, each byte in `data` is always encoded using two hex
/// digits. Thus, the resulting string contains exactly twice as many bytes as
/// the input data.
///
/// # Example
///
/// ```
/// assert_eq!(hex::encode("Hello world!"), "48656c6c6f20776f726c6421");
/// assert_eq!(hex::encode(vec![1, 2, 3, 15, 16]), "0102030f10");
/// ```
pub fn encode<T: AsRef<[u8]>>(data: T) -> String {
    let mut s = String::with_capacity(data.as_ref().len() * 2);

    // Writing to a string never errors, so we can unwrap here.
    data.write_hex(&mut s).unwrap();
    s
}

/// Encodes `data` as hex string using uppercase characters.
///
/// Apart from the characters' casing, this works exactly like `encode()`.
///
/// # Example
///
/// ```
/// assert_eq!(hex::encode_upper("Hello world!"), "48656C6C6F20776F726C6421");
/// assert_eq!(hex::encode_upper(vec![1, 2, 3, 15, 16]), "0102030F10");
/// ```
pub fn encode_upper<T: AsRef<[u8]>>(data: T) -> String {
    let mut s = String::with_capacity(data.as_ref().len() * 2);

    // Writing to a string never errors, so we can unwrap here.
    data.write_hex_upper(&mut s).unwrap();
    s
}

/// Decodes a hex string into raw bytes.
///
/// Both, upper and lower case characters are valid in the input string and can
/// even be mixed (e.g. `f9b4ca`, `F9B4CA` and `f9B4Ca` are all valid strings).
///
/// # Example
/// ```
/// assert_eq!(
///     hex::decode("48656c6c6f20776f726c6421"),
///     Ok("Hello world!".to_owned().into_bytes())
/// );
///
/// assert_eq!(hex::decode("123"), Err(hex::FromHexError::OddLength));
/// assert!(hex::decode("foo").is_err());
/// ```
pub fn decode<T: AsRef<[u8]>>(data: T) -> Result<Vec<u8>, FromHexError> {
    FromHex::from_hex(data)
}


#[cfg(test)]
mod test {
    use super::{encode, decode, FromHex, FromHexError};

    #[test]
    fn test_encode() {
        assert_eq!(encode("foobar"), "666f6f626172");
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode("666f6f626172"), Ok("foobar".to_owned().into_bytes()));
    }

    #[test]
    pub fn test_from_hex_okay_str() {
        assert_eq!(
            Vec::from_hex("666f6f626172").unwrap(),
            b"foobar"
        );
        assert_eq!(
            Vec::from_hex("666F6F626172").unwrap(),
            b"foobar"
        );
    }

    #[test]
    pub fn test_from_hex_okay_bytes() {
        assert_eq!(
            Vec::from_hex(b"666f6f626172").unwrap(),
            b"foobar"
        );
        assert_eq!(
            Vec::from_hex(b"666F6F626172").unwrap(),
            b"foobar"
        );
    }

    #[test]
    pub fn test_invalid_length() {
        assert_eq!(
            Vec::from_hex("1").unwrap_err(),
            FromHexError::OddLength
        );
        assert_eq!(
            Vec::from_hex("666f6f6261721").unwrap_err(),
            FromHexError::OddLength
        );
    }

    #[test]
    pub fn test_invalid_char() {
        assert_eq!(
            Vec::from_hex("66ag").unwrap_err(),
            FromHexError::InvalidHexCharacter {
                c: 'g',
                index: 3
            }
        );
    }

    #[test]
    pub fn test_empty() {
        assert_eq!(Vec::from_hex("").unwrap(), b"");
    }

    #[test]
    pub fn test_from_hex_whitespace() {
        assert_eq!(
            Vec::from_hex("666f 6f62617").unwrap_err(),
            FromHexError::InvalidHexCharacter {
                c: ' ',
                index: 4
            }
        );
    }

    #[test]
    pub fn test_from_hex_array() {
        assert_eq!(
            <[u8; 6] as FromHex>::from_hex("666f6f626172"),
            Ok([0x66, 0x6f, 0x6f, 0x62, 0x61, 0x72])
        );

        assert_eq!(
            <[u8; 5] as FromHex>::from_hex("666f6f626172"),
            Err(FromHexError::InvalidStringLength)
        );
    }
}


#[cfg(all(feature = "benchmarks", test))]
mod bench {
    extern crate test;
    use self::test::Bencher;

    use super::*;

    const MY_OWN_SOURCE: &[u8] = include_bytes!("lib.rs");

    #[bench]
    fn a_bench(b: &mut Bencher) {
        b.bytes = MY_OWN_SOURCE.len() as u64;

        b.iter(|| {
            encode(MY_OWN_SOURCE)
        });
    }
}
