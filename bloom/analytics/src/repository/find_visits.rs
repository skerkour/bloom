use super::Repository;
use crate::{entities::Visit, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn find_visits<'c, C: Queryer<'c>>(&self, db: C, namespace_id: Uuid) -> Result<Vec<Visit>, Error> {
        const QUERY: &str = "WITH events AS (
			SELECT * FROM analytics_page_events
			WHERE namespace_id = $1 AND timestamp > (CURRENT_DATE - INTERVAL '31 days')
		)
		SELECT date.date, COUNT(events.id) AS views, COUNT (DISTINCT events.visitor_id) AS visitors
		FROM (
			SELECT to_char(date_trunc('day', (current_date - offs)), 'YYYY-MM-DD')
			AS date
			FROM generate_series(0, 30, 1)
			AS offs
		) date LEFT OUTER JOIN
		events
		ON date.date = to_char(date_trunc('day', events.timestamp), 'YYYY-MM-DD')
        GROUP BY date.date;
        ";

        match sqlx::query_as::<_, Visit>(QUERY).bind(namespace_id).fetch_all(db).await {
            Err(err) => {
                error!("analytics.find_visits: finding visits: {}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }
}
