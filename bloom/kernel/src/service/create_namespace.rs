use super::{CreateNamespaceInput, Service};
use crate::{db::Queryer, entities::Namespace};
use stdx::{chrono::Utc, ulid::Ulid};

impl Service {
    pub async fn create_namespace<'c, C: Queryer<'c>>(
        &self,
        db: C,
        input: CreateNamespaceInput,
    ) -> Result<Namespace, crate::Error> {
        self.validate_namespace(&input.path)?;

        let now = Utc::now();
        let namespace = Namespace {
            id: Ulid::new().into(),
            created_at: now,
            updated_at: now,
            path: input.path,
            r#type: input.namespace_type,
            parent_id: None,
        };
        self.repo.create_namespace(db, &namespace).await?;

        Ok(namespace)
    }
}
