use crate::Service;
use stdx::uuid::Uuid;

impl Service {
    pub fn unsubscribe_link(&self, subscription_id: Uuid) -> String {
        format!(
            "{}/unsubscribe?subscription={}",
            self.kernel_service.base_url(),
            subscription_id.to_hyphenated().to_string()
        )
    }
}
