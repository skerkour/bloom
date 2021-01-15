use super::InitNamespaceInput;
use crate::Service;
use kernel::db::Queryer;

impl Service {
    pub async fn init_namespace<'c, C: Queryer<'c>>(
        &self,
        _db: C,
        _input: InitNamespaceInput,
    ) -> Result<(), kernel::Error> {
        todo!();
    }
}
