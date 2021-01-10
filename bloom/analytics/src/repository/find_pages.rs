use super::Repository;
use crate::{entities::Page, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn find_pages<'c, C: Queryer<'c>>(&self, db: C, namespace_id: Uuid) -> Result<Vec<Page>, Error> {
        const QUERY: &str = "SELECT COUNT (id) AS views, COUNT (DISTINCT visitor_id) AS visitors, path, '' as url
		FROM analytics_page_events
		WHERE namespace_id = $1 AND timestamp > (CURRENT_DATE - INTERVAL '30 days')
		GROUP BY path";

        match sqlx::query_as::<_, Page>(QUERY).bind(namespace_id).fetch_all(db).await {
            Err(err) => {
                error!("analytics.find_pages: finding pages: {}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }
}
