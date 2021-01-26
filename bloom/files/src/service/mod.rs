use crate::{
    entities::{File, FilePath},
    repository::Repository,
};
use kernel::{db::DB, drivers};
use std::sync::Arc;
use stdx::{
    sqlx::{Postgres, Transaction},
    uuid::Uuid,
};

mod clean_namespace;
mod complete_file_upload;
mod create_folder;
mod empty_trash;
mod find_file;
mod find_trash;
mod get_file_download_url;
mod init_namespace;
mod move_files;
mod move_files_to_trash;
mod rename_file;
mod restore_files_from_trash;
mod validators;

#[derive(Debug)]
pub struct Service {
    repo: Repository,
    db: DB,
    storage: Arc<dyn drivers::Storage>,
    kernel_service: Arc<kernel::Service>,
}

impl Service {
    pub fn new(kernel_service: Arc<kernel::Service>, db: DB, storage: Arc<dyn drivers::Storage>) -> Service {
        let repo = Repository::new();
        Service {
            db,
            repo,
            storage,
            kernel_service,
        }
    }
}

#[async_trait::async_trait]
impl kernel::domain::files::Service for Service {
    async fn init_namespace<'c>(
        &self,
        tx: &mut Transaction<'c, Postgres>,
        namespace_id: Uuid,
    ) -> Result<(), kernel::Error> {
        self.init_namespace(tx, namespace_id).await
    }

    async fn clean_namespace<'c>(
        &self,
        tx: &mut Transaction<'c, Postgres>,
        namespace_id: Uuid,
    ) -> Result<(), kernel::Error> {
        self.clean_namespace(tx, namespace_id).await
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
    pub namespace_id: Uuid,
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

#[derive(Debug, Clone)]
pub struct FindFileInput {
    pub namespace_id: Uuid,
    pub file_id: Option<Uuid>,
}

#[derive(Debug, Clone)]
pub struct FileWithPathAndChildren {
    pub file: File,
    pub children: Vec<File>,
    pub path: Vec<FilePath>,
}
