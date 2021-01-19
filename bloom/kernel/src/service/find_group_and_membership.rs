use super::Service;
use crate::{
    db::Queryer,
    entities::{Group, GroupMembership},
};
use stdx::uuid::Uuid;

impl Service {
    pub async fn find_group_and_membership<'c, C: Queryer<'c>>(
        &self,
        _db: C,
        _user_id: Uuid,
        _group_id: Uuid,
    ) -> Result<(Group, GroupMembership), crate::Error> {
        todo!();
    }
}
