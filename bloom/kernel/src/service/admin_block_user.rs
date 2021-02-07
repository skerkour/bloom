use super::Service;
use crate::{entities::User, errors::kernel::Error, Actor};
use stdx::{chrono::Utc, uuid::Uuid};

impl Service {
    pub async fn admin_block_user(&self, actor: Actor, user_id: Uuid) -> Result<User, crate::Error> {
        let actor = self.current_user(actor)?;

        if !actor.is_admin {
            return Err(Error::PermissionDenied.into());
        }

        let mut user = self.repo.find_user_by_id(&self.db, user_id).await?;

        if user.is_admin {
            return Err(Error::CantBlockAdmin.into());
        }

        let now = Utc::now();
        user.updated_at = now;
        user.blocked_at = Some(now);

        let sessions = self.repo.find_sessions_by_user_id(&self.db, user.id).await?;

        let mut tx = self.db.begin().await?;

        self.repo.update_user(&mut tx, &user).await?;

        // TODO: batch
        for session in sessions {
            self.repo.delete_session(&mut tx, session.id).await?;
        }

        tx.commit().await?;

        Ok(user)
    }
}
