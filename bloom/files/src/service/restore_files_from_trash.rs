use super::{RestoreFilesFromTrashInput, Service};
use crate::{entities::File, Error};
use kernel::Actor;
use std::collections::HashMap;
use stdx::{chrono::Utc, uuid::Uuid};

impl Service {
    // TODO: improve performance: bulk update
    pub async fn restore_files_from_trash(
        &self,
        actor: Actor,
        input: RestoreFilesFromTrashInput,
    ) -> Result<(), kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        if input.files.is_empty() {
            return Ok(());
        }

        let files = self.repo.find_files_by_ids(&self.db, &input.files).await?;

        if files.len() != input.files.len() {
            return Err(Error::FileNotFound.into());
        }

        let namespace_id = files[0].namespace_id;
        if namespace_id.is_none() {
            return Err(Error::FileNotFound.into());
        }
        let namespace_id = namespace_id.unwrap();

        self.kernel_service
            .check_namespace_membership(&self.db, &actor, namespace_id)
            .await?;

        for file in &files {
            if file.namespace_id.is_none() || file.namespace_id.unwrap() != namespace_id {
                return Err(Error::PermissionDenied.into());
            }

            if file.parent_id.is_none() {
                return Err(Error::PermissionDenied.into());
            }

            if file.is_root() {
                return Err(Error::PermissionDenied.into());
            }

            if file.trashed_at.is_none() {
                return Err(Error::PermissionDenied.into());
            }

            match self
                .repo
                .find_file_by_parent_and_name(&self.db, file.parent_id.unwrap(), &file.name)
                .await
            {
                Ok(_) => return Err(Error::FileAlreadyExists.into()),
                Err(Error::FileNotFound) => {}
                Err(err) => return Err(err.into()),
            };
        }

        let mut all_children = HashMap::<Uuid, Vec<File>>::with_capacity(files.len());

        for file in &files {
            let children = self.repo.find_children_recursively(&self.db, file.id).await?;
            all_children.insert(file.id, children);
        }

        let now = Utc::now();
        let mut tx = self.db.begin().await?;

        for mut file in files {
            file.updated_at = now;
            file.trashed_at = None;
            file.explicitly_trashed = false;
            self.repo.update_file(&mut tx, &file).await?;
        }

        for (_, children) in all_children {
            for mut child in children {
                child.updated_at = now;
                child.trashed_at = None;
                child.explicitly_trashed = false;
                self.repo.update_file(&mut tx, &child).await?;
            }
        }

        tx.commit().await?;

        Ok(())
    }
}
