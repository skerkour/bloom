use crate::entities::Visitor;

use super::{FindOrCreateVisitorInput, Service};
use kernel::db::Queryer;

impl Service {
    pub async fn find_or_create_visitor<'c, C: Queryer<'c>>(
        &self,
        db: C,
        input: FindOrCreateVisitorInput,
    ) -> Result<Visitor, crate::Error> {
        todo!();
    }
}
