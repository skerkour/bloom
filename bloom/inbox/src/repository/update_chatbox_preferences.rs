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
            updated_at = $1, color = $2, name = $3, avatar_id = $4, show_branding = $5,
                welcome_message = $6, twitter = $7, facebook_url = $8, instagram = $9, whatsapp_number = $10,
                mastodon_url = $11, website_url = $12, telegram = $13
            WHERE id = $14";

        match sqlx::query(QUERY)
            .bind(preferences.updated_at)
            .bind(&preferences.color)
            .bind(&preferences.name)
            .bind(&preferences.avatar_id)
            .bind(preferences.show_branding)
            .bind(&preferences.welcome_message)
            .bind(&preferences.twitter)
            .bind(&preferences.facebook_url)
            .bind(&preferences.instagram)
            .bind(&preferences.whatsapp_number)
            .bind(&preferences.mastodon_url)
            .bind(&preferences.website_url)
            .bind(&preferences.telegram)
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
