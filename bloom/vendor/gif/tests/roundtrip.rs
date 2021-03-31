use gif::{Decoder, Encoder, Frame};

#[test]
fn encode_roundtrip() {
    const ORIGINAL: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/samples/2x2.gif"));
    round_trip_from_image(ORIGINAL);
}

fn round_trip_from_image(original: &[u8]) {
    let (width, height, global_palette);
    let frames: Vec<Frame> = {
        let mut decoder = Decoder::new(original).unwrap();
        width = decoder.width();
        height = decoder.height();
        global_palette = decoder
            .global_palette()
            .unwrap_or_default()
            .to_vec();
        core::iter::from_fn(move || {
            decoder.read_next_frame().unwrap().cloned()
        }).collect()
    };

    let mut buffer = vec![];
    {
        let mut encoder = Encoder::new(&mut buffer, width, height, &global_palette).unwrap();
        for frame in &frames {
            encoder.write_frame(frame).unwrap();
        }
    }

    {
        let mut decoder = Decoder::new(&buffer[..]).expect("Invalid info encoded");
        assert_eq!(decoder.width(), width);
        assert_eq!(decoder.height(), height);
        assert_eq!(global_palette, decoder.global_palette().unwrap_or_default());
        let new_frames: Vec<_> = core::iter::from_fn(move || {
            decoder.read_next_frame().unwrap().cloned()
        }).collect();
        assert_eq!(new_frames.len(), frames.len(), "Diverging number of frames");
        for (new, reference) in new_frames.iter().zip(&frames) {
            assert_eq!(new.delay, reference.delay);
            assert_eq!(new.dispose, reference.dispose);
            assert_eq!(new.transparent, reference.transparent);
            assert_eq!(new.needs_user_input, reference.needs_user_input);
            assert_eq!(new.top, reference.top);
            assert_eq!(new.left, reference.left);
            assert_eq!(new.width, reference.width);
            assert_eq!(new.height, reference.height);
            assert_eq!(new.interlaced, reference.interlaced);
            assert_eq!(new.palette, reference.palette);
            assert_eq!(new.buffer, reference.buffer);
        }
    }
}
