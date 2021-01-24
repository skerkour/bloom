use super::{Me, Service};
use crate::Actor;

impl Service {
    pub async fn me(&self, actor: Actor) -> Result<Me, crate::Error> {
        let (actor, session) = self.current_user_and_session(actor)?;

        let groups = self.repo.find_groups_for_user(&self.db, actor.id).await?;

        Ok(Me {
            session,
            user: actor,
            groups,
        })
    }
}
