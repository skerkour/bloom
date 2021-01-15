use super::Repository;
use crate::{entities::ChatboxPreferences, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx};

impl Repository {
    pub async fn create_chatbox_preferences<'c, C: Queryer<'c>>(
        &self,
        db: C,
        preferences: &ChatboxPreferences,
    ) -> Result<(), Error> {
        todo!();
    }
}
