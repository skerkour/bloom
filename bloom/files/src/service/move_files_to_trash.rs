use crate::Error;

use super::{MoveFilesToTrashInput, Service};
use kernel::Actor;

impl Service {
    pub async fn move_files_to_trash(&self, actor: Actor, input: MoveFilesToTrashInput) -> Result<(), kernel::Error> {
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

        let root_file = self.repo.find_root_file_for_namespace(&self.db, namespace_id).await?;

        for file in &files {
            if file.namespace_id.is_none() || file.namespace_id.unwrap() != namespace_id {
                return Err(Error::PermissionDenied.into());
            }

            if file.id == root_file.id {
                return Err(Error::PermissionDenied.into());
            }

            if file.trashed_at.is_some() {
                return Err(Error::PermissionDenied.into());
            }
        }

        self.kernel_service
            .check_namespace_membership(&self.db, actor.id, namespace_id)
            .await?;

        // allChildren := make([][]collaboration.File, len(files))
        // for i, file := range files {
        //     var fileChildren []collaboration.File

        //     fileChildren, err = service.collaborationRepo.FindChildrenRecursively(ctx, service.db, file.ID)
        //     if err != nil {
        //         return
        //     }
        //     allChildren[i] = fileChildren
        // }

        // err = service.db.Transaction(ctx, func(tx db.Queryer) (err error) {
        //     now := time.Now().UTC()
        //     for _, file := range files {
        //         file.UpdatedAt = now
        //         file.TrashedAt = &now
        //         file.ExplicitlyTrashed = true
        //         err = service.collaborationRepo.UpdateFile(ctx, tx, file)
        //         if err != nil {
        //             return
        //         }
        //     }

        //     for _, children := range allChildren {
        //         for _, file := range children {
        //             file.UpdatedAt = now
        //             file.TrashedAt = &now
        //             err = service.collaborationRepo.UpdateFile(ctx, tx, file)
        //             if err != nil {
        //                 return
        //             }
        //         }
        //     }

        //     return
        // })
        // if err != nil {
        //     return
        // }

        Ok(())
    }
}
