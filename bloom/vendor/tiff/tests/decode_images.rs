extern crate tiff;

use tiff::decoder::{ifd, Decoder, DecodingResult};
use tiff::ColorType;

use std::fs::File;
use std::path::PathBuf;

const TEST_IMAGE_DIR: &str = "./tests/images/";

macro_rules! test_image_sum {
    ($name:ident, $buffer:ident, $sum_ty:ty) => {
        fn $name(file: &str, expected_type: ColorType, expected_sum: $sum_ty) {
            let path = PathBuf::from(TEST_IMAGE_DIR).join(file);
            let img_file = File::open(path).expect("Cannot find test image!");
            let mut decoder = Decoder::new(img_file).expect("Cannot create decoder");
            assert_eq!(decoder.colortype().unwrap(), expected_type);
            let img_res = decoder.read_image().unwrap();

            match img_res {
                DecodingResult::$buffer(res) => {
                    let sum: $sum_ty = res.into_iter().map(<$sum_ty>::from).sum();
                    assert_eq!(sum, expected_sum);
                }
                _ => panic!("Wrong bit depth"),
            }
        }
    };
}

test_image_sum!(test_image_sum_u8, U8, u64);
test_image_sum!(test_image_sum_u16, U16, u64);
test_image_sum!(test_image_sum_u32, U32, u64);
test_image_sum!(test_image_sum_u64, U64, u64);
test_image_sum!(test_image_sum_f32, F32, f32);
test_image_sum!(test_image_sum_f64, F64, f64);

/// Tests that a decoder can be constructed for an image and the color type
/// read from the IFD and is of the appropriate type, but the type is
/// unsupported.
fn test_image_color_type_unsupported(file: &str, expected_type: ColorType) {
    let path = PathBuf::from(TEST_IMAGE_DIR).join(file);
    let img_file = File::open(path).expect("Cannot find test image!");
    let mut decoder = Decoder::new(img_file).expect("Cannot create decoder");
    assert_eq!(decoder.colortype().unwrap(), expected_type);
    assert!(match decoder.read_image() {
        Err(tiff::TiffError::UnsupportedError(
            tiff::TiffUnsupportedError::UnsupportedColorType(_),
        )) => true,
        _ => false,
    });
}

#[test]
fn test_cmyk_u8() {
    test_image_sum_u8("cmyk-3c-8b.tiff", ColorType::CMYK(8), 8522658);
}

#[test]
fn test_cmyk_u16() {
    test_image_sum_u16("cmyk-3c-16b.tiff", ColorType::CMYK(16), 2181426827);
}

#[test]
fn test_cmyk_f32() {
    test_image_sum_f32("cmyk-3c-32b-float.tiff", ColorType::CMYK(32), 496.0405);
}

#[test]
fn test_gray_u8() {
    test_image_sum_u8("minisblack-1c-8b.tiff", ColorType::Gray(8), 2840893);
}

#[test]
fn test_gray_u12() {
    test_image_color_type_unsupported("12bit.cropped.tiff", ColorType::Gray(12));
}

#[test]
fn test_gray_u16() {
    test_image_sum_u16("minisblack-1c-16b.tiff", ColorType::Gray(16), 733126239);
}

#[test]
fn test_gray_u32() {
    test_image_sum_u32("gradient-1c-32b.tiff", ColorType::Gray(32), 549892913787);
}

#[test]
fn test_gray_u64() {
    test_image_sum_u64("gradient-1c-64b.tiff", ColorType::Gray(64), 549892913787);
}

#[test]
fn test_gray_f32() {
    test_image_sum_f32("gradient-1c-32b-float.tiff", ColorType::Gray(32), 128.03194);
}

#[test]
fn test_gray_f64() {
    test_image_sum_f64(
        "gradient-1c-64b-float.tiff",
        ColorType::Gray(64),
        128.0319210877642,
    );
}

#[test]
fn test_rgb_u8() {
    test_image_sum_u8("rgb-3c-8b.tiff", ColorType::RGB(8), 7842108);
}

#[test]
fn test_rgb_u12() {
    test_image_color_type_unsupported("12bit.cropped.rgb.tiff", ColorType::RGB(12));
}

#[test]
fn test_rgb_u16() {
    test_image_sum_u16("rgb-3c-16b.tiff", ColorType::RGB(16), 2024349944);
}

#[test]
fn test_rgb_u32() {
    test_image_sum_u32("gradient-3c-32b.tiff", ColorType::RGB(32), 2030834111716);
}

#[test]
fn test_rgb_u64() {
    test_image_sum_u64("gradient-3c-64b.tiff", ColorType::RGB(64), 2030834111716);
}

#[test]
fn test_rgb_f32() {
    test_image_sum_f32("gradient-3c-32b-float.tiff", ColorType::RGB(32), 472.8405);
}

#[test]
fn test_string_tags() {
    // these files have null-terminated strings for their Software tag. One has extra bytes after
    // the null byte, so we check both to ensure that we're truncating properly
    let filenames = ["minisblack-1c-16b.tiff", "rgb-3c-16b.tiff"];
    for filename in filenames.iter() {
        let path = PathBuf::from(TEST_IMAGE_DIR).join(filename);
        let img_file = File::open(path).expect("Cannot find test image!");
        let mut decoder = Decoder::new(img_file).expect("Cannot create decoder");
        let software = decoder.get_tag(tiff::tags::Tag::Software).unwrap();
        match software {
            ifd::Value::Ascii(s) => assert_eq!(
                &s,
                "GraphicsMagick 1.2 unreleased Q16 http://www.GraphicsMagick.org/"
            ),
            _ => assert!(false),
        };
    }
}

#[test]
fn test_decode_data() {
    let mut image_data = Vec::new();
    for x in 0..100 {
        for y in 0..100u8 {
            let val = x + y;
            image_data.push(val);
            image_data.push(val);
            image_data.push(val);
        }
    }
    let file = File::open("./tests/decodedata-rgb-3c-8b.tiff").unwrap();
    let mut decoder = Decoder::new(file).unwrap();
    assert_eq!(decoder.colortype().unwrap(), ColorType::RGB(8));
    assert_eq!(decoder.dimensions().unwrap(), (100, 100));
    if let DecodingResult::U8(img_res) = decoder.read_image().unwrap() {
        assert_eq!(image_data, img_res);
    } else {
        panic!("Wrong data type");
    }
}

#[test]
fn issue_69() {
    test_image_sum_u16("issue_69_lzw.tiff", ColorType::Gray(16), 1015486);
    test_image_sum_u16("issue_69_packbits.tiff", ColorType::Gray(16), 1015486);
}

// TODO: GrayA support
//#[test]
//fn test_gray_alpha_u8()
//{
//let img_file = File::open("./tests/images/minisblack-2c-8b-alpha.tiff").expect("Cannot find test image!");
//let mut decoder = Decoder::new(img_file).expect("Cannot create decoder");
//assert_eq!(decoder.colortype().unwrap(), ColorType::GrayA(8));
//let img_res = decoder.read_image();
//assert!(img_res.is_ok());
//}
