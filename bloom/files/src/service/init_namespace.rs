use crate::{consts, entities::File, Service};
use kernel::db::Queryer;
use stdx::{chrono::Utc, ulid::Ulid, uuid::Uuid};

impl Service {
    pub async fn init_namespace<'c, C: Queryer<'c>>(&self, db: C, namespace_id: Uuid) -> Result<(), kernel::Error> {
        let now = Utc::now();
        let root_file = File {
            id: Ulid::new().into(),
            created_at: now,
            updated_at: now,
            name: consts::ROOT_FILE_NAME.to_string(),
            size: 0,
            r#type: consts::FILE_TYPE_FOLDER.to_string(),
            explicitly_trashed: false,
            trashed_at: None,
            namespace_id: Some(namespace_id),
            parent_id: None,
        };
        self.repo.create_file(db, &root_file).await?;
        Ok(())
    }
}
