use crate::repository::Repository;
use kernel::service::Service as KernelService;
use kernel::{db::DB, drivers};
use sqlx::types::Uuid;
use std::sync::Arc;

mod complete_file_upload;
mod create_folder;

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
