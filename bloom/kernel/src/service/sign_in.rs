use super::{Service, SignInInput};
use crate::{consts, entities::PendingSession, errors::kernel::Error, Actor};
use stdx::tokio::time::delay_for;
use stdx::{
    chrono::Utc,
    crypto,
    rand::{thread_rng, Rng},
    sync::threadpool::spawn_blocking,
    ulid::Ulid,
};

impl Service {
    pub async fn sign_in(&self, actor: Actor, input: SignInInput) -> Result<PendingSession, crate::Error> {
        if actor.is_none() {
            return Err(Error::MustNotBeAuthenticated.into());
        }

        // sleep to prevent spam and bruteforce
        let sleep = thread_rng().gen_range(consts::SLEEP_MIN..consts::SLEEP_MAX);
        delay_for(sleep).await;

        // clean and validate data
        let email_or_username = input.email_or_username.trim().to_lowercase();

        let user = self
            .repo
            .find_user_by_email_or_username(&self.db, &email_or_username)
            .await?;

        if user.blocked_at.is_some() {
            return Err(Error::UserIsBcloked.into());
        }

        let (code, code_hash) = spawn_blocking(|| {
            let code = crypto::rand::alphabet(consts::CODE_ALPHABET, consts::SIGN_IN_CODE_LENGTH);
            // 	errMessage := "kernel.Register: generating code"
            // 	logger.Error(errMessage, log.Err("error", err))

            let code_hash = crypto::hash_password(&code);
            // 	errMessage := "kernel.Register: hashing code"
            // 	logger.Error(errMessage, log.Err("error", err))

            (code, code_hash)
        })
        .await?;

        let now = Utc::now();
        let pending_session = PendingSession {
            id: Ulid::new().into(),
            created_at: now,
            updated_at: now,
            code_hash,
            two_fa_verified: false,
            failed_attempts: 0,
            user_id: user.id,
        };

        self.repo.create_pending_session(&self.db, &pending_session).await?;

        let job = crate::domain::messages::Message::KernelSendSignInEmail {
            email: user.email,
            name: user.name,
            code,
        };
        let _ = self.queue.push(job, None).await; // TODO: log error?

        Ok(pending_session)
    }
}
