use super::Service;
use crate::{
    consts::{self, TwoFaMethod},
    errors::kernel::Error,
    Actor,
};
use stdx::{base64, chrono::Utc, crypto, image, log::error, otp::totp, sync::threadpool::spawn_blocking};

impl Service {
    pub async fn setup_two_fa(&self, actor: Actor) -> Result<String, crate::Error> {
        let mut actor = self.current_user(actor)?;

        if actor.two_fa_enabled {
            return Err(Error::TwoFaAlreadyEnabled.into());
        }

        // generate secret
        let totp_key = totp::generate(consts::TOTP_ISSUER.to_string(), actor.username.clone()).await?;

        // encrypt secret
        let master_key = self.config.master_key.clone();
        let plaintext: Vec<u8> = totp_key.secret().as_bytes().into();
        let ad: Vec<u8> = actor.id.as_bytes()[..].into();
        let (encrypted_totp_secret, nonce) =
            match spawn_blocking(move || crypto::aead_encrypt(&master_key, &plaintext, &ad)).await? {
                Ok(res) => res,
                Err(err) => {
                    error!("kernel.setup_two_fa: encrypting totp secret: {}", err);
                    return Err(err.into());
                }
            };

        actor.encrypted_totp_secret = Some(encrypted_totp_secret);
        actor.totp_secret_nonce = Some(nonce);
        actor.two_fa_method = Some(TwoFaMethod::Totp);
        actor.updated_at = Utc::now();
        self.repo.update_user(&self.db, &actor).await?;

        let qr_code_image = match totp_key.image(consts::TOTP_QR_CODE_SIZE, consts::TOTP_QR_CODE_SIZE) {
            Ok(res) => res,
            Err(err) => {
                error!("kernel.setup_two_fa: generating TOTP QR code: {}", err);
                return Err(err.into());
            }
        };

        let mut qr_code_buffer: Vec<u8> = Vec::new();
        qr_code_image.write_to(
            &mut qr_code_buffer,
            image::ImageOutputFormat::Jpeg(consts::TOTP_QR_JPEG_QUALITY),
        )?;

        let base64_encoded_qr_code_image = base64::encode(qr_code_buffer);
        Ok(base64_encoded_qr_code_image)
    }
}
