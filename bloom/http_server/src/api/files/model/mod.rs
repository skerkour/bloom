use crate::api::scalars::{Id, Time};
use serde::{Deserialize, Serialize};

pub mod input;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub id: Id,
    pub created_at: Time,
    pub updated_at: Time,
    pub name: String,
    pub size: i64,
    pub r#type: String,
    pub explicitly_trashed: bool,
    pub trashed_at: Option<Time>,

    pub path: Vec<FilePath>,
    pub children: Option<Vec<File>>,
}

impl From<files::entities::File> for File {
    fn from(file: files::entities::File) -> Self {
        File {
            id: file.id,
            created_at: file.created_at,
            updated_at: file.updated_at,
            name: file.name,
            size: file.size,
            r#type: file.r#type,
            explicitly_trashed: file.explicitly_trashed,
            trashed_at: file.trashed_at,
            children: None,
            path: Vec::new(),
        }
    }
}

impl From<files::service::FileWithPathAndChildren> for File {
    fn from(file: files::service::FileWithPathAndChildren) -> Self {
        File {
            id: file.file.id,
            created_at: file.file.created_at,
            updated_at: file.file.updated_at,
            name: file.file.name,
            size: file.file.size,
            r#type: file.file.r#type,
            explicitly_trashed: file.file.explicitly_trashed,
            trashed_at: file.file.trashed_at,
            children: Some(file.children.into_iter().map(Into::into).collect()),
            path: file.path.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePath {
    pub id: Id,
    pub name: String,
}

impl From<files::entities::FilePath> for FilePath {
    fn from(path: files::entities::FilePath) -> FilePath {
        FilePath {
            id: path.id,
            name: path.name,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDownloadUrl {
    pub url: String,
}
