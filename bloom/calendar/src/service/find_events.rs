use super::FindEventsInput;
use crate::{entities::Event, Service};
use kernel::Actor;

impl Service {
    pub async fn find_events(&self, _actor: Actor, _input: FindEventsInput) -> Result<Vec<Event>, kernel::Error> {
        todo!();
    }
}
