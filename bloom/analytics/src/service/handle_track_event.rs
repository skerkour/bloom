use super::Service;
use kernel::{domain::analytics::events::TrackEvent, Actor};

impl Service {
    pub async fn handle_track_event(&self, _actor: Actor, _input: TrackEvent) -> Result<(), kernel::Error> {
        unimplemented!();
    }
}
