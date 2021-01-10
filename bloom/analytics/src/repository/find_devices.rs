use super::Repository;
use crate::{entities::Device, Error};
use kernel::db::Queryer;
use stdx::uuid::Uuid;

impl Repository {
    pub async fn find_devices<'c, C: Queryer<'c>>(&self, db: C, namespace_id: Uuid) -> Result<Vec<Device>, Error> {
        todo!();
    }
}
