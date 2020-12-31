extern crate criterion;
extern crate jpeg_decoder;

use criterion::{black_box, Criterion};

use jpeg_decoder as jpeg;
use jpeg_decoder::ImageInfo;

fn read_image(image: &[u8]) -> Vec<u8> {
    jpeg::Decoder::new(black_box(image)).decode().unwrap()
}

fn read_metadata(image: &[u8]) -> ImageInfo {
    let mut decoder = jpeg::Decoder::new(black_box(image));
    decoder.read_info().unwrap();
    decoder.info().unwrap()
}

fn main() {
    let mut c = Criterion::default().configure_from_args();
    c.bench_function("decode a 512x512 JPEG", |b| b.iter(|| {
        read_image(include_bytes!("tower.jpg"))
    }));

    c.bench_function("decode a 512x512 progressive JPEG", |b| b.iter(|| {
        read_image(include_bytes!("tower_progressive.jpg"))
    }));

    c.bench_function("decode a 512x512 grayscale JPEG", |b| b.iter(|| {
        read_image(include_bytes!("tower_grayscale.jpg"))
    }));

    c.bench_function("extract metadata from an image", |b| b.iter(|| {
        read_metadata(include_bytes!("tower.jpg"))
    }));
    c.final_summary();
}