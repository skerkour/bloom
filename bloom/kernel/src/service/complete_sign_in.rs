use super::{CompleteSignInInput, Me, Service, SignedIn};
use crate::{consts, errors::kernel::Error, Actor};
use consts::TwoFaMethod;
use stdx::tokio::time::delay_for;
use stdx::{
    chrono::{Duration, Utc},
    crypto,
    rand::{thread_rng, Rng},
    sync::threadpool::spawn_blocking,
};

impl Service {
    pub async fn complete_sign_in(&self, actor: Actor, input: CompleteSignInInput) -> Result<SignedIn, crate::Error> {
        if actor.is_some() {
            return Err(Error::MustNotBeAuthenticated.into());
        }

        // sleep to prevent spam and bruteforce
        let sleep = thread_rng().gen_range(consts::SLEEP_MIN..consts::SLEEP_MAX);
        delay_for(sleep).await;

        let mut pending_session = self
            .repo
            .find_pending_session_by_id(&self.db, input.pending_session_id)
            .await?;

        // clean and validate data
        let code = input.code.trim().to_lowercase().replace("-", "");
        let now = Utc::now();

        if pending_session.failed_attempts + 1 >= consts::SIGN_IN_MAX_FAILED_ATTEMPTS {
            return Err(Error::MaxSignInAttempsReached.into());
        }

        let created_at_plus_30_mins = pending_session.created_at + Duration::minutes(30);
        if now >= created_at_plus_30_mins {
            return Err(Error::SignInCodeExpired.into());
        }

        let code_hash = pending_session.code_hash.clone();
        let is_code_valid = spawn_blocking(move || crypto::verify_password(&code, &code_hash)).await?;

        if !is_code_valid {
            pending_session.failed_attempts += 1;
            let _ = self.repo.update_pending_session(&self.db, &pending_session).await;
            return Err(Error::InvalidSignInCode.into());
        }

        let user = self.repo.find_user_by_id(&self.db, pending_session.user_id).await?;

        if user.blocked_at.is_some() {
            return Err(Error::UserIsBcloked.into());
        }

        if user.two_fa_enabled {
            pending_session.two_fa_verified = true;
            pending_session.updated_at = now;
            self.repo.update_pending_session(&self.db, &pending_session).await?;
            return Ok(SignedIn::TwoFa(TwoFaMethod::Totp));
        }

        // otherwise, we continue the normal login flow
        let new_session = self.new_session(user.id).await?;

        let groups = self.repo.find_groups_for_user(&self.db, user.id).await?;

        // create a new session and delete pending session
        let mut tx = self.db.begin().await?;

        self.repo.delete_pending_session(&mut tx, pending_session.id).await?;

        self.repo.create_session(&mut tx, &new_session.session).await?;

        tx.commit().await?;

        let me = Me {
            session: new_session.session,
            user,
            groups,
        };
        Ok(SignedIn::Success {
            me,
            token: new_session.token,
        })
    }
}
