use super::Service;
use crate::service::Analytics;
use kernel::Actor;
use stdx::uuid::Uuid;

impl Service {
    pub async fn find_analytics(&self, actor: Actor, namespace_id: Uuid) -> Result<Analytics, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;
        self.kernel_service
            .check_namespace_membership(&self.db, &actor, namespace_id)
            .await?;

        let visits = self.repo.find_visits(&self.db, namespace_id).await?;
        let pages = self.repo.find_pages(&self.db, namespace_id).await?;
        let referrers = self.repo.find_referrers(&self.db, namespace_id).await?;
        let devices = self.repo.find_devices(&self.db, namespace_id).await?;
        let events = self.repo.find_events(&self.db, namespace_id).await?;

        let analytics = Analytics {
            visits,
            pages,
            referrers,
            devices,
            events,
        };
        Ok(analytics)
    }
}
