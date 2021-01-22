use super::Repository;
use crate::{db::Queryer, entities, errors::kernel::Error};
use stdx::sqlx;
use stdx::{
    chrono::{DateTime, Utc},
    log::error,
};

impl Repository {
    pub async fn find_old_uploads<'c, C: Queryer<'c>>(
        &self,
        db: C,
        older_than: DateTime<Utc>,
    ) -> Result<Vec<entities::Upload>, Error> {
        const QUERY: &str = "SELECT * FROM kernel_uploads WHERE created_at < $1";

        match sqlx::query_as::<_, entities::Upload>(QUERY)
            .bind(older_than)
            .fetch_all(db)
            .await
        {
            Err(err) => {
                error!("kernel.find_old_uploads: finding uploads: {}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }
}
