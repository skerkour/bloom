use super::InitNamespaceInput;
use crate::{consts, entities::ChatboxPreferences, Service};
use kernel::db::Queryer;
use stdx::{chrono::Utc, ulid::Ulid};

impl Service {
    pub async fn init_namespace<'c, C: Queryer<'c>>(
        &self,
        db: C,
        input: InitNamespaceInput,
    ) -> Result<(), kernel::Error> {
        let now = Utc::now();
        let preferences = ChatboxPreferences {
            id: Ulid::new().into(),
            created_at: now,
            updated_at: now,
            color: consts::DEFAULT_CHATBOX_COLOR.to_string(),
            name: input.name,
            avatar_storage_key: None,
            show_branding: true,
            welcome_message: String::new(),
            namespace_id: input.namespace_id,
        };
        self.repo.create_chatbox_preferences(db, &preferences).await?;

        Ok(())
    }
}
