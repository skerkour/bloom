use crate::api::scalars::Id;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub namespace: String,
    pub file_id: Option<Id>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trash {
    pub namespace: String,
}
