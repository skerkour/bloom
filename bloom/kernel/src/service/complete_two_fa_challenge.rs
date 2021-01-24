use super::{CompleteTwoFaChallengeInput, Me, Service, SignedIn};
use crate::{consts, errors::kernel::Error, Actor};
use stdx::{
    chrono::{Duration, Utc},
    crypto,
    otp::totp,
    rand::{thread_rng, Rng},
    sync::threadpool::spawn_blocking,
};
use stdx::{log::error, tokio::time::delay_for};

impl Service {
    pub async fn complete_two_fa_challenge(
        &self,
        actor: Actor,
        input: CompleteTwoFaChallengeInput,
    ) -> Result<SignedIn, crate::Error> {
        if actor.is_some() {
            return Err(Error::MustNotBeAuthenticated.into());
        }
        let now = Utc::now();

        // sleep to prevent spam and bruteforce
        let sleep = thread_rng().gen_range(consts::SLEEP_MIN..consts::SLEEP_MAX);
        delay_for(sleep).await;

        let mut pending_session = self
            .repo
            .find_pending_session_by_id(&self.db, input.pending_session_id)
            .await?;

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

        let actor = self.repo.find_user_by_id(&self.db, pending_session.user_id).await?;

        // clean and validate data
        let two_fa_code = input.code.trim().to_lowercase().replace("-", "");

        let master_key = self.config.master_key.clone();
        let encrypted_totp_secret = actor
            .encrypted_totp_secret
            .clone()
            .expect("kernel.complete_two_fa_challenge: accessing actor.encrypted_totp_secret");
        let totp_secret_nonce = actor
            .totp_secret_nonce
            .clone()
            .expect("kernel.complete_two_fa_challenge: accessing actor.totp_secret_nonce");
        let ad: Vec<u8> = actor.id.as_bytes()[..].into();
        let totp_secret = match spawn_blocking(move || {
            crypto::aead_decrypt(&master_key, &encrypted_totp_secret, &totp_secret_nonce, &ad)
        })
        .await?
        {
            Ok(res) => res,
            Err(err) => {
                error!("kernel.complete_two_fa_challenge: decrypting totp secret: {}", err);
                return Err(err.into());
            }
        };

        let totp_secret = String::from_utf8(totp_secret)?;

        if !(totp::validate(two_fa_code, totp_secret).await?) {
            pending_session.failed_attempts += 1;
            let _ = self.repo.update_pending_session(&self.db, &pending_session).await;
            return Err(Error::TwoFACodeIsNotValid.into());
        }

        let new_session = self.new_session(actor.id).await?;

        let groups = self.repo.find_groups_for_user(&self.db, actor.id).await?;

        // create a new session and delete pending session
        let mut tx = self.db.begin().await?;

        self.repo.delete_pending_session(&mut tx, pending_session.id).await?;

        self.repo.create_session(&mut tx, &new_session.session).await?;

        tx.commit().await?;

        let me = Me {
            session: new_session.session,
            user: actor,
            groups,
        };
        Ok(SignedIn::Success {
            me,
            token: new_session.token,
        })
    }
}
