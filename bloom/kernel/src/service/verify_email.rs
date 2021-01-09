use super::{Service, VerifyEmailInput};
use crate::{consts, errors::kernel::Error, Actor};
use stdx::tokio::time::delay_for;
use stdx::{
    chrono::{Duration, Utc},
    crypto,
    rand::{thread_rng, Rng},
    sync::threadpool::spawn_blocking,
};

impl Service {
    pub async fn verify_email(&self, actor: Actor, input: VerifyEmailInput) -> Result<(), crate::Error> {
        let mut actor = self.current_user(actor)?;

        // sleep to prevent spam and bruteforce
        let sleep = thread_rng().gen_range(consts::SLEEP_MIN..consts::SLEEP_MAX);
        delay_for(sleep).await;

        let mut pending_email = self
            .repo
            .find_pending_email_by_id(&self.db, input.pending_email_id)
            .await?;

        if pending_email.failed_attempts >= consts::VERIFY_EMAIL_MAX_FAILED_ATTEMPTS {
            return Err(Error::MaxEmailVerificationAttempsReached.into());
        }

        let code = input.code.trim().to_lowercase().replace("-", "");
        let now = Utc::now();

        let created_at_plus_30_mins = pending_email.created_at + Duration::minutes(30);
        if now >= created_at_plus_30_mins {
            return Err(Error::EmailVerificationCodeExpired.into());
        }

        let code_hash = pending_email.code_hash.clone();
        let is_code_valid = spawn_blocking(move || crypto::verify_password(&code, &code_hash)).await?;

        if !is_code_valid {
            pending_email.failed_attempts += 1;
            let _ = self.repo.update_pending_email(&self.db, &pending_email).await;
            return Err(Error::InvalidRegistrationCode.into());
        }

        let mut tx = self.db.begin().await?;

        let find_existing_email = self.repo.find_user_by_email(&mut tx, &pending_email.email).await;
        match find_existing_email {
            Ok(_) => Err(Error::EmailAlreadyExists),
            Err(Error::UserNotFound) => Ok(()),
            Err(err) => Err(err),
        }?;

        let old_email = actor.email.clone();
        actor.email = pending_email.email;
        actor.updated_at = Utc::now();

        self.repo.update_user(&mut tx, &actor).await?;

        self.repo.delete_pending_email(&mut tx, pending_email.id).await?;

        tx.commit().await?;

        let job = crate::domain::messages::Message::KernelSendEmailChangedEmail {
            email: old_email,
            name: actor.name,
            new_email: actor.email,
        };
        let _ = self.queue.push(job, None).await; // TODO: log error?

        Ok(())
    }
}
