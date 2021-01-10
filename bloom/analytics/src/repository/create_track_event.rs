use super::Repository;
use crate::{entities::TrackEvent, Error};
use kernel::db::Queryer;

impl Repository {
    pub async fn create_track_event<'c, C: Queryer<'c>>(&self, db: C, event: &TrackEvent) -> Result<(), Error> {
        todo!();
    }
}
