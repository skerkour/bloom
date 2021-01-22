use super::{RenameFileInput, Service};
use crate::{entities::File, Error};
use kernel::Actor;
use stdx::chrono::Utc;

impl Service {
    pub async fn rename_file(&self, actor: Actor, input: RenameFileInput) -> Result<File, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        let mut file = self.repo.find_file_by_id(&self.db, input.file_id).await?;

        if file.namespace_id.is_none() || file.parent_id.is_none() {
            return Err(Error::FileNotFound.into());
        }

        self.kernel_service
            .check_namespace_membership(&self.db, &actor, file.namespace_id.unwrap())
            .await?;

        if file.is_root() {
            return Err(Error::PermissionDenied.into());
        }

        // valdiate input
        self.validate_file_name(&input.name)?;

        match self
            .repo
            .find_file_by_parent_and_name(&self.db, file.parent_id.unwrap(), &input.name)
            .await
        {
            Ok(_) => return Err(Error::FileAlreadyExists.into()),
            Err(Error::FileNotFound) => {}
            Err(err) => return Err(err.into()),
        };

        let now = Utc::now();
        file.updated_at = now;
        file.name = input.name;
        self.repo.update_file(&self.db, &file).await?;

        return Ok(file);
    }
}
