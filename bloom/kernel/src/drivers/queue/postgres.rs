use super::{Job, Queue};
use crate::{db::DB, domain::messages::Message};
use sqlx::types::Json;
use std::time::Duration;
use stdx::chrono;
use tokio::time::delay_for;

#[derive(Debug, Clone)]
pub struct PostgresQueue {
    db: DB,
    max_attempts: u32,
}

/// A Job, as represented in DB
/// we use a BIGSERIAL instead of habitual UUID (ULID) to improve performance:
/// faster index insert / retrieves, smaller index size...
#[derive(sqlx::FromRow, Debug, Clone)]
struct PostgresJob {
    id: i64,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,

    scheduled_for: chrono::DateTime<chrono::Utc>,
    failed_attempts: i32, // or retries_count or receive_count or delivery_attempts
    // priority: i64,
    status: PostgresJobStatus,
    message: Json<Message>,
}

// We use a INT postgres representation for performance reasons
#[derive(Debug, Clone, sqlx::Type, PartialEq)]
#[repr(i32)]
enum PostgresJobStatus {
    Queued,
    Running,
    Failed,
}

impl From<PostgresJob> for Job {
    fn from(item: PostgresJob) -> Self {
        Job {
            id: item.id.to_string(),
            message: item.message.0,
        }
    }
}

impl PostgresQueue {
    pub fn new(db: DB) -> PostgresQueue {
        let queue = PostgresQueue { db, max_attempts: 5 };

        let loop_queue = queue.clone();
        tokio::spawn(async move { loop_queue.watch_loop().await });

        queue
    }
}

#[async_trait::async_trait]
impl Queue for PostgresQueue {
    async fn push(&self, job: Message, date: Option<chrono::DateTime<chrono::Utc>>) -> Result<(), crate::Error> {
        let scheduled_for = date.unwrap_or(chrono::Utc::now());
        let receive_count = 0;
        let message = Json(job);
        let status = PostgresJobStatus::Queued;
        let now = chrono::Utc::now();
        let query = "INSERT INTO kernel_queue
            (created_at, updated_at, scheduled_for, receive_count, status, message)
            VALUES ($1, $2, $3, $4, $5, $6)";

        sqlx::query(query)
            .bind(now)
            .bind(now)
            .bind(scheduled_for)
            .bind(receive_count)
            .bind(status)
            .bind(message)
            .execute(&self.db)
            .await?;
        Ok(())
    }

    async fn delete_job(&self, job_id: String) -> Result<(), crate::Error> {
        let query = "DELETE FROM kernel_queue WHERE id = $1";
        let job_id = job_id.parse::<i64>()?;

        sqlx::query(query).bind(job_id).execute(&self.db).await?;
        Ok(())
    }

    async fn fail_job(&self, job_id: String) -> Result<(), crate::Error> {
        unimplemented!(); // TODO
    }

    async fn pull(&self, number_of_jobs: u32) -> Result<Vec<Job>, crate::Error> {
        let number_of_jobs = if (number_of_jobs > 100) { 100 } else { number_of_jobs };
        let now = chrono::Utc::now();
        let query = "UPDATE kernel_queue
            SET status = $1, updated_at = $2
            WHERE id IN (
                SELECT id
                FROM kernel_queue
                WHERE status = $3 AND scheduled_for >= $4
                ORDER BY scheduled_for
                FOR UPDATE SKIP LOCKED
                LIMIT $5
            )
            RETURNING *";

        let jobs: Vec<PostgresJob> = sqlx::query_as::<_, PostgresJob>(query)
            .bind(PostgresJobStatus::Running)
            .bind(now)
            .bind(PostgresJobStatus::Queued)
            .bind(now)
            .bind(number_of_jobs)
            .fetch_all(&self.db)
            .await?;
        Ok(jobs.into_iter().map(Into::into).collect())
    }

    async fn clear(&self) -> Result<(), crate::Error> {
        let query = "DELETE FROM kernel_queue";

        sqlx::query(query).execute(&self.db).await?;
        Ok(())
    }
}

impl PostgresQueue {
    // TODO: repush jobs that were running for to much time
    async fn watch_loop(&self) {
        loop {
            delay_for(Duration::from_secs(5)).await;
            // unimplemented!() // TODO
        }
    }
}
