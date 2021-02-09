use super::Repository;
use crate::{entities, Error};
use kernel::db::Queryer;
use stdx::{
    chrono::{TimeZone, Utc},
    log::error,
    sqlx,
    ulid::Ulid,
    uuid::Uuid,
};

impl Repository {
    pub async fn find_inbox_messages_for_conversation<'c, C: Queryer<'c>>(
        &self,
        db: C,
        conversation_id: Uuid,
        after: Option<Uuid>,
    ) -> Result<Vec<entities::Message>, Error> {
        const QUERY: &str = "SELECT * FROM inbox_messages
            WHERE conversation_id = $1
                AND id > $2
            ORDER BY id";

        let after = after.unwrap_or(Ulid::from_datetime(Utc.ymd(1970, 1, 1).and_hms(0, 0, 0)).into());

        match sqlx::query_as::<_, entities::Message>(QUERY)
            .bind(conversation_id)
            .bind(after)
            .fetch_all(db)
            .await
        {
            Err(err) => {
                error!("inbox.find_inbox_messages_for_conversation: Finding messages: {}", &err);
                Err(err.into())
            }
            Ok(messages) => Ok(messages),
        }
    }
}
