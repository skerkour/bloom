use super::Service;
use crate::{
    db::Queryer,
    entities::{GroupMembership, Namespace},
};
use stdx::uuid::Uuid;

impl Service {
    pub async fn find_namespace_and_membership<'c, C: Queryer<'c>>(
        &self,
        _db: C,
        _user_id: Uuid,
        _namespace_id: Uuid,
    ) -> Result<(Namespace, Option<GroupMembership>), crate::Error> {
        todo!();
    }
}
