use super::Repository;
use crate::{entities::ChatboxPreferences, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx};

impl Repository {
    pub async fn create_chatbox_preferences<'c, C: Queryer<'c>>(
        &self,
        db: C,
        preferences: &ChatboxPreferences,
    ) -> Result<(), Error> {
        const QUERY: &str = "INSERT INTO inbox_chatbox_preferences
        (id, created_at, updated_at, color, name, avatar_storage_key, show_branding, welcome_message, namespace_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)";

        match sqlx::query(QUERY)
            .bind(preferences.id)
            .bind(preferences.created_at)
            .bind(preferences.updated_at)
            .bind(&preferences.color)
            .bind(&preferences.name)
            .bind(&preferences.avatar_storage_key)
            .bind(preferences.show_branding)
            .bind(&preferences.welcome_message)
            .bind(preferences.namespace_id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("inbox.create_chatbox_preferences: Inserting preferences: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
