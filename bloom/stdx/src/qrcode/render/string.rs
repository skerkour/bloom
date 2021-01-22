//! String rendering support.

use crate::qrcode::cast::As;
use crate::qrcode::render::{Canvas as RenderCanvas, Pixel};
use crate::qrcode::types::Color;

pub trait Element: Copy {
    fn default_color(color: Color) -> Self;
    fn strlen(self) -> usize;
    fn append_to_string(self, string: &mut String);
}

impl Element for char {
    fn default_color(color: Color) -> Self {
        color.select('\u{2588}', ' ')
    }

    fn strlen(self) -> usize {
        self.len_utf8()
    }

    fn append_to_string(self, string: &mut String) {
        string.push(self)
    }
}

impl<'a> Element for &'a str {
    fn default_color(color: Color) -> Self {
        color.select("\u{2588}", " ")
    }

    fn strlen(self) -> usize {
        self.len()
    }

    fn append_to_string(self, string: &mut String) {
        string.push_str(self)
    }
}

#[doc(hidden)]
pub struct Canvas<P: Element> {
    buffer: Vec<P>,
    width: usize,
    dark_pixel: P,
    dark_cap_inc: isize,
    capacity: isize,
}

impl<P: Element> Pixel for P {
    type Canvas = Canvas<Self>;
    type Image = String;

    fn default_unit_size() -> (u32, u32) {
        (1, 1)
    }

    fn default_color(color: Color) -> Self {
        <Self as Element>::default_color(color)
    }
}

impl<P: Element> RenderCanvas for Canvas<P> {
    type Pixel = P;
    type Image = String;

    fn new(width: u32, height: u32, dark_pixel: P, light_pixel: P) -> Self {
        let width = width.as_usize();
        let height = height.as_isize();
        let dark_cap = dark_pixel.strlen().as_isize();
        let light_cap = light_pixel.strlen().as_isize();
        Self {
            buffer: vec![light_pixel; width * height.as_usize()],
            width,
            dark_pixel,
            dark_cap_inc: dark_cap - light_cap,
            capacity: light_cap * width.as_isize() * height + (height - 1),
        }
    }

    fn draw_dark_pixel(&mut self, x: u32, y: u32) {
        let x = x.as_usize();
        let y = y.as_usize();
        self.capacity += self.dark_cap_inc;
        self.buffer[x + y * self.width] = self.dark_pixel;
    }

    fn into_image(self) -> String {
        let mut result = String::with_capacity(self.capacity.as_usize());
        for (i, pixel) in self.buffer.into_iter().enumerate() {
            if i != 0 && i % self.width == 0 {
                result.push('\n');
            }
            pixel.append_to_string(&mut result);
        }
        result
    }
}

#[test]
fn test_render_to_string() {
    use crate::qrcode::render::Renderer;

    let colors = &[Color::Dark, Color::Light, Color::Light, Color::Dark];
    let image: String = Renderer::<char>::new(colors, 2, 1).build();
    assert_eq!(&image, "    \n \u{2588}  \n  \u{2588} \n    ");

    let image2 = Renderer::new(colors, 2, 1)
        .light_color("A")
        .dark_color("!B!")
        .module_dimensions(2, 2)
        .build();

    assert_eq!(
        &image2,
        "AAAAAAAA\n\
         AAAAAAAA\n\
         AA!B!!B!AAAA\n\
         AA!B!!B!AAAA\n\
         AAAA!B!!B!AA\n\
         AAAA!B!!B!AA\n\
         AAAAAAAA\n\
         AAAAAAAA"
    );
}
