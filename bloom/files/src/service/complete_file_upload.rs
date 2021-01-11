use super::{CompleteFileUploadInput, Service};
use crate::entities::File;
use kernel::Actor;

impl Service {
    pub async fn complete_file_upload(
        &self,
        _actor: Actor,
        _input: CompleteFileUploadInput,
    ) -> Result<File, kernel::Error> {
        todo!();
        // _, err = service.kernelService.CurrentUser(ctx)
        // if err != nil {
        //     return
        // }
        // logger := log.FromCtx(ctx)

        // parent, err := service.collaborationRepo.FindFile(ctx, service.db, input.ParentID)
        // if err != nil {
        //     return
        // }

        // if parent.ProjectID == nil {
        //     err = collaboration.ErrFileNotFound
        //     return
        // }

        // // check project membership
        // project, err := service.projectsService.FindProjectByID(ctx, *parent.ProjectID)
        // if err != nil {
        //     return
        // }

        // // clean and valdiate input
        // if parent.TrashedAt != nil {
        //     err = collaboration.ErrFolderIsTrashed
        //     return
        // }

        // err = service.ValidateFileName(input.Name)
        // if err != nil {
        //     return
        // }

        // if input.Type == "" {
        //     input.Type = collaboration.FileTypeDefault
        // }

        // err = service.ValidateFileUploadType(input.Type)
        // if err != nil {
        //     return
        // }

        // _, err = service.collaborationRepo.FindFileByNameAndParent(ctx, service.db, input.ParentID, input.Name, false)
        // if err == nil {
        //     err = collaboration.ErrFileAlreadyExists
        //     return
        // }
        // if err != nil {
        //     if !errors.Is(err, collaboration.ErrFileNotFound) {
        //         return
        //     }
        //     err = nil
        // }

        // namespace, err := service.kernelService.FindNamespaceByID(ctx, service.db, project.NamespaceID)
        // if err != nil {
        //     return
        // }

        // size, err := service.storage.GetFileSize(ctx, input.TmpKey)
        // if err != nil {
        //     errMessage := "collaboration.CompleteFileUpload: retrieving file size"
        //     logger.Error(errMessage, log.Err("error", err))
        //     err = errors.Internal(errMessage, err)
        //     return
        // }

        // if size != input.Size {
        //     err = kernel.ErrPermissionDenied
        //     return
        // }

        // newFileID := uuid.New()
        // now := time.Now().UTC()

        // namespace.UsedStorage += size

        // // check if storage limits are not reached
        // if (namespace.Plan == kernel.PlanFree && namespace.UsedStorage > kernel.PlanStorageFree) ||
        //     (namespace.Plan == kernel.PlanStarter && namespace.UsedStorage > kernel.PlanStorageStarter) ||
        //     (namespace.Plan == kernel.PlanPro && namespace.UsedStorage > kernel.PlanStoragePro) ||
        //     (namespace.Plan == kernel.PlanUltra && namespace.UsedStorage > kernel.PlanStorageUltra) ||
        //     namespace.UsedStorage < 0 {
        //     err = kernel.ErrPlanStorageLimitReached
        // }
        // if err != nil {
        //     return
        // }

        // ret = collaboration.File{
        //     ID:                newFileID,
        //     CreatedAt:         now,
        //     UpdatedAt:         now,
        //     Name:              input.Name,
        //     Size:              input.Size,
        //     Type:              input.Type,
        //     ExplicitlyTrashed: false,
        //     TrashedAt:         nil,
        //     ProjectID:         parent.ProjectID,
        //     ParentID:          &parent.ID,
        // }

        // err = service.storage.CopyObject(ctx, input.TmpKey, ret.StorageKey())
        // if err != nil {
        //     errMessage := "collaboration.CompleteFileUpload: copying object from tmp"
        //     logger.Error(errMessage, log.Err("error", err))
        //     err = errors.Internal(errMessage, err)
        //     return
        // }

        // err = service.db.Transaction(ctx, func(tx db.Queryer) (err error) {
        //     err = service.collaborationRepo.CreateFile(ctx, tx, ret)
        //     if err != nil {
        //         return
        //     }

        //     err = service.kernelService.UpdateNamespace(ctx, tx, namespace)
        //     if err != nil {
        //         return
        //     }

        //     return
        // })
        // if err != nil {
        //     return
        // }
    }
}
