use super::Service;
use crate::db::Queryer;
use stdx::uuid::Uuid;

impl Service {
    pub async fn check_namespace_membership<'c, C: Queryer<'c>>(
        &self,
        _db: C,
        _user_id: Uuid,
        _namespace_id: Uuid,
    ) -> Result<(), crate::Error> {
        todo!();
    }
}
