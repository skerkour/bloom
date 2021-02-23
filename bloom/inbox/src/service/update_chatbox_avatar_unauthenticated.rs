use crate::Service;
use kernel::domain::inbox::UpdateChatboxAvatarInput;
use stdx::sqlx::{Postgres, Transaction};

impl Service {
    pub async fn update_chatbox_avatar_unauthenticated<'c>(
        &self,
        tx: &mut Transaction<'c, Postgres>,
        input: UpdateChatboxAvatarInput,
    ) -> Result<(), kernel::Error> {
        let mut preferences = self
            .repo
            .find_chatbox_preferences_for_namespace(&self.db, input.namespace_id)
            .await?;

        preferences.avatar_id = input.avatar_id;
        self.repo.update_chatbox_preferences(tx, &preferences).await?;

        Ok(())
    }
}
