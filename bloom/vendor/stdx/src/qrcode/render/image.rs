use crate::qrcode::render::{Canvas, Pixel};
use crate::qrcode::types::Color;

use image::{ImageBuffer, Luma, LumaA, Pixel as ImagePixel, Primitive, Rgb, Rgba};

macro_rules! impl_pixel_for_image_pixel {
    ($p:ident<$s:ident>: $c:pat => $d:expr) => {
        impl<$s: Primitive + 'static> Pixel for $p<$s> {
            type Image = ImageBuffer<Self, Vec<S>>;
            type Canvas = (Self, Self::Image);

            fn default_color(color: Color) -> Self {
                match color.select($s::zero(), $s::max_value()) {
                    $c => $p($d),
                }
            }
        }
    };
}

impl_pixel_for_image_pixel! { Luma<S>: p => [p] }
impl_pixel_for_image_pixel! { LumaA<S>: p => [p, S::max_value()] }
impl_pixel_for_image_pixel! { Rgb<S>: p => [p, p, p] }
impl_pixel_for_image_pixel! { Rgba<S>: p => [p, p, p, S::max_value()] }

impl<P: ImagePixel + 'static> Canvas for (P, ImageBuffer<P, Vec<P::Subpixel>>) {
    type Pixel = P;
    type Image = ImageBuffer<P, Vec<P::Subpixel>>;

    fn new(width: u32, height: u32, dark_pixel: P, light_pixel: P) -> Self {
        (dark_pixel, ImageBuffer::from_pixel(width, height, light_pixel))
    }

    fn draw_dark_pixel(&mut self, x: u32, y: u32) {
        self.1.put_pixel(x, y, self.0);
    }

    fn into_image(self) -> ImageBuffer<P, Vec<P::Subpixel>> {
        self.1
    }
}

#[cfg(test)]
mod render_tests {
    use crate::qrcode::render::Renderer;
    use crate::qrcode::types::Color;
    use image::{Luma, Rgba};

    #[test]
    fn test_render_luma8_unsized() {
        let image = Renderer::<Luma<u8>>::new(
            &[
                Color::Light,
                Color::Dark,
                Color::Dark,
                //
                Color::Dark,
                Color::Light,
                Color::Light,
                //
                Color::Light,
                Color::Dark,
                Color::Light,
            ],
            3,
            1,
        )
        .module_dimensions(1, 1)
        .build();

        #[rustfmt::skip]
        let expected = [
            255, 255, 255, 255, 255,
            255, 255,   0,   0, 255,
            255,   0, 255, 255, 255,
            255, 255,   0, 255, 255,
            255, 255, 255, 255, 255,
        ];
        assert_eq!(image.into_raw(), expected);
    }

    #[test]
    fn test_render_rgba_unsized() {
        let image = Renderer::<Rgba<u8>>::new(&[Color::Light, Color::Dark, Color::Dark, Color::Dark], 2, 1)
            .module_dimensions(1, 1)
            .build();

        #[rustfmt::skip]
        let expected: &[u8] = &[
            255,255,255,255, 255,255,255,255, 255,255,255,255, 255,255,255,255,
            255,255,255,255, 255,255,255,255,   0,  0,  0,255, 255,255,255,255,
            255,255,255,255,   0,  0,  0,255,   0,  0,  0,255, 255,255,255,255,
            255,255,255,255, 255,255,255,255, 255,255,255,255, 255,255,255,255,
        ];

        assert_eq!(image.into_raw(), expected);
    }

    #[test]
    fn test_render_resized_min() {
        let image = Renderer::<Luma<u8>>::new(&[Color::Dark, Color::Light, Color::Light, Color::Dark], 2, 1)
            .min_dimensions(10, 10)
            .build();

        #[rustfmt::skip]
        let expected: &[u8] = &[
            255,255,255, 255,255,255, 255,255,255, 255,255,255,
            255,255,255, 255,255,255, 255,255,255, 255,255,255,
            255,255,255, 255,255,255, 255,255,255, 255,255,255,

            255,255,255,   0,  0,  0, 255,255,255, 255,255,255,
            255,255,255,   0,  0,  0, 255,255,255, 255,255,255,
            255,255,255,   0,  0,  0, 255,255,255, 255,255,255,

            255,255,255, 255,255,255,   0,  0,  0, 255,255,255,
            255,255,255, 255,255,255,   0,  0,  0, 255,255,255,
            255,255,255, 255,255,255,   0,  0,  0, 255,255,255,

            255,255,255, 255,255,255, 255,255,255, 255,255,255,
            255,255,255, 255,255,255, 255,255,255, 255,255,255,
            255,255,255, 255,255,255, 255,255,255, 255,255,255,
        ];

        assert_eq!(image.dimensions(), (12, 12));
        assert_eq!(image.into_raw(), expected);
    }

    #[test]
    fn test_render_resized_max() {
        let image = Renderer::<Luma<u8>>::new(&[Color::Dark, Color::Light, Color::Light, Color::Dark], 2, 1)
            .max_dimensions(10, 5)
            .build();

        #[rustfmt::skip]
        let expected: &[u8] = &[
            255,255, 255,255, 255,255, 255,255,

            255,255,   0,  0, 255,255, 255,255,

            255,255, 255,255,   0,  0, 255,255,

            255,255, 255,255, 255,255, 255,255,
        ];

        assert_eq!(image.dimensions(), (8, 4));
        assert_eq!(image.into_raw(), expected);
    }
}
