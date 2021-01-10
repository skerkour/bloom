use super::Repository;
use crate::{entities::Referrer, Error};
use kernel::db::Queryer;
use stdx::uuid::Uuid;

impl Repository {
    pub async fn find_referrers<'c, C: Queryer<'c>>(&self, db: C, namespace_id: Uuid) -> Result<Vec<Referrer>, Error> {
        todo!();
    }
}
