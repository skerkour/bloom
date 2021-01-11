use super::Service;
use crate::{db::Queryer, entities::Namespace};

impl Service {
    pub async fn update_namespace<'c, C: Queryer<'c>>(
        &self,
        _db: C,
        _namespace: &Namespace,
    ) -> Result<(), crate::Error> {
        todo!();
    }
}
