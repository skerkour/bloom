use crate::qrcode::QrCode;
use image::{DynamicImage, Luma};

pub mod totp;

#[derive(thiserror::Error, Debug, Clone, Copy)]
pub enum Error {
    #[error("otp: GeneratingQRCode")]
    GeneratingQRCode,

    #[error("otp: JoinError")]
    JoinError,
}

impl std::convert::From<crate::sync::threadpool::Error> for Error {
    fn from(err: crate::sync::threadpool::Error) -> Self {
        match err {
            _ => Error::JoinError,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Key {
    secret: String,
    url: String,
}

impl Key {
    pub fn new(secret: String, url: String) -> Key {
        Key {
            secret,
            url,
        }
    }

    pub fn secret(&self) -> String {
        self.secret.clone()
    }

    pub fn image(&self, _width: u32, _height: u32) -> Result<image::DynamicImage, Error> {
        let code = QrCode::new(self.url.as_bytes()).map_err(|_| Error::GeneratingQRCode)?;

        // Render the bits into an image.
        let image = code.render::<Luma<u8>>().build();
        Ok(DynamicImage::ImageLuma8(image))
    }
}
