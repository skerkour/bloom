use crate::{entities::User, errors::kernel::Error, Service};
use stdx::{
    base64, crypto,
    uuid::{self, Uuid},
};

impl Service {
    pub async fn decode_and_validate_session_token(&self, token: String) -> Result<User, crate::Error> {
        let token = base64::decode(token)?;

        if token.len() != crypto::KEY_SIZE_512 + uuid::SIZE {
            return Err(Error::InvalidSession.into());
        }

        let session_id_bytes = &token[..uuid::SIZE];
        let secret: Vec<u8> = token[uuid::SIZE..].into();
        let session_id = Uuid::from_slice(session_id_bytes).map_err(|_| Error::InvalidSession)?;

        let session = self.repo.find_session_by_id(&self.db, session_id).await?;

        self.verify_session_secret(&session, secret).await?;

        let user = self.repo.find_user_by_id(&self.db, session.user_id).await?;

        Ok(user)
    }
}
