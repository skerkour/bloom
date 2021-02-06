use super::CreateEventInput;
use crate::{entities::Event, Service};
use kernel::Actor;
use stdx::{chrono::Utc, ulid::Ulid};

impl Service {
    pub async fn create_event(&self, actor: Actor, input: CreateEventInput) -> Result<Event, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        self.kernel_service
            .check_namespace_membership(&self.db, &actor, input.namespace_id)
            .await?;

        // clean and validate input
        let title = input.title.trim().replace('\n', " ");
        self.validate_event_title(&title)?;

        let description = input.description.trim().to_string();
        self.validate_event_description(&description)?;

        let location = input.location.trim().replace('\n', " ");
        self.validate_event_location(&location)?;

        let start_at = input.start_at;
        let end_at = input.end_at;
        self.validate_event_dates(start_at, end_at)?;

        let now = Utc::now();
        let event = Event {
            id: Ulid::new().into(),
            created_at: now,
            updated_at: now,
            title,
            description,
            location,
            start_at,
            end_at,
            namespace_id: input.namespace_id,
        };
        self.repo.create_event(&self.db, &event).await?;

        Ok(event)
    }
}
