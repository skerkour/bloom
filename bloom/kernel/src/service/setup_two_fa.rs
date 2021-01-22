use stdx::{chrono::Utc, crypto, log::error, otp::totp, sync::threadpool::spawn_blocking};

use super::{EnableTwoFaInput, Service};
use crate::{errors::kernel::Error, Actor};

impl Service {
    pub async fn setup_two_fa(&self, actor: Actor, input: EnableTwoFaInput) -> Result<(), crate::Error> {
        let mut actor = self.current_user(actor)?;

        if actor.encrypted_totp_secret == None || actor.totp_secret_nonce == None || actor.two_fa_method == None {
            return Err(Error::TwoFaIsNotEnabled.into());
        }

        if actor.two_fa_enabled {
            return Err(Error::TwoFaAlreadyEnabled.into());
        }

        let two_fa_code = input.code.trim().to_lowercase().replace("-", "");

        let master_key = self.config.master_key.clone();
        let encrypted_totp_secret = actor
            .encrypted_totp_secret
            .clone()
            .expect("kernel.setup_two_fa: accessing actor.encrypted_totp_secret");
        let totp_secret_nonce = actor
            .totp_secret_nonce
            .clone()
            .expect("kernel.setup_two_fa: accessing actor.totp_secret_nonce");
        let ad: Vec<u8> = actor.id.as_bytes()[..].into();
        let totp_secret = match spawn_blocking(move || {
            crypto::aead_decrypt(&master_key, &encrypted_totp_secret, &totp_secret_nonce, &ad)
        })
        .await?
        {
            Ok(res) => res,
            Err(err) => {
                error!("kernel.setup_two_fa: decrypting totp secret: {}", err);
                return Err(err.into());
            }
        };

        let totp_secret = String::from_utf8(totp_secret)?;
        if !(totp::validate(two_fa_code, totp_secret).await?) {
            return Err(Error::TwoFACodeIsNotValid.into());
        }

        actor.two_fa_enabled = true;
        actor.updated_at = Utc::now();
        self.repo.update_user(&self.db, &actor).await?;

        Ok(())
    }
}
