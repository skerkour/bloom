use crate::{consts, Actor, Error, Service};
use stdx::{
    base64,
    image::{self, imageops::FilterType, DynamicImage, Luma},
    qrcode::QrCode,
    sync::threadpool::spawn_blocking,
};

impl Service {
    pub async fn qr_code(&self, actor: Actor, input: String) -> Result<String, Error> {
        let _ = self.current_user(actor)?;
        self.validate_qr_code_input(&input)?;

        let base64_encoded_qr_code_image = spawn_blocking(move || -> Result<String, Error> {
            let code = QrCode::new(input.as_bytes()).map_err(|err| Error::Internal(err.to_string()))?;

            let qr_code_image = DynamicImage::ImageLuma8(code.render::<Luma<u8>>().build());
            let qr_code_image = qr_code_image.resize(
                consts::TOTP_QR_CODE_SIZE,
                consts::TOTP_QR_CODE_SIZE,
                FilterType::Lanczos3,
            );

            let mut qr_code_buffer: Vec<u8> = Vec::new();
            qr_code_image.write_to(
                &mut qr_code_buffer,
                image::ImageOutputFormat::Jpeg(consts::TOTP_QR_JPEG_QUALITY),
            )?;

            let base64_encoded_qr_code_image = base64::encode(qr_code_buffer);
            Ok(base64_encoded_qr_code_image)
        })
        .await??;

        Ok(base64_encoded_qr_code_image)
    }
}
