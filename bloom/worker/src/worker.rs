use kernel::{drivers::queue::Job, Error};

pub struct Worker {}

impl Worker {
    pub fn new() -> Self {
        Worker {}
    }

    pub async fn handle_job(&self, _job: Job) -> Result<(), Error> {
        Ok(())
    }
}
