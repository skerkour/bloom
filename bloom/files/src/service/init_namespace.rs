use crate::Service;
use kernel::db::Queryer;
use stdx::uuid::Uuid;

impl Service {
    pub async fn init_namespace<'c, C: Queryer<'c>>(&self, _db: C, _namespace_id: Uuid) -> Result<(), crate::Error> {
        todo!(); // TODO
    }
}
