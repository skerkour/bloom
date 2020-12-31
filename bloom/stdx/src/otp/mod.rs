pub mod totp;

#[derive(Debug, Clone)]
pub struct Key {
    // TODO
}

impl Key {
    pub fn secret(&self) -> String {
        unimplemented!(); // TODO
    }

    pub fn image(&self, _width: u32, _height: u32) -> image::DynamicImage {
        unimplemented!(); // TODO
    }
}
