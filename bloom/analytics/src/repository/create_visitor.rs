use super::Repository;
use crate::{entities::Visitor, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx};

impl Repository {
    pub async fn create_visitor<'c, C: Queryer<'c>>(&self, db: C, visitor: &Visitor) -> Result<(), Error> {
        const QUERY: &str = "INSERT INTO analytics_visitors
        (id, created_at, updated_at, anonymous_id, namespace_id)
        VALUES ($1, $2, $3, $4, $5)";

        match sqlx::query(QUERY)
            .bind(visitor.id)
            .bind(visitor.created_at)
            .bind(visitor.updated_at)
            .bind(visitor.anonymous_id)
            .bind(visitor.namespace_id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("analytics.create_visitor: Inserting visitor: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
