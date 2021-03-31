use gif::{DecodeOptions, DisposalMethod, Encoder, Frame};

#[test]
fn frame_consistency_is_configurable() {
    let image = create_image_with_oob_frames();

    {
        let options = DecodeOptions::new();
        let mut data = image.as_slice();
        let mut decoder = options.clone().read_info(&mut data).unwrap();
        assert!(decoder.read_next_frame().is_ok());
        assert!(decoder.read_next_frame().is_ok());
    }

    {
        let mut options = DecodeOptions::new();
        options.check_frame_consistency(true);
        let mut data = image.as_slice();
        let mut decoder = options.clone().read_info(&mut data).unwrap();
        assert!(decoder.read_next_frame().is_ok());
        assert!(decoder.read_next_frame().is_err());
    }

    {
        let mut options = DecodeOptions::new();
        options.check_frame_consistency(false);
        let mut data = image.as_slice();
        let mut decoder = options.clone().read_info(&mut data).unwrap();
        assert!(decoder.read_next_frame().is_ok());
        assert!(decoder.read_next_frame().is_ok());
    }
}

fn create_image_with_oob_frames() -> Vec<u8> {
    let mut data = vec![];
    let mut encoder = Encoder::new(&mut data, 2, 2, &[0, 0, 0]).unwrap();

    let mut frame = Frame {
        delay: 1,
        dispose: DisposalMethod::Any,
        transparent: None,
        needs_user_input: false,
        top: 0,
        left: 0,
        width: 2,
        height: 2,
        interlaced: false,
        palette: None,
        buffer: vec![0, 0, 0, 0].into(),
    };

    encoder.write_frame(&frame).unwrap();
    frame.top = 1;
    frame.left = 1;
    encoder.write_frame(&frame).unwrap();

    drop(encoder);
    data
}

#[test]
fn check_for_end_code_is_configurable() {
    // In this particular image, the image data of the 62nd frame has no end code.
    let image: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/samples/gifplayer-muybridge.gif"));

    {
        let options = DecodeOptions::new();
        let mut decoder = options.clone().read_info(&image[..]).unwrap();
        for _ in 0..61 {
            assert!(decoder.read_next_frame().is_ok());
        }
        assert!(decoder.read_next_frame().is_ok());
    }

    {
        let mut options = DecodeOptions::new();
        options.check_lzw_end_code(true);
        let mut decoder = options.clone().read_info(&image[..]).unwrap();
        for _ in 0..61 {
            assert!(decoder.read_next_frame().is_ok());
        }
        assert!(decoder.read_next_frame().is_err());
    }
}
