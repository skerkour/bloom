use super::Repository;
use crate::{entities, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn find_chatbox_preferences_for_namespace<'c, C: Queryer<'c>>(
        &self,
        db: C,
        namespace_id: Uuid,
    ) -> Result<entities::ChatboxPreferences, Error> {
        const QUERY: &str = "SELECT * FROM inbox_chatbox_preferences
            WHERE namespace_id = $1";

        match sqlx::query_as::<_, entities::ChatboxPreferences>(QUERY)
            .bind(namespace_id)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                error!(
                    "inbox.find_chatbox_preferences_for_namespace: finding preferences: {}",
                    &err
                );
                Err(err.into())
            }
            Ok(None) => Err(Error::ChatboxPreferencesNotFound),
            Ok(Some(res)) => Ok(res),
        }
    }
}
