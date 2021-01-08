use super::{QuitGroupInput, Service};
use crate::{errors::kernel::Error, Actor};

impl Service {
    pub async fn quit_group(&self, actor: Actor, input: QuitGroupInput) -> Result<(), crate::Error> {
        let actor = self.current_user(actor)?;

        // check that user is member
        let membership = self
            .repo
            .find_group_membership(&self.db, input.group_id, actor.id)
            .await?;

        let mut tx = self.db.begin().await?;

        self.repo.delete_group_membership(&mut tx, &membership).await?;

        let admins_count = self.repo.get_group_admins_count(&mut tx, input.group_id).await?;

        if admins_count == 0 {
            return Err(Error::AtLeatOneAdminMustRemainInGroup.into());
        }

        tx.commit().await?;

        Ok(())
    }
}
