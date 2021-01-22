use super::Repository;
use crate::{entities, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn find_contacts_for_list<'c, C: Queryer<'c>>(
        &self,
        db: C,
        list_id: Uuid,
    ) -> Result<Vec<entities::Contact>, Error> {
        todo!();
    }
}
