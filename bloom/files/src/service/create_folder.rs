use super::{CreateFolderInput, Service};
use crate::{consts, entities::File, Error};
use kernel::Actor;
use stdx::{chrono::Utc, ulid::Ulid};

impl Service {
    pub async fn create_folder(&self, actor: Actor, input: CreateFolderInput) -> Result<File, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        let parent = self.repo.find_file_by_id(&self.db, input.parent_id).await?;

        if parent.namespace_id.is_none() {
            return Err(Error::FileNotFound.into());
        }

        self.kernel_service
            .check_namespace_membership(&self.db, &actor, parent.namespace_id.unwrap())
            .await?;

        // valdiate input
        if parent.trashed_at.is_some() {
            return Err(Error::FolderIsInTrash.into());
        }

        match self
            .repo
            .find_file_by_parent_and_name(&self.db, parent.id, &input.name)
            .await
        {
            Ok(_) => return Err(Error::FileAlreadyExists.into()),
            Err(Error::FileNotFound) => {}
            Err(err) => return Err(err.into()),
        };

        self.validate_file_name(&input.name)?;

        let now = Utc::now();
        let file = File {
            id: Ulid::new().into(),
            created_at: now,
            updated_at: now,

            name: input.name,
            size: 0,
            r#type: consts::FILE_TYPE_FOLDER.to_string(),
            explicitly_trashed: false,
            trashed_at: None,

            namespace_id: parent.namespace_id,
            parent_id: Some(parent.id),
        };
        self.repo.create_file(&self.db, &file).await?;

        Ok(file)
    }
}
