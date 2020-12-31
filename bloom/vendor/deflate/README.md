# deflate-rs

[![Build Status](https://travis-ci.org/image-rs/deflate-rs.svg)](https://travis-ci.org/image-rs/deflate-rs)[![Crates.io](https://img.shields.io/crates/v/deflate.svg)](https://crates.io/crates/deflate)[![Docs](https://docs.rs/deflate/badge.svg)](https://docs.rs/deflate)


An implementation of a [DEFLATE](http://www.gzip.org/zlib/rfc-deflate.html) encoder in pure Rust. Not a direct port, but does take some inspiration from [zlib](http://www.zlib.net/), [miniz](https://github.com/richgel999/miniz) and [zopfli](https://github.com/google/zopfli). The API is based on the one in the [flate2](https://crates.io/crates/flate2) crate that contains bindings, zlib miniz_oxide, and miniz.

Deflate encoding with and without zlib and gzip metadata (zlib dictionaries are not supported) is supported. No unsafe code is used.

This library is now mostly in maintenance mode, focus being on the Rust backend of [flate2](https://crates.io/crates/flate2) instead.

# Usage:
## Simple compression function:
``` rust
use deflate::deflate_bytes;

let data = b"Some data";
let compressed = deflate_bytes(&data);
```

## Using a writer:

``` rust
use std::io::Write;

use deflate::Compression;
use deflate::write::ZlibEncoder;

let data = b"This is some test data";
let mut encoder = ZlibEncoder::new(Vec::new(), Compression::Default);
encoder.write_all(data).unwrap();
let compressed_data = encoder.finish().unwrap();
```

# Other deflate/zlib Rust projects from various people
* [flate2](http://alexcrichton.com/flate2-rs/flate2/index.html) FLATE, Gzip, and Zlib bindings for Rust - can use miniz_oxide for a pure Rust implementation.
* [Zopfli in Rust](https://github.com/carols10cents/zopfli) Rust port of zopfli
* [inflate](https://github.com/PistonDevelopers/inflate) DEFLATE decoder implemented in Rust
* [miniz-oxide](https://github.com/Frommi/miniz_oxide) Port of miniz to Rust.
* [libflate](https://github.com/sile/libflate) Another DEFLATE/Zlib/Gzip encoder and decoder written in Rust. (Only does some very light compression).

# License
deflate is distributed under the terms of both the MIT and Apache 2.0 licences.

bitstream.rs is © @nwin and was released under both MIT and Apache 2.0

Some code in length_encode.rs has been ported from the `miniz` library, which is public domain.

The test data (src/pg11.txt) is borrowed from [Project Gutenberg](https://www.gutenberg.org/ebooks/11) and is available under public domain, or the Project Gutenberg Licence
