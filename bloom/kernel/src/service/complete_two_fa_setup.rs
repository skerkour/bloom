use super::Service;
use crate::{
    consts::{self, TwoFaMethod},
    errors::kernel::Error,
    Actor,
};
use std::io::BufWriter;
use stdx::{base64, chrono::Utc, crypto, image::codecs::jpeg::JpegEncoder, log::error, otp::totp};

impl Service {
    pub async fn complete_two_fa_setup(&self, actor: Actor) -> Result<String, crate::Error> {
        let mut actor = self.current_user(actor)?;

        if actor.two_fa_enabled {
            return Err(Error::TwoFaAlreadyEnabled.into());
        }

        // generate secret
        let totp_key = totp::generate(consts::TOTP_ISSUER, &actor.username);

        // encrypt secret
        let (encrypted_totp_secret, nonce) = crypto::aead_encrypt(
            &self.config.master_key,
            totp_key.secret().as_bytes(),
            &actor.id.as_bytes()[..],
        );
        // TODO
        // if err != nil {
        //     errMessage := "kernel.SetupTwoFA: encrypting secret"
        //     logger.Error(errMessage, log.Err("error", err))
        //     err = errors.Internal(errMessage, err)
        //     return
        // }

        actor.encrypted_totp_secret = Some(encrypted_totp_secret);
        actor.totp_secret_nonce = Some(nonce);
        actor.two_fa_method = Some(TwoFaMethod::Totp);
        actor.updated_at = Utc::now();
        self.repo.update_user(&self.db, &actor).await?;

        let qr_code_image = match totp_key.image(consts::TOTP_QR_CODE_SIZE, consts::TOTP_QR_CODE_SIZE) {
            Ok(res) => res,
            Err(err) => {
                error!("kernel.complete_two_fa_setup: generating TOTP QR code: {}", err);
                return Err(err.into());
            }
        };

        let ref mut qr_code_buffer = BufWriter::new(Vec::new());
        let mut jpeg_encoder = JpegEncoder::new_with_quality(qr_code_buffer, consts::TOTP_QR_JPEG_QUALITY);
        jpeg_encoder.encode(
            qr_code_image.as_bytes(),
            consts::TOTP_QR_CODE_SIZE,
            consts::TOTP_QR_CODE_SIZE,
            qr_code_image.color(),
        )?;
        // TODO
        // if err != nil {
        //     errMessage := "kernel.SetupTwoFA: encoding QR code to jpeg"
        //     logger.Error(errMessage, log.Err("error", err))
        //     err = errors.Internal(errMessage, err)
        //     return
        // }

        let base64_encoded_qr_code_image = base64::encode(qr_code_buffer.buffer());
        Ok(base64_encoded_qr_code_image)
    }
}
