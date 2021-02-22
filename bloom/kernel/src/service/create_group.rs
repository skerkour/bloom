use super::{CreateGroupInput, CreateNamespaceInput, Service};
use crate::{
    consts::{GroupRole, NamespaceType},
    entities::{Group, GroupMembership},
    errors::kernel::Error,
    Actor,
};
use stdx::{chrono::Utc, ulid::Ulid};

impl Service {
    pub async fn create_group(&self, actor: Actor, input: CreateGroupInput) -> Result<Group, crate::Error> {
        let actor = self.current_user(actor)?;

        // clean and validate input
        let path = input.path.trim().to_lowercase();
        let name = input.name.trim().to_string();
        let description = input.description.trim().to_string();
        let now = Utc::now();

        self.validate_namespace(&path)?;
        self.validate_group_name(&name)?;
        self.validate_group_description(&description)?;

        // create group and namespace
        let mut tx = self.db.begin().await?;

        let namespace_exists = self.check_namespace_exists(&mut tx, &path).await?;
        if namespace_exists {
            return Err(Error::NamespaceAlreadyExists.into());
        }

        let create_namespace_input = CreateNamespaceInput {
            path: path.clone(),
            namespace_type: NamespaceType::Group,
            name: name.clone(),
        };
        let namespace = self.create_namespace(&mut tx, create_namespace_input).await?;

        let group = Group {
            id: Ulid::new().into(),
            created_at: now,
            updated_at: now,
            name,
            description,
            avatar_id: None,
            namespace_id: namespace.id,
            path,
        };
        self.repo.create_group(&mut tx, &group).await?;

        let membership = GroupMembership {
            joined_at: now,
            role: GroupRole::Administrator,
            user_id: actor.id,
            group_id: group.id,
        };
        self.repo.create_group_membership(&mut tx, &membership).await?;

        tx.commit().await?;

        Ok(group)
    }
}
