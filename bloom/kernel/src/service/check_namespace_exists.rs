use super::Service;
use crate::{db::Queryer, errors::kernel::Error};

impl Service {
    /// returns true if a namespace exists. false otherwise
    pub async fn check_namespace_exists<'c, C: Queryer<'c>>(&self, db: C, path: &str) -> Result<bool, crate::Error> {
        let find_existing_namespace_res = self.repo.find_namespace_by_path(db, path).await;
        match find_existing_namespace_res {
            Ok(_) => Ok(true),
            Err(Error::NamespaceNotFound) => Ok(false),
            Err(err) => Err(err.into()),
        }
    }
}
