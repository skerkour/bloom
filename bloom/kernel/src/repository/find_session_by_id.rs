use super::Repository;
use crate::{db::Queryer, entities, errors::kernel::Error};
use stdx::uuid::Uuid;

impl Repository {
    pub async fn find_session_by_id<'c, C: Queryer<'c>>(
        &self,
        db: C,
        session_id: Uuid,
    ) -> Result<entities::Session, Error> {
        const QUERY: &str = "SELECT * FROM kernel_session WHERE id = $1";

        match sqlx::query_as::<_, entities::Session>(QUERY)
            .bind(session_id)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                println!("kernel.find_session_by_id: finding session: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(Error::SessionNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
