use super::Repository;
use crate::{entities::Contact, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx};

impl Repository {
    pub async fn update_contact<'c, C: Queryer<'c>>(&self, db: C, contact: &Contact) -> Result<(), Error> {
        const QUERY: &str = "UPDATE inbox_contacts SET
                updated_at = $1, name = $2, birthday = $3, email = $4, pgp_key = $5, phone = $6, address = $7,
                website = $8, twitter = $9, instagram = $10, facebook = $11, linkedin = $12, skype = $13, telegram = $14,
                bloom = $15, notes = $16, country = $17, country_code = $18, plan = $19, user_id = $20, avatar_id = $21
            WHERE id = $22";

        match sqlx::query(QUERY)
            .bind(contact.updated_at)
            .bind(&contact.name)
            .bind(contact.birthday)
            .bind(&contact.email)
            .bind(&contact.pgp_key)
            .bind(&contact.phone)
            .bind(&contact.address)
            .bind(&contact.website)
            .bind(&contact.twitter)
            .bind(&contact.instagram)
            .bind(&contact.facebook)
            .bind(&contact.linkedin)
            .bind(&contact.skype)
            .bind(&contact.telegram)
            .bind(&contact.bloom)
            .bind(&contact.notes)
            .bind(&contact.country)
            .bind(&contact.country_code)
            .bind(&contact.plan)
            .bind(&contact.user_id)
            .bind(&contact.avatar_id)
            .bind(contact.id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("files.update_contact: Updating contact: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
