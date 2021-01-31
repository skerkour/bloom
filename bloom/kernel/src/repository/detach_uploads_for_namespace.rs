use super::Repository;
use crate::{db, errors::kernel::Error};
use stdx::log::error;
use stdx::{chrono::Utc, sqlx, uuid::Uuid};

impl Repository {
    pub async fn detach_uploads_for_namespace<'c, C: db::Queryer<'c>>(
        &self,
        db: C,
        namespace_id: Uuid,
    ) -> Result<(), Error> {
        let now = Utc::now();
        const QUERY: &str = "UPDATE kernel_uploads SET
		updated_at = $1, namespace_id = NULL
		WHERE namespace_id = $2";

        match sqlx::query(QUERY).bind(now).bind(namespace_id).execute(db).await {
            Err(err) => {
                error!("kernel.detach_uploads_for_namespace: updating uploads: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
