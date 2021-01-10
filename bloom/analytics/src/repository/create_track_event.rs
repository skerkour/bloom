use super::Repository;
use crate::{entities::TrackEvent, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx};

impl Repository {
    pub async fn create_track_event<'c, C: Queryer<'c>>(&self, db: C, event: &TrackEvent) -> Result<(), Error> {
        const QUERY: &str = "INSERT INTO analytics_track_events
        (id, created_at, timestamp, sent_at, received_at, page_name, url, user_agent, referrer, device_type,
        country, os_name, os_version, browser_name, browser_version, path, screen_width, screen_height,
        visitor_id, namespace_id, country_code, event_name, properties)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23)";

        match sqlx::query(QUERY)
            .bind(event.id)
            .bind(event.created_at)
            .bind(event.timestamp)
            .bind(event.sent_at)
            .bind(event.received_at)
            .bind(&event.page_name)
            .bind(&event.url)
            .bind(&event.user_agent)
            .bind(&event.referrer)
            .bind(event.device_type)
            .bind(&event.country)
            .bind(&event.os_name)
            .bind(&event.os_version)
            .bind(&event.browser_name)
            .bind(&event.browser_version)
            .bind(&event.path)
            .bind(event.screen_width)
            .bind(event.screen_height)
            .bind(event.visitor_id)
            .bind(event.namespace_id)
            .bind(&event.country_code)
            .bind(&event.event_name)
            .bind(&event.properties)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("analytics.create_track_event: Inserting event: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
