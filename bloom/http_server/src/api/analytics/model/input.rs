use crate::api::scalars::Id;
use serde::{Deserialize, Serialize};

pub type PageEvent = kernel::domain::analytics::events::PageEvent;
pub type TrackEvent = kernel::domain::analytics::events::TrackEvent;

#[derive(Serialize, Deserialize)]
pub struct Analytics {
    pub namespace_id: Id,
}
