use crate::domain::messages::Message;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use stdx::chrono;

pub mod postgres;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub message: Message,
}

#[async_trait::async_trait]
pub trait Queue: Send + Sync + Debug {
    async fn push(
        &self,
        job: Message,
        scheduled_for: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<(), crate::Error>;
    /// pull fetches at most `number_of_jobs` from the queue.
    async fn pull(&self, number_of_jobs: u32) -> Result<Vec<Job>, crate::Error>;
    async fn delete_job(&self, job_id: String) -> Result<(), crate::Error>;
    async fn fail_job(&self, job_id: String) -> Result<(), crate::Error>;
    async fn clear(&self) -> Result<(), crate::Error>;
}

// // Queue interface represents more a queue borker than an individual queue
// type Queue interface {
// 	Register(ctx context.Context, queueName string, worker QueueWorker, concurrency uint64)
// 	// remove all messages from queue
// 	Stop(ctx context.Context) (err error)
// }
