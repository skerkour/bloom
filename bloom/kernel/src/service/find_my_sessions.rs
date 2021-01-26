use super::{Service, Session};
use crate::Actor;

impl Service {
    pub async fn find_my_sessions(&self, actor: Actor) -> Result<Vec<Session>, crate::Error> {
        let actor = self.current_user(actor)?;

        let sessions = self.repo.find_sessions_by_user_id(&self.db, actor.id).await?;

        Ok(sessions)
    }
}
