use crate::api::scalars::{Id, Time};
use serde::{Deserialize, Serialize};

pub mod input;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    id: Id,
    created_at: Time,
}

impl From<files::entities::File> for File {
    fn from(file: files::entities::File) -> Self {
        File {
            id: file.id,
            created_at: file.created_at,
        }
    }
}
