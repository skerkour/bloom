use super::Repository;
use crate::{entities::ChatboxPreferences, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx};

impl Repository {
    pub async fn update_chatbox_preferences<'c, C: Queryer<'c>>(
        &self,
        db: C,
        preferences: &ChatboxPreferences,
    ) -> Result<(), Error> {
        const QUERY: &str = "UPDATE inbox_chatbox_preferences SET
            updated_at = $1, color = $2, name = $3, avatar_storage_key = $4, show_branding = $5,
                welcome_message = $6
            WHERE id = $7";

        match sqlx::query(QUERY)
            .bind(preferences.updated_at)
            .bind(&preferences.color)
            .bind(&preferences.name)
            .bind(&preferences.avatar_storage_key)
            .bind(preferences.show_branding)
            .bind(&preferences.welcome_message)
            .bind(preferences.id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("files.update_chatbox_preferences: Updating preferences: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
