use super::{EmptyTrashInput, Service};
use kernel::Actor;

impl Service {
    pub async fn empty_trash(&self, actor: Actor, input: EmptyTrashInput) -> Result<(), kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        let (mut namespace, _) = self
            .kernel_service
            .find_namespace_and_membership(&self.db, actor.id, input.namespace_id)
            .await?;

        let trashed_files = self.repo.find_all_trashed_files(&self.db, namespace.id).await?;

        for file in trashed_files {
            namespace.used_storage -= file.size;
        }

        let mut tx = self.db.begin().await?;

        self.repo
            .detach_all_trashed_files_from_namespace(&mut tx, namespace.id)
            .await?;

        self.kernel_service.update_namespace(&mut tx, &namespace).await?;

        tx.commit().await?;

        Ok(())
    }
}
