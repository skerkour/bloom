use super::Service;
use kernel::{domain::analytics::events::PageEvent, Actor};

impl Service {
    pub async fn handle_page_event(&self, _actor: Actor, _input: PageEvent) -> Result<(), kernel::Error> {
        unimplemented!();
    }
}
