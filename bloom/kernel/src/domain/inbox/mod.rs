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
}

#[derive(Debug, Clone)]
pub struct InitNamespaceInput {
    pub namespace_id: Uuid,
    pub name: String,
}
