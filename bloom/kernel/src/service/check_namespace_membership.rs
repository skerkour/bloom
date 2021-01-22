use super::Service;
use crate::{db::Queryer, entities::User, errors::kernel::Error};
use stdx::uuid::Uuid;

impl Service {
    pub async fn check_namespace_membership<'c, C: Queryer<'c>>(
        &self,
        db: C,
        actor: &User,
        namespace_id: Uuid,
    ) -> Result<(), crate::Error> {
        if actor.namespace_id == namespace_id {
            return Ok(());
        }

        match self
            .repo
            .find_namespace_group_membership(db, namespace_id, actor.id)
            .await
        {
            Ok(_) => Ok(()),
            Err(Error::GroupMemberNotFound) => Err(Error::PermissionDenied),
            Err(err) => Err(err),
        }?;

        Ok(())
    }
}
