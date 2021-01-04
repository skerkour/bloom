use std::sync::Arc;

use kernel::{config::Config, drivers::queue::postgres::PostgresQueue, Error};

pub fn run() -> Result<(), Error> {
    let mut runtime = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap();

    let config = Config::load()?;

    runtime.block_on(async move {
        let db = kernel::db::connect(&config.database).await?;
        let queue = Arc::new(PostgresQueue::new(db.clone()));

        worker::run(queue, config.worker.concurrency).await
    })
}
