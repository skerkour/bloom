use super::Service;
use crate::{entities, errors::kernel::Error, Actor};
use stdx::uuid::Uuid;

impl Service {
    pub async fn get_billing_information(
        &self,
        actor: Actor,
        namespace_id: Uuid,
    ) -> Result<entities::BillingInformation, crate::Error> {
        let actor = self.current_user(actor)?;

        self.check_namespace_membership(&self.db, &actor, namespace_id).await?;

        let namespace = self.repo.find_namespace_by_id(&self.db, namespace_id).await?;

        let customer = match self.repo.find_customer_by_namespace_id(&self.db, namespace_id).await {
            Ok(customer) => Ok(Some(customer)),
            Err(Error::CustomerNotFound) => Ok(None),
            Err(err) => Err(err),
        }?;

        Ok(entities::BillingInformation {
            total_storage: self.get_storage_for_plan(namespace.plan),
            namespace,
            customer,
        })
    }
}
