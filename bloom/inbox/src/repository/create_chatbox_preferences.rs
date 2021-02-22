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
            (id, created_at, updated_at, color, name, avatar_id, show_branding, welcome_message, namespace_id,
                twitter, facebook_url, instagram, whatsapp_number, mastodon_url, website_url, telegram)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)";

        match sqlx::query(QUERY)
            .bind(preferences.id)
            .bind(preferences.created_at)
            .bind(preferences.updated_at)
            .bind(&preferences.color)
            .bind(&preferences.name)
            .bind(&preferences.avatar_id)
            .bind(preferences.show_branding)
            .bind(&preferences.welcome_message)
            .bind(preferences.namespace_id)
            .bind(&preferences.twitter)
            .bind(&preferences.facebook_url)
            .bind(&preferences.instagram)
            .bind(&preferences.whatsapp_number)
            .bind(&preferences.mastodon_url)
            .bind(&preferences.website_url)
            .bind(&preferences.telegram)
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
