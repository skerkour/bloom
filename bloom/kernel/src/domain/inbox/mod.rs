use std::fmt::Debug;
use stdx::{
    sqlx::{Postgres, Transaction},
    uuid::Uuid,
};

#[async_trait::async_trait]
pub trait Service: Send + Sync + Debug {
    async fn init_namespace<'c>(
        &self,
        tx: &mut Transaction<'c, Postgres>,
        input: InitNamespaceInput,
    ) -> Result<(), crate::Error>;

    async fn clean_user<'c>(&self, tx: &mut Transaction<'c, Postgres>, user_id: Uuid) -> Result<(), crate::Error>;
    async fn update_chatbox_avatar_unauthenticated<'c>(
        &self,
        tx: &mut Transaction<'c, Postgres>,
        input: UpdateChatboxAvatarInput,
    ) -> Result<(), crate::Error>;
}

#[derive(Debug, Clone)]
pub struct InitNamespaceInput {
    pub namespace_id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct UpdateChatboxAvatarInput {
    pub namespace_id: Uuid,
    pub avatar_id: Option<String>,
}
