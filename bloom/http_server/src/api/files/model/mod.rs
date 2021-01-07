use crate::api::scalars::{Id, Time};
use serde::{Deserialize, Serialize};

pub mod input;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    id: Id,
    created_at: Time,
}
