use super::Repository;
use crate::{entities::Visitor, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn find_visitor_by_anonymous_id<'c, C: Queryer<'c>>(
        &self,
        db: C,
        anonymous_id: Uuid,
    ) -> Result<Visitor, Error> {
        const QUERY: &str = "SELECT * FROM analytics_visitors WHERE anonymous_id = $1";

        match sqlx::query_as::<_, Visitor>(QUERY)
            .bind(anonymous_id)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!("analytics.find_visitor_by_anonymous_id: finding visitor: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(Error::VisitorNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
