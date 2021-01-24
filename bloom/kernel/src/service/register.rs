use super::{RegisterInput, Service};
use crate::{consts, entities::PendingUser, errors::kernel::Error, Actor};
use stdx::{
    chrono, crypto,
    rand::{self, thread_rng, Rng},
};
use stdx::{log::error, tokio::time::delay_for};
use stdx::{sync::threadpool::spawn_blocking, ulid::Ulid};

impl Service {
    pub async fn register(&self, actor: Actor, input: RegisterInput) -> Result<PendingUser, crate::Error> {
        if actor.is_some() {
            return Err(Error::MustNotBeAuthenticated.into());
        }

        // sleep to prevent spam and bruteforce
        let sleep = thread_rng().gen_range(consts::SLEEP_MIN..consts::SLEEP_MAX);
        delay_for(sleep).await;

        // clean and validate data
        let email = input.email.trim().to_lowercase();
        self.validate_email(&email, true)?;
        let username = input.username.trim().to_lowercase();
        self.validate_username(&username)?;

        let mut tx = self.db.begin().await?;

        let find_existing_user_res = self.repo.find_user_by_email(&mut tx, &email).await;
        match find_existing_user_res {
            Ok(_) => Err(Error::EmailAlreadyExists),
            Err(Error::UserNotFound) => Ok(()),
            Err(err) => Err(err),
        }?;

        let namespace_exists = self.check_namespace_exists(&mut tx, &username).await?;
        if namespace_exists {
            return Err(Error::UsernameAlreadyExists.into());
        }

        // create new pending user
        let now = chrono::Utc::now();
        let (code, code_hash) = spawn_blocking(|| {
            let code = rand::alphabet(consts::CODE_ALPHABET, consts::REGISTER_CODE_LENGTH);

            let code_hash = match crypto::hash_password(&code) {
                Ok(res) => res,
                Err(err) => {
                    error!("kernel.register: hashing code: {}", &err);
                    return Err(crate::Error::Internal(err.to_string()));
                }
            };

            Ok((code, code_hash))
        })
        .await??;

        let pending_user = PendingUser {
            id: Ulid::new().into(),
            created_at: now,
            updated_at: now,
            username,
            email,
            failed_attempts: 0,
            code_hash,
        };
        self.repo.create_pending_user(&mut tx, &pending_user).await?;

        tx.commit().await?;

        let job = crate::domain::messages::Message::KernelSendRegisterEmail {
            email: pending_user.email.clone(),
            username: pending_user.username.clone(),
            code,
        };
        let _ = self.queue.push(job, None).await; // TODO: log error?

        Ok(pending_user)
    }
}
