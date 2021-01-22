use super::Service;
use crate::entities::File;
use kernel::Actor;
use stdx::uuid::Uuid;

impl Service {
    pub async fn find_trash(&self, actor: Actor, namespace_id: Uuid) -> Result<Vec<File>, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        self.kernel_service
            .check_namespace_membership(&self.db, &actor, namespace_id)
            .await?;

        let trashed_files = self.repo.find_all_trashed_files(&self.db, namespace_id).await?;

        Ok(trashed_files)
    }
}
