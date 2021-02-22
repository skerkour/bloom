use super::Repository;
use crate::{entities::Contact, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx};

impl Repository {
    pub async fn create_contact<'c, C: Queryer<'c>>(&self, db: C, contact: &Contact) -> Result<(), Error> {
        const QUERY: &str = "INSERT INTO inbox_contacts
            (id, created_at, updated_at, name, birthday, email, pgp_key, phone, address, website, twitter, instagram, facebook,
                linkedin, skype, telegram, bloom, notes, country, country_code, plan, user_id, avatar_id, namespace_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18,
                $19, $20, $21, $22, $23, $24)";

        match sqlx::query(QUERY)
            .bind(contact.id)
            .bind(contact.created_at)
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
            .bind(contact.namespace_id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("inbox.create_contact: Inserting contact: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
