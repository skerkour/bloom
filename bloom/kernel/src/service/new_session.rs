use super::NewSession;
use crate::{entities::Session, Service};
use stdx::{
    chrono::Utc,
    crypto,
    rand::{thread_rng, Rng},
    uuid::Uuid,
};

impl Service {
    pub async fn new_session(&self, user_id: Uuid) -> Result<NewSession, crate::Error> {
        let session_id = Uuid::new_v4();
        let now = Utc::now();

        let mut secret = [0u8; crypto::KEY_SIZE_512];
        thread_rng().fill(&mut secret[..]);

        let secret_hash = self.hash_session(session_id, secret.into()).await?;

        let token = self.encode_session_token(session_id, secret.into())?;

        let session = Session {
            id: session_id,
            created_at: now,
            updated_at: now,
            secret_hash,
            user_id,
        };

        Ok(NewSession {
            session,
            token,
        })
    }
}
