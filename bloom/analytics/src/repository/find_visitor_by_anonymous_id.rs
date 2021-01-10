use super::Repository;
use crate::{entities::Visitor, Error};
use kernel::db::Queryer;
use stdx::uuid::Uuid;

impl Repository {
    pub async fn find_visitor_by_anonymous_id<'c, C: Queryer<'c>>(
        &self,
        db: C,
        anonymous_id: Uuid,
    ) -> Result<Visitor, Error> {
        todo!();
    }
}
