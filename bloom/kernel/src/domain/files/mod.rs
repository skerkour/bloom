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
        namespace_id: Uuid,
    ) -> Result<(), crate::Error>;
    async fn clean_namespace<'c>(
        &self,
        tx: &mut Transaction<'c, Postgres>,
        namespace_id: Uuid,
    ) -> Result<(), crate::Error>;
}
