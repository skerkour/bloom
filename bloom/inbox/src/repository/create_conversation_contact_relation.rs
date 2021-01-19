use super::Repository;
use crate::{entities::ConversationContactRelation, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx};

impl Repository {
    pub async fn create_conversation_contact_relation<'c, C: Queryer<'c>>(
        &self,
        db: C,
        relation: &ConversationContactRelation,
    ) -> Result<(), Error> {
        const QUERY: &str = "INSERT INTO inbox_conversations_contacts
            (contact_id, conversation_id)
            VALUES ($1, $2)";

        match sqlx::query(QUERY)
            .bind(relation.contact_id)
            .bind(relation.conversation_id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!(
                    "inbox.create_conversation_contact_relation: Inserting relation: {}",
                    &err
                );
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
