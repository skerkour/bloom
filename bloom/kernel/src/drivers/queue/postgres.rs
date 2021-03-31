use super::{Job, Queue};
use crate::{db::DB, domain::messages::Message};
use std::time::Duration;
use stdx::tokio::time::delay_for;
use stdx::{chrono, ulid::Ulid, uuid::Uuid};
use stdx::{
    log::error,
    sqlx::{self, types::Json},
};

#[derive(Debug, Clone)]
pub struct PostgresQueue {
    db: DB,
    max_attempts: u32,
}

const MAX_FAILED_ATTEMPTS: i32 = 3; // low, as most jobs also use retries internally
const TOO_LONG: Duration = Duration::from_secs(60 * 15); // 15 mins

/// A Job, as represented in DB
/// we use a BIGSERIAL instead of habitual UUID (ULID) to improve performance:
/// faster index insert / retrieves, smaller index size...
#[derive(sqlx::FromRow, Debug, Clone)]
struct PostgresJob {
    id: Uuid,
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
            id: item.id,
            message: item.message.0,
        }
    }
}

impl PostgresQueue {
    pub fn new(db: DB) -> PostgresQueue {
        let queue = PostgresQueue {
            db,
            max_attempts: 5,
        };

        let loop_queue = queue.clone();
        stdx::tokio::spawn(async move { loop_queue.watch_loop().await });

        queue
    }
}

#[async_trait::async_trait]
impl Queue for PostgresQueue {
    async fn push(&self, job: Message, date: Option<chrono::DateTime<chrono::Utc>>) -> Result<(), crate::Error> {
        let scheduled_for = date.unwrap_or(chrono::Utc::now());
        let failed_attempts = 0;
        let message = Json(job);
        let status = PostgresJobStatus::Queued;
        let now = chrono::Utc::now();
        let job_id: Uuid = Ulid::new().into();
        let query = "INSERT INTO kernel_queue
            (id, created_at, updated_at, scheduled_for, failed_attempts, status, message)
            VALUES ($1, $2, $3, $4, $5, $6, $7)";

        sqlx::query(query)
            .bind(job_id)
            .bind(now)
            .bind(now)
            .bind(scheduled_for)
            .bind(failed_attempts)
            .bind(status)
            .bind(message)
            .execute(&self.db)
            .await?;
        Ok(())
    }

    async fn delete_job(&self, job_id: Uuid) -> Result<(), crate::Error> {
        let query = "DELETE FROM kernel_queue WHERE id = $1";

        sqlx::query(query).bind(job_id).execute(&self.db).await?;
        Ok(())
    }

    async fn fail_job(&self, job_id: Uuid) -> Result<(), crate::Error> {
        let now = chrono::Utc::now();
        let query = "UPDATE kernel_queue
            SET status = $1, updated_at = $2, failed_attempts = failed_attempts + 1
            WHERE id = $3";

        sqlx::query(query)
            .bind(PostgresJobStatus::Queued)
            .bind(now)
            .bind(job_id)
            .execute(&self.db)
            .await?;
        Ok(())
    }

    async fn pull(&self, number_of_jobs: u32) -> Result<Vec<Job>, crate::Error> {
        let number_of_jobs = if number_of_jobs > 100 { 100 } else { number_of_jobs };
        let now = chrono::Utc::now();
        let query = "UPDATE kernel_queue
            SET status = $1, updated_at = $2
            WHERE id IN (
                SELECT id
                FROM kernel_queue
                WHERE status = $3 AND scheduled_for <= $4 AND failed_attempts < $5
                ORDER BY scheduled_for
                FOR UPDATE SKIP LOCKED
                LIMIT $6
            )
            RETURNING *";

        let jobs: Vec<PostgresJob> = sqlx::query_as::<_, PostgresJob>(query)
            .bind(PostgresJobStatus::Running)
            .bind(now)
            .bind(PostgresJobStatus::Queued)
            .bind(now)
            .bind(MAX_FAILED_ATTEMPTS)
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
    async fn watch_loop(&self) {
        loop {
            delay_for(Duration::from_secs(20)).await;
            self.mark_jobs_as_failed().await;
            self.requeue_stalled_jobs().await;
        }
    }

    async fn mark_jobs_as_failed(&self) {
        let now = chrono::Utc::now();
        let query = "UPDATE kernel_queue
            SET status = $1, updated_at = $2
            WHERE id IN (
                SELECT id
                FROM kernel_queue
                WHERE status = $3 AND failed_attempts >= $4
                FOR UPDATE SKIP LOCKED
            )";

        match sqlx::query(query)
            .bind(PostgresJobStatus::Failed)
            .bind(now)
            .bind(PostgresJobStatus::Queued)
            .bind(MAX_FAILED_ATTEMPTS)
            .execute(&self.db)
            .await
        {
            Ok(_) => {}
            Err(err) => {
                error!(
                    "kernel.driver.queue.postgres.mark_jobs_as_failed: marking jobs as failed: {}",
                    &err
                );
            }
        }
    }

    async fn requeue_stalled_jobs(&self) {
        let now = chrono::Utc::now();
        let too_long = chrono::Duration::from_std(TOO_LONG)
            .expect("kernel.driver.queue.postgres.requeue_stalled_jobs: converting duration");
        let query = "UPDATE kernel_queue
            SET status = $1, updated_at = $2
            WHERE id IN (
                SELECT id
                FROM kernel_queue
                WHERE status = $3 AND updated_at < $4
                FOR UPDATE SKIP LOCKED
            )";

        match sqlx::query(query)
            .bind(PostgresJobStatus::Queued)
            .bind(now)
            .bind(PostgresJobStatus::Running)
            .bind(now - too_long)
            .execute(&self.db)
            .await
        {
            Ok(_) => {}
            Err(err) => {
                error!(
                    "kernel.driver.queue.postgres.requeue_stalled_jobs: requeueing stalled jobs: {}",
                    &err
                );
            }
        }
    }
}
