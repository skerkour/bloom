use super::Service;
use crate::entities::Analytics;
use kernel::Actor;
use stdx::uuid::Uuid;

impl Service {
    pub async fn find_analytics(&self, actor: Actor, namespace_id: Uuid) -> Result<Analytics, kernel::Error> {
        todo!();
    }
}
