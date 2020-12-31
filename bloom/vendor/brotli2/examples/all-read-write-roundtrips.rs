extern crate brotli2;
extern crate rand;

use std::io::{Read, Write};
use brotli2::read;
use brotli2::write;
use brotli2::CompressParams;
use brotli2::raw::{compress_buf, decompress_buf};
use rand::Rng;
use rand::SeedableRng;

// Used in functions as temporary storage space before producing a vec
const BIGBUF_SIZE: usize = 120*1024*1024;
static mut BIGBUF: [u8; BIGBUF_SIZE] = [0; BIGBUF_SIZE];

fn main() {
    let v1 = vec![1; 1024];
    let v2 = vec![44; 10*1024*1024];
    let datas: &[&[u8]] = &[
        b"",
        b"a",
        b"aaaaa",
        b";",
        &v1,
        &v2,
    ];
    let mut params = CompressParams::new();
    params.quality(6);
    let params = &params;

    fn bufencode(data: &[u8], params: &CompressParams) -> Vec<u8> {
        let bufref = &mut unsafe { &mut BIGBUF[..] };
        let n = compress_buf(params, data, bufref).unwrap();
        assert!(n > 0 && n <= BIGBUF_SIZE && n == bufref.len());
        bufref.to_owned()
    }
    fn bufdecode(data: &[u8]) -> Vec<u8> {
        let bufref = &mut unsafe { &mut BIGBUF[..] };
        let n = decompress_buf(data, bufref).unwrap();
        assert!(n <= BIGBUF_SIZE && n == bufref.len());
        bufref.to_owned()
    }
    fn ioreadencode(data: &[u8], params: &CompressParams) -> Vec<u8> {
        let mut buf = vec![];
        read::BrotliEncoder::from_params(data, params).read_to_end(&mut buf).unwrap();
        assert!(buf.len() > 0);
        buf
    }
    fn ioreaddecode (data: &[u8]) -> Vec<u8> {
        let mut buf = vec![];
        read::BrotliDecoder::new(data).read_to_end(&mut buf).unwrap();
        buf
    }
    fn iowriteencode(data: &[u8], params: &CompressParams) -> Vec<u8> {
        let mut enc = write::BrotliEncoder::from_params(vec![], params);
        enc.write_all(data).unwrap();
        enc.finish().unwrap()
    }
    fn iowritedecode(data: &[u8]) -> Vec<u8> {
        let mut dec = write::BrotliDecoder::new(vec![]);
        dec.write_all(data).unwrap();
        dec.finish().unwrap()
    }

    let check = |data: &[u8]| {
        let c1 = &bufencode(data, params);
        let c2 = &ioreadencode(data, params);
        let c3 = &iowriteencode(data, params);
        assert!(data == &*bufdecode(c1));
        assert!(data == &*ioreaddecode(c1));
        assert!(data == &*iowritedecode(c1));
        // it's valid for them to be different, but we need to do more work
        if c2 != c1 {
            assert!(data == &*bufdecode(c2));
            assert!(data == &*ioreaddecode(c2));
            assert!(data == &*iowritedecode(c2));
        }
        if c3 != c1 && c3 != c2 {
            assert!(data == &*bufdecode(c2));
            assert!(data == &*ioreaddecode(c2));
            assert!(data == &*iowritedecode(c2));
        }
    };

    for &data in datas.iter() {
        check(data)
    }
    let mut rng = rand::XorShiftRng::from_seed([1, 4, 55, 98]);
    for _ in 0..3 {
        let rnum: usize = rng.gen_range(1, 100*1024*1024);
        let mut buf = vec![0; rnum];
        rng.fill_bytes(&mut buf);
        check(&buf)
    }
}
