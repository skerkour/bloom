use super::Service;
use crate::{
    db::DB,
    entities::{Group, GroupMembership},
    errors::kernel::Error,
};
use stdx::uuid::Uuid;

impl Service {
    pub async fn find_group_and_membership(
        &self,
        db: &DB,
        user_id: Uuid,
        group_id: Uuid,
    ) -> Result<(Group, GroupMembership), crate::Error> {
        let membership = match self.repo.find_group_membership(db, group_id, user_id).await {
            Ok(membership) => Ok(membership),
            Err(Error::GroupMemberNotFound) => Err(Error::PermissionDenied),
            Err(err) => Err(err),
        }?;

        let group = self.repo.find_group_by_id(db, group_id).await?;

        Ok((group, membership))
    }
}
