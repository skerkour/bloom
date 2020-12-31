use stdx::{chrono::Utc, crypto, otp::totp};

use super::{DisableTwoFaInput, Service};
use crate::{entities::User, errors::kernel::Error};

impl Service {
    pub async fn disable_two_fa(&self, actor: Option<User>, input: DisableTwoFaInput) -> Result<(), crate::Error> {
        let mut actor = self.current_user(actor)?;

        if !actor.two_fa_enabled {
            return Err(Error::TwoFaIsNotEnabled.into());
        }

        let two_fa_code = input.code.trim().to_lowercase().replace("-", "");

        let totp_secret = crypto::aead_decrypt(
            &self.config.master_key,
            &actor
                .encrypted_totp_secret
                .clone()
                .expect("kernel/disable_two_fa: accessing actor.encrypted_totp_secret"),
            &actor
                .totp_secret_nonce
                .clone()
                .expect("kernel/disable_two_fa: accessing actor.totp_secret_nonce"),
            &actor.id.as_bytes()[..],
        );
        // TODO
        // if err != nil {
        //     errMessage := "kernel.DisableTwoFA: decrypting TOTP secret"
        //     logger.Error(errMessage, log.Err("error", err))
        //     err = errors.Internal(errMessage, err)
        //     return
        // }

        let totp_secret = String::from_utf8(totp_secret)?;

        if !totp::validate(&two_fa_code, &totp_secret) {
            return Err(Error::TwoFACodeIsNotValid.into());
        }

        actor.two_fa_enabled = false;
        actor.encrypted_totp_secret = None;
        actor.totp_secret_nonce = None;
        actor.two_fa_method = None;
        actor.updated_at = Utc::now();
        self.repo.update_user(&self.db, &actor).await?;

        Ok(())
    }
}
