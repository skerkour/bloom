use super::Service;
use kernel::domain::analytics::events::PageEvent;

impl Service {
    pub async fn process_page_event(&self, _input: PageEvent) -> Result<(), kernel::Error> {
        unimplemented!();
    }
}
