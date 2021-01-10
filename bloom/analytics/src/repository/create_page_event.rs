use crate::{entities::PageEvent, Error};

use super::Repository;
use kernel::db::Queryer;

impl Repository {
    pub async fn create_page_event<'c, C: Queryer<'c>>(&self, db: C, event: &PageEvent) -> Result<(), Error> {
        todo!();
    }
}
