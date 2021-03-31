//! Exports each GIF frame as a separate image.

use std::env;
use std::fs::File;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = PathBuf::from(
        env::args_os()
            .nth(1)
            .ok_or("Specify a GIF path as the first argument")?,
    );

    let input = File::open(&input_path)?;
    let mut options = gif::DecodeOptions::new();
    options.set_color_output(gif::ColorOutput::Indexed);
    let mut decoder = options.read_info(input)?;
    let screen_width = decoder.width();
    let screen_height = decoder.height();
    let global_pal = decoder.global_palette().unwrap_or_default().to_vec();

    let output_file_stem = input_path.file_stem().unwrap().to_str().unwrap();
    let mut frame_number = 1;
    while let Some(frame) = decoder.read_next_frame()? {
        let output_path = format!("{}.{:03}.gif", output_file_stem, frame_number);
        let mut output = File::create(&output_path)?;
        let mut encoder = gif::Encoder::new(&mut output, screen_width, screen_height, &global_pal)?;
        encoder.write_frame(&frame)?;
        frame_number += 1;

        use gif::DisposalMethod::*;
        let disposal = match frame.dispose {
            Any => "any",
            Keep => "keep",
            Background => "background",
            Previous => "previous",
        };
        eprintln!(
            "Written {} ({}x{}@{}x{} delay={} {})",
            output_path, frame.width, frame.height, frame.top, frame.left, frame.delay, disposal
        );
    }
    Ok(())
}
