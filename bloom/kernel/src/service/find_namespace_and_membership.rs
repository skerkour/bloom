use super::Service;
use crate::{
    consts::NamespaceType,
    db::DB,
    entities::{GroupMembership, Namespace},
    errors::kernel::Error,
};
use stdx::uuid::Uuid;

impl Service {
    pub async fn find_namespace_and_membership(
        &self,
        db: &DB,
        user_id: Uuid,
        namespace_id: Uuid,
    ) -> Result<(Namespace, Option<GroupMembership>), crate::Error> {
        let namespace = self.repo.find_namespace_by_id(db, namespace_id).await?;

        let membership = if namespace.r#type == NamespaceType::Group {
            Some(match self
                .repo
                .find_namespace_group_membership(db, namespace_id, user_id)
                .await
            {
                Ok(membership) => Ok(membership),
                Err(Error::GroupMemberNotFound) => Err(Error::PermissionDenied),
                Err(err) => Err(err),
            }?)
        } else {
            None
        };

        Ok((namespace, membership))
    }
}
