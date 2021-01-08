use super::Service;
use kernel::domain::analytics::events::TrackEvent;

impl Service {
    pub async fn process_track_event(&self, _input: TrackEvent) -> Result<(), kernel::Error> {
        unimplemented!();
    }
}
