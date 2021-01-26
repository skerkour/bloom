use super::Service;
use crate::Error;
use kernel::Actor;
use stdx::uuid::Uuid;

impl Service {
    pub async fn get_file_download_url(&self, actor: Actor, file_id: Uuid) -> Result<String, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        let file = self.repo.find_file_by_id(&self.db, file_id).await?;

        if file.namespace_id.is_none() {
            return Err(Error::FileNotFound.into());
        }

        if file.is_folder() {
            return Err(Error::PermissionDenied.into());
        }

        self.kernel_service
            .check_namespace_membership(&self.db, &actor, file.namespace_id.unwrap())
            .await?;

        let storage_key = file.storage_key();
        let download_url = self
            .storage
            .get_object_download_url(&storage_key, &file.name, &file.r#type)
            .await;

        Ok(download_url)
    }
}
