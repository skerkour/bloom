use super::Repository;
use crate::{entities::Event, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn find_events<'c, C: Queryer<'c>>(&self, db: C, namespace_id: Uuid) -> Result<Vec<Event>, Error> {
        const QUERY: &str = "SELECT event_name, COUNT (id) AS views, COUNT (DISTINCT visitor_id) AS visitors
		FROM analytics_track_events
		WHERE namespace_id = $1 AND timestamp > (CURRENT_DATE - INTERVAL '30 days')
		GROUP BY event_name";

        match sqlx::query_as::<_, Event>(QUERY).bind(namespace_id).fetch_all(db).await {
            Err(err) => {
                error!("analytics.find_events: finding events: {}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }
}
