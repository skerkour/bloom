use crate::{config::Config, Service};
use std::sync::Arc;

impl Service {
    pub fn config(&self) -> Arc<Config> {
        Arc::clone(&self.config)
    }
}
