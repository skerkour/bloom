use crate::domain::messages::Message;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use stdx::{chrono, uuid::Uuid};

pub mod postgres;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: Uuid,
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
    async fn delete_job(&self, job_id: Uuid) -> Result<(), crate::Error>;
    async fn fail_job(&self, job_id: Uuid) -> Result<(), crate::Error>;
    async fn clear(&self) -> Result<(), crate::Error>;
}

#[cfg(test)]
pub mod test {
    use super::{Job, Message, Queue, Uuid};
    use stdx::chrono;

    #[derive(Clone, Debug)]
    pub struct QueueMock {}

    impl QueueMock {
        pub fn new() -> Self {
            QueueMock {}
        }
    }

    #[async_trait::async_trait]
    impl Queue for QueueMock {
        async fn push(&self, _: Message, _: Option<chrono::DateTime<chrono::Utc>>) -> Result<(), crate::Error> {
            Ok(())
        }

        async fn pull(&self, _: u32) -> Result<Vec<Job>, crate::Error> {
            Ok(Vec::new())
        }

        async fn delete_job(&self, _: Uuid) -> Result<(), crate::Error> {
            Ok(())
        }

        async fn fail_job(&self, _: Uuid) -> Result<(), crate::Error> {
            Ok(())
        }

        async fn clear(&self) -> Result<(), crate::Error> {
            Ok(())
        }
    }
}
