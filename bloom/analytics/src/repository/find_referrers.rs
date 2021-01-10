use super::Repository;
use crate::{entities::Referrer, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn find_referrers<'c, C: Queryer<'c>>(&self, db: C, namespace_id: Uuid) -> Result<Vec<Referrer>, Error> {
        const QUERY: &str = "SELECT referrer, COUNT (id) AS views, COUNT (DISTINCT visitor_id) AS visitors
		FROM analytics_page_events
		WHERE namespace_id = $1 AND timestamp > (CURRENT_DATE - INTERVAL '30 days')
		GROUP BY referrer";

        match sqlx::query_as::<_, Referrer>(QUERY)
            .bind(namespace_id)
            .fetch_all(db)
            .await
        {
            Err(err) => {
                error!("analytics.find_referrers: finding referrers: {}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }
}
