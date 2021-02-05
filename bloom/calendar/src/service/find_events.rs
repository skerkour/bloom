use super::FindEventsInput;
use crate::{entities::Event, Service};
use kernel::Actor;
use stdx::chrono::{Datelike, Duration, Utc};

impl Service {
    pub async fn find_events(&self, actor: Actor, input: FindEventsInput) -> Result<Vec<Event>, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        self.kernel_service
            .check_namespace_membership(&self.db, &actor, input.namespace_id)
            .await?;

        // clean and validate input
        let now = Utc::now();

        let start_at = input
            .start_at
            .unwrap_or_else(|| now.with_day(1).expect("calendar.find_events: start_at.with_day"));
        let end_at = input.end_at.unwrap_or_else(|| start_at + Duration::days(31));
        self.validate_event_dates(start_at, end_at)?;

        let events = self
            .repo
            .find_events(&self.db, input.namespace_id, start_at, end_at)
            .await?;

        Ok(events)
    }
}
