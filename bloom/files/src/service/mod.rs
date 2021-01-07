use crate::repository::Repository;
use kernel::service::Service as KernelService;
use kernel::{db::DB, drivers};
use std::sync::Arc;
use stdx::uuid::Uuid;

mod complete_file_upload;
mod create_folder;
mod empty_trash;
mod move_files;
mod move_files_to_trash;
mod rename_file;

#[derive(Debug)]
pub struct Service {
    repo: Repository,
    db: DB,
    storage: Arc<dyn drivers::Storage>,
    kernel_service: Arc<KernelService>,
}

impl Service {
    pub fn new(kernel_service: Arc<KernelService>, db: DB, storage: Arc<dyn drivers::Storage>) -> Service {
        let repo = Repository::new();
        Service {
            db,
            repo,
            storage,
            kernel_service,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompleteFileUploadInput {
    pub upload_id: Uuid,
    pub parent_id: Uuid,
    pub name: String,
    pub mime_type: String,
}

#[derive(Debug, Clone)]
pub struct CreateFolderInput {
    pub parent_id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct TrashInput {
    pub namespace: String,
}

#[derive(Debug, Clone)]
pub struct MoveFilesToTrashInput {
    pub files: Vec<Uuid>,
}

#[derive(Debug, Clone)]
pub struct RestoreFilesFromTrashInput {
    pub files: Vec<Uuid>,
}

#[derive(Debug, Clone)]
pub struct EmptyTrashInput {
    pub namespace: String,
}

#[derive(Debug, Clone)]
pub struct MoveFilesInput {
    pub files: Vec<Uuid>,
    pub destination: Uuid,
}

#[derive(Debug, Clone)]
pub struct RenameFileInput {
    pub file_id: Uuid,
    pub name: String,
}
