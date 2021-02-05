use super::CreateEventInput;
use crate::{entities::Event, Service};
use kernel::Actor;

impl Service {
    pub async fn create_event(&self, actor: Actor, input: CreateEventInput) -> Result<Event, kernel::Error> {
        todo!();
    }
}
