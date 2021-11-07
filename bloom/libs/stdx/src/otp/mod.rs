use crate::qrcode::QrCode;
use image::{imageops::FilterType, DynamicImage, Luma};

pub mod totp;

#[derive(thiserror::Error, Debug, Clone, Copy)]
pub enum Error {
    #[error("otp: GeneratingQRCode")]
    GeneratingQRCode,

    #[error("otp: DecodingSecret")]
    DecodingSecret,

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

    pub fn image(&self, width: u32, height: u32) -> Result<image::DynamicImage, Error> {
        let code = QrCode::new(self.url.as_bytes()).map_err(|_| Error::GeneratingQRCode)?;

        // Render the bits into an image.
        let image = code.render::<Luma<u8>>().min_dimensions(width, height).build();
        let image = DynamicImage::ImageLuma8(image);
        let image = image.resize(width, height, FilterType::Lanczos3);

        Ok(image)
    }
}
