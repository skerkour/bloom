use super::UpdateEventInput;
use crate::{entities::Event, Service};
use kernel::Actor;

impl Service {
    pub async fn update_event(&self, actor: Actor, input: UpdateEventInput) -> Result<Event, kernel::Error> {
        todo!();
    }
}
