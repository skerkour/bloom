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
        }
    }
}
