use crate::{entities::Visitor, Error};

use super::{FindOrCreateVisitorInput, Service};
use kernel::db::Queryer;
use stdx::{chrono::Utc, ulid::Ulid};

impl Service {
    pub async fn find_or_create_visitor<'c, 'a, C>(
        &self,
        db: &'a C,
        input: FindOrCreateVisitorInput,
    ) -> Result<Visitor, crate::Error>
    where
        &'a C: Queryer<'c>,
    {
        match self.repo.find_visitor_by_anonymous_id(db, input.anonymous_id).await {
            Ok(visitor) => Ok(visitor),
            Err(Error::VisitorNotFound) => {
                let now = Utc::now();
                let visitor = Visitor {
                    id: Ulid::new().into(),
                    created_at: now,
                    updated_at: now,
                    anonymous_id: input.anonymous_id,
                    namespace_id: input.namespace_id,
                };
                self.repo.create_visitor(db, &visitor).await?;
                Ok(visitor)
            }
            Err(err) => Err(err),
        }
    }
}
