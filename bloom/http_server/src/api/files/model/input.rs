use crate::api::scalars::Id;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub namespace_id: Id,
    pub file_id: Option<Id>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trash {
    pub namespace_id: Id,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveFilesToTrash {
    pub files: Vec<Id>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreFilesFromTrash {
    pub files: Vec<Id>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyTrash {
    pub namespace_id: Id,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveFiles {
    pub files: Vec<Id>,
    pub destination: Id,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFolder {
    pub parent_id: Id,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenameFile {
    pub file_id: Id,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteFileUpload {
    pub upload_id: Id,
    pub parent_id: Id,
    pub name: String,
    pub mime_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDownloadUrl {
    pub file_id: Id,
}
