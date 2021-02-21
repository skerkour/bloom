use super::Repository;
use crate::{service::NewsletterListAcquisition, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx, uuid::Uuid};

impl Repository {
    pub async fn find_newsletter_list_acquisition<'c, C: Queryer<'c>>(
        &self,
        db: C,
        list_id: Uuid,
    ) -> Result<Vec<NewsletterListAcquisition>, Error> {
        const QUERY: &str = "WITH subscriptions AS (
            SELECT * FROM newsletter_lists_subscriptions
            WHERE list_id = $1 AND created_at > (CURRENT_DATE - INTERVAL '31 days')
        )
        SELECT date.date, COUNT(id) AS new_contacts
        FROM (
            SELECT to_char(date_trunc('day', (current_date - offs)), 'YYYY-MM-DD')
            AS date
            FROM generate_series(0, 30, 1)
            AS offs
        ) date LEFT OUTER JOIN
        subscriptions
        ON date.date = to_char(date_trunc('day', subscriptions.created_at), 'YYYY-MM-DD')
        GROUP BY date.date
        ORDER BY date;
        ";

        match sqlx::query_as::<_, NewsletterListAcquisition>(QUERY)
            .bind(list_id)
            .fetch_all(db)
            .await
        {
            Err(err) => {
                error!("inbox.find_newsletter_list_acquisition: finding acquisition: {}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }
}
