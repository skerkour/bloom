use super::{RenameFileInput, Service};
use crate::{Error, entities::File};
use kernel::Actor;

impl Service {
    pub async fn rename_file(&self, actor: Actor, input: RenameFileInput) -> Result<File, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        let file = self.repo.find_file_by_id(&self.db, input.file_id).await?;

        if file.namespace_id.is_none() || file.parent_id.is_none() {
            return Err(Error::FileNotFound.into());
        }

        self.kernel_service.check_namespace_membership(&self.db, actor.id, file.namespace_id.unwrap()).await?;


        if file.is_root() {
            return Err(Error::PermissionDenied.into())
        }

        // valdiate input
        self.validate_file_name(&input.name)?;

        return Ok(file)
    }
}
