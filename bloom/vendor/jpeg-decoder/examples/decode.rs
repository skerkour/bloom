extern crate jpeg_decoder as jpeg;
extern crate png;

use std::env;
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::process;

fn usage() -> ! {
    write!(io::stderr(), "usage: decode image.jpg image.png").unwrap();
    process::exit(1)
}

fn main() {
    let mut args = env::args().skip(1);
    let input_path = args.next().unwrap_or_else(|| usage());
    let output_path = args.next().unwrap_or_else(|| usage());

    let input_file = File::open(input_path).expect("The specified input file could not be opened");
    let mut decoder = jpeg::Decoder::new(BufReader::new(input_file));
    let mut data = decoder.decode().expect("Decoding failed. If other software can successfully decode the specified JPEG image, then it's likely that there is a bug in jpeg-decoder");
    let info = decoder.info().unwrap();

    let output_file = File::create(output_path).unwrap();
    let mut encoder = png::Encoder::new(output_file, info.width as u32, info.height as u32);
    encoder.set_depth(png::BitDepth::Eight);

    match info.pixel_format {
        jpeg::PixelFormat::L8     => encoder.set_color(png::ColorType::Grayscale),
        jpeg::PixelFormat::RGB24  => encoder.set_color(png::ColorType::RGB),
        jpeg::PixelFormat::CMYK32 => {
            data = cmyk_to_rgb(&mut data);
            encoder.set_color(png::ColorType::RGB)
        },
    };

    encoder.write_header()
           .expect("writing png header failed")
           .write_image_data(&data)
           .expect("png encoding failed");
}

fn cmyk_to_rgb(input: &[u8]) -> Vec<u8> {
    let size = input.len() - input.len() / 4;
    let mut output = Vec::with_capacity(size);

    for pixel in input.chunks(4) {
        let c = pixel[0] as f32 / 255.0;
        let m = pixel[1] as f32 / 255.0;
        let y = pixel[2] as f32 / 255.0;
        let k = pixel[3] as f32 / 255.0;

        // CMYK -> CMY
        let c = c * (1.0 - k) + k;
        let m = m * (1.0 - k) + k;
        let y = y * (1.0 - k) + k;

        // CMY -> RGB
        let r = (1.0 - c) * 255.0;
        let g = (1.0 - m) * 255.0;
        let b = (1.0 - y) * 255.0;

        output.push(r as u8);
        output.push(g as u8);
        output.push(b as u8);
    }

    output
}
