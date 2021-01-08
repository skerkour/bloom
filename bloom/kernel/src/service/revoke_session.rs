use super::{RevokeSessionInput, Service};
use crate::{errors::kernel::Error, Actor};

impl Service {
    pub async fn revoke_session(&self, actor: Actor, input: RevokeSessionInput) -> Result<(), crate::Error> {
        let actor = self.current_user(actor)?;

        let session = self.repo.find_session_by_id(&self.db, input.session_id).await?;

        if session.user_id != actor.id {
            return Err(Error::PermissionDenied.into());
        }

        self.repo.delete_session(&self.db, session.id).await?;

        Ok(())
    }
}
