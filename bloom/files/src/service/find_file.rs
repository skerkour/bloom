use super::{FileWithPathAndChildren, FindFileInput, Service};
use crate::Error;
use kernel::Actor;

impl Service {
    pub async fn find_file(
        &self,
        actor: Actor,
        input: FindFileInput,
    ) -> Result<FileWithPathAndChildren, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        self.kernel_service
            .check_namespace_membership(&self.db, &actor, input.namespace_id)
            .await?;

        let file = if let Some(file_id) = input.file_id {
            self.repo.find_file_by_id(&self.db, file_id).await?
        } else {
            self.repo
                .find_root_file_for_namespace(&self.db, input.namespace_id)
                .await?
        };

        if file.namespace_id.is_none() {
            return Err(Error::FileNotFound.into());
        }

        if file.namespace_id.unwrap() != input.namespace_id {
            return Err(Error::FileNotFound.into());
        }

        let children = if file.is_folder() {
            self.repo.find_children(&self.db, file.id).await?
        } else {
            Vec::new()
        };

        let path = self.repo.get_file_path(&self.db, file.id).await?;

        Ok(FileWithPathAndChildren {
            file,
            children,
            path,
        })
    }
}
