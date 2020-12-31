#![cfg(test)]

#[cfg(feature = "gzip")]
use gzip_header::GzHeader;

fn get_test_file_data(name: &str) -> Vec<u8> {
    use std::fs::File;
    use std::io::Read;
    let mut input = Vec::new();
    let mut f = File::open(name).unwrap();

    f.read_to_end(&mut input).unwrap();
    input
}

pub fn get_test_data() -> Vec<u8> {
    use std::env;
    let path = env::var("TEST_FILE").unwrap_or("tests/pg11.txt".to_string());
    get_test_file_data(&path)
}

/// Helper function to decompress into a `Vec<u8>`
pub fn decompress_to_end(input: &[u8]) -> Vec<u8> {
    use miniz_oxide::inflate::decompress_to_vec;

    decompress_to_vec(input).expect("Decompression failed!")
}

#[cfg(feature = "gzip")]
pub fn decompress_gzip(compressed: &[u8]) -> (GzHeader, Vec<u8>) {
    use gzip_header::{read_gz_header, Crc};
    use std::io::Cursor;
    let mut c = Cursor::new(compressed);
    let h = read_gz_header(&mut c).expect("Failed to decode gzip header!");
    let pos = c.position();
    let compressed = &c.into_inner()[pos as usize..];

    let result = miniz_oxide::inflate::decompress_to_vec(compressed).expect("Decompression failed");

    let s = compressed.len();

    let crc = u32::from_le_bytes([
        compressed[s - 8],
        compressed[s - 7],
        compressed[s - 6],
        compressed[s - 5],
    ]);
    let len = u32::from_le_bytes([
        compressed[s - 4],
        compressed[s - 3],
        compressed[s - 2],
        compressed[s - 1],
    ]);

    let mut comp_crc = Crc::new();
    comp_crc.update(&result);

    assert_eq!(
        crc,
        comp_crc.sum(),
        "Checksum failed File: {}, computed: {}",
        crc,
        comp_crc.sum()
    );
    assert_eq!(len, result.len() as u32, "Length mismatch");

    (h, result)
}

pub fn decompress_zlib(compressed: &[u8]) -> Vec<u8> {
    miniz_oxide::inflate::decompress_to_vec_zlib(&compressed).expect("Decompression failed!")
}
