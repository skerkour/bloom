use super::{CompleteTwoFaChallengeInput, Service, SignedIn};
use crate::{consts, errors::kernel::Error, Actor};
use stdx::tokio::time::delay_for;
use stdx::{
    chrono::{Duration, Utc},
    crypto,
    otp::totp,
    rand::{thread_rng, Rng},
};

impl Service {
    pub async fn complete_two_fa_challenge(
        &self,
        actor: Actor,
        input: CompleteTwoFaChallengeInput,
    ) -> Result<SignedIn, crate::Error> {
        let actor = self.current_user(actor)?;
        let now = Utc::now();

        // sleep to prevent spam and bruteforce
        let sleep = thread_rng().gen_range(consts::SLEEP_MIN..consts::SLEEP_MAX);
        delay_for(sleep).await;

        let mut pending_session = self
            .repo
            .find_pending_session_by_id(&self.db, input.pending_session_id)
            .await?;

        if pending_session.user_id != actor.id {
            return Err(Error::PermissionDenied.into());
        }

        if pending_session.failed_attempts + 1 >= consts::SIGN_IN_MAX_FAILED_ATTEMPTS {
            return Err(Error::MaxSignInAttempsReached.into());
        }

        let created_at_plus_35_mins = pending_session.created_at + Duration::minutes(35);
        if now >= created_at_plus_35_mins {
            return Err(Error::SignInCodeExpired.into());
        }

        if !pending_session.two_fa_verified {
            return Err(Error::PermissionDenied.into());
        }

        // clean and validate data
        let two_fa_code = input.code.trim().to_lowercase().replace("-", "");

        let totp_secret = crypto::aead_decrypt(
            &self.config.master_key,
            &actor
                .encrypted_totp_secret
                .clone()
                .expect("kernel/complete_two_fa_challenge: accessing actor.encrypted_totp_secret"),
            &actor
                .totp_secret_nonce
                .clone()
                .expect("kernel/complete_two_fa_challenge: accessing actor.totp_secret_nonce"),
            &actor.id.as_bytes()[..],
        );
        // TODO
        // if err != nil {
        //     errMessage := "kernel.CompleteTwoFA: decrypting TOTP secret"
        //     logger.Error(errMessage, log.Err("error", err))
        //     err = errors.Internal(errMessage, err)
        //     return
        // }

        let totp_secret = String::from_utf8(totp_secret)?;

        if !totp::validate(&two_fa_code, &totp_secret) {
            pending_session.failed_attempts += 1;
            let _ = self.repo.update_pending_session(&self.db, &pending_session).await;
            return Err(Error::TwoFACodeIsNotValid.into());
        }

        let session = self.new_session(actor.id).await?;

        // create a new session and delete pending session
        let mut tx = self.db.begin().await?;

        self.repo.delete_pending_session(&mut tx, pending_session.id).await?;

        self.repo.create_session(&mut tx, &session).await?;

        tx.commit().await?;

        Ok(SignedIn::Success { session, user: actor })
    }
}
