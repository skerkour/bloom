quoted-printable
===
[![Build Status](https://travis-ci.org/staktrace/quoted-printable.svg?branch=master)](https://travis-ci.org/staktrace/quoted-printable)
[![Crate](https://img.shields.io/crates/v/quoted_printable.svg)](https://crates.io/crates/quoted_printable)

A quoted-printable decoder and encoder.

API
---
quoted-printable exposes two functions at the moment:

```rust
    decode<R: AsRef<[u8]>>(input: R, mode: ParseMode) -> Result<Vec<u8>, QuotedPrintableError>
    encode<R: AsRef<[u8]>>(input: R) -> Vec<u8>
```

using `R: AsRef<[u8]>` means that you can pass in a variety of types, including:
`String`, `&String`, `&str`, `Vec<u8>`, `&Vec<u8>`, `&[u8]`, `Box<[u8]>`, `Arc<[u8]>`


The decode function can be used to convert a quoted-printable string into the decoded bytes, as per the description in [IETF RFC 2045, section 6.7](https://tools.ietf.org/html/rfc2045#section-6.7).
The ParseMode option can be used to control whether the decoding is "strict" or "robust", as per the comments in that RFC.
In general you should probably use "robust" decoding, as it will gracefully handle more malformed input.

The encode function obviously does the reverse, and converts a set of raw bytes into quoted-printable.


Additionally there is following helper function:

```rust
    encode_to_str<R: AsRef<[u8]>>(input: R) -> String
```

which takes advantage of the fact that the `Vec<u8>` returned by `encode` can only
contain valid us-ascii and converts it to a `String` (using the unsafe
`String::from_utf8_unchecked`)

Documentation
---
See document on [https://docs.rs](https://docs.rs/quoted_printable/).
