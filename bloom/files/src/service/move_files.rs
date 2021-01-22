use crate::Error;

use super::{MoveFilesInput, Service};
use kernel::Actor;
use stdx::chrono::Utc;

impl Service {
    // TODO: check paths sheningans
    pub async fn move_files(&self, actor: Actor, input: MoveFilesInput) -> Result<(), kernel::Error> {
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

        for file in &files {
            if file.namespace_id.is_none() || file.namespace_id.unwrap() != namespace_id {
                return Err(Error::PermissionDenied.into());
            }

            if file.is_root() {
                return Err(Error::PermissionDenied.into());
            }

            if file.id == input.destination {
                return Err(Error::PermissionDenied.into());
            }

            if file.trashed_at.is_some() {
                return Err(Error::PermissionDenied.into());
            }

            match self
                .repo
                .find_file_by_parent_and_name(&self.db, input.destination, &file.name)
                .await
            {
                Ok(_) => return Err(Error::FileAlreadyExists.into()),
                Err(Error::FileNotFound) => {}
                Err(err) => return Err(err.into()),
            };
        }

        let destination = self.repo.find_file_by_id(&self.db, input.destination).await?;

        if destination.namespace_id.is_none() || destination.namespace_id.unwrap() != namespace_id {
            return Err(Error::PermissionDenied.into());
        }

        if destination.trashed_at.is_some() {
            return Err(Error::PermissionDenied.into());
        }

        self.kernel_service
            .check_namespace_membership(&self.db, &actor, namespace_id)
            .await?;

        let now = Utc::now();
        let mut tx = self.db.begin().await?;

        for mut file in files {
            file.updated_at = now;
            file.parent_id = Some(destination.id);
            self.repo.update_file(&mut tx, &file).await?;
        }

        tx.commit().await?;

        Ok(())
    }
}
