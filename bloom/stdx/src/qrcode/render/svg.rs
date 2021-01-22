//! SVG rendering support.
//!
//! # Example
//!
//! ```
//! use qrcode::QrCode;
//! use qrcode::render::svg;
//!
//! let code = QrCode::new(b"Hello").unwrap();
//! let svg_xml = code.render::<svg::Color>().build();
//! println!("{}", svg_xml);

#![cfg(feature = "svg")]

use std::fmt::Write;
use std::marker::PhantomData;

use crate::qrcode::render::{Canvas as RenderCanvas, Pixel};
use crate::qrcode::types::Color as ModuleColor;

/// An SVG color.
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Color<'a>(pub &'a str);

impl<'a> Pixel for Color<'a> {
    type Canvas = Canvas<'a>;
    type Image = String;

    fn default_color(color: ModuleColor) -> Self {
        Color(color.select("#000", "#fff"))
    }
}

#[doc(hidden)]
pub struct Canvas<'a> {
    svg: String,
    marker: PhantomData<Color<'a>>,
}

impl<'a> RenderCanvas for Canvas<'a> {
    type Pixel = Color<'a>;
    type Image = String;

    fn new(width: u32, height: u32, dark_pixel: Color<'a>, light_pixel: Color<'a>) -> Self {
        Canvas {
            svg: format!(
                concat!(
                    r#"<?xml version="1.0" standalone="yes"?>"#,
                    r#"<svg xmlns="http://www.w3.org/2000/svg""#,
                    r#" version="1.1" width="{w}" height="{h}""#,
                    r#" viewBox="0 0 {w} {h}" shape-rendering="crispEdges">"#,
                    r#"<rect x="0" y="0" width="{w}" height="{h}" fill="{bg}"/>"#,
                    r#"<path fill="{fg}" d=""#,
                ),
                w = width,
                h = height,
                fg = dark_pixel.0,
                bg = light_pixel.0
            ),
            marker: PhantomData,
        }
    }

    fn draw_dark_pixel(&mut self, x: u32, y: u32) {
        self.draw_dark_rect(x, y, 1, 1)
    }

    fn draw_dark_rect(&mut self, left: u32, top: u32, width: u32, height: u32) {
        write!(
            self.svg,
            "M{l} {t}h{w}v{h}H{l}V{t}",
            l = left,
            t = top,
            w = width,
            h = height
        )
        .unwrap();
    }

    fn into_image(mut self) -> String {
        self.svg.push_str(r#""/></svg>"#);
        self.svg
    }
}
