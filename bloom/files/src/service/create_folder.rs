use std::unimplemented;

use super::{CreateFolderInput, Service};
use crate::entities::File;
use kernel::Actor;

impl Service {
    pub async fn create_folder(&self, actor: Actor, _input: CreateFolderInput) -> Result<File, kernel::Error> {
        let _actor = self.kernel_service.current_user(actor)?;

        // parent, err := service.collaborationRepo.FindFile(ctx, service.db, input.ParentID)
        // if err != nil {
        //     return
        // }

        // if parent.ProjectID == nil {
        //     err = collaboration.ErrFileNotFound
        //     return
        // }

        // // check project membership
        // _, err = service.projectsService.FindProjectByID(ctx, *parent.ProjectID)
        // if err != nil {
        //     return
        // }

        // // valdiate input
        // if parent.TrashedAt != nil {
        //     err = collaboration.ErrFolderIsTrashed
        //     return
        // }

        // err = service.ValidateFileName(input.Name)
        // if err != nil {
        //     return
        // }

        // now := time.Now().UTC()
        // ret = collaboration.File{
        //     ID:                uuid.New(),
        //     CreatedAt:         now,
        //     UpdatedAt:         now,
        //     Name:              input.Name,
        //     Size:              0,
        //     Type:              collaboration.FileTypeFolder,
        //     ExplicitlyTrashed: false,
        //     TrashedAt:         nil,
        //     ProjectID:         parent.ProjectID,
        //     ParentID:          &parent.ID,
        // }
        // err = service.collaborationRepo.CreateFile(ctx, service.db, ret)
        // if err != nil {
        //     return
        // }

        // return
        unimplemented!();
    }
}
