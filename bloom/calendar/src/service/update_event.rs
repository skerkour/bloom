use super::UpdateEventInput;
use crate::{entities::Event, Service};
use kernel::Actor;

impl Service {
    pub async fn update_event(&self, actor: Actor, input: UpdateEventInput) -> Result<Event, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        let mut event = self.repo.find_event_by_id(&self.db, input.event_id).await?;

        self.kernel_service
            .check_namespace_membership(&self.db, &actor, event.namespace_id)
            .await?;

        // clean and validate input
        if let Some(mut title) = input.title {
            title = title.trim().replace('\n', " ");
            self.validate_event_title(&title)?;
            event.title = title;
        }

        if let Some(mut description) = input.description {
            description = description.trim().to_string();
            self.validate_event_description(&description)?;
            event.description = description;
        }

        if let Some(mut location) = input.location {
            location = location.trim().replace('\n', " ");
            self.validate_event_location(&location)?;
            event.location = location;
        }

        if let Some(start_at) = input.start_at {
            event.start_at = start_at;
        }
        if let Some(end_at) = input.end_at {
            event.end_at = end_at;
        }
        self.validate_event_dates(event.start_at, event.end_at)?;

        self.repo.update_event(&self.db, &event).await?;

        Ok(event)
    }
}
