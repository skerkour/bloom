use crate::{
    entities::{Session, User},
    Service,
};

impl Service {
    pub async fn decode_and_validate_session_token(&self, token: String) -> Result<(User, Session), crate::Error> {
        let decoded_token = self.decode_session_token(token)?;

        let session = self.repo.find_session_by_id(&self.db, decoded_token.session_id).await?;

        self.verify_session_secret(&session, decoded_token.secret).await?;

        let user = self.repo.find_user_by_id(&self.db, session.user_id).await?;

        Ok((user, session))
    }
}
