use super::Repository;
use crate::{entities::Device, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn find_devices<'c, C: Queryer<'c>>(&self, db: C, namespace_id: Uuid) -> Result<Vec<Device>, Error> {
        const QUERY: &str = "SELECT device_type, COUNT (id) AS views, COUNT (DISTINCT visitor_id) AS visitors
		FROM analytics_page_events
		WHERE namespace_id = $1 AND timestamp > (CURRENT_DATE - INTERVAL '30 days')
		GROUP BY 1";

        match sqlx::query_as::<_, Device>(QUERY)
            .bind(namespace_id)
            .fetch_all(db)
            .await
        {
            Err(err) => {
                error!("analytics.find_devices: finding devices: {}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }
}
