use super::Service;
use crate::{db::Queryer, entities::Namespace};

impl Service {
    pub async fn update_namespace<'c, C: Queryer<'c>>(&self, db: C, namespace: &Namespace) -> Result<(), crate::Error> {
        self.repo.update_namespace(db, namespace).await?;
        Ok(())
    }
}
