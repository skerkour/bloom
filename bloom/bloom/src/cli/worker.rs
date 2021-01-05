use env_logger::Builder;
use kernel::{config::Config, drivers::queue::postgres::PostgresQueue, Error};
use std::sync::Arc;

pub fn run() -> Result<(), Error> {
    let config = Config::load()?;
    let log_level = if config.debug {
        stdx::log::LevelFilter::Debug
    } else {
        stdx::log::LevelFilter::Info
    };
    let mut log_builder = Builder::new();
    log_builder.filter_level(log_level);

    let mut runtime = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async move {
        let db = kernel::db::connect(&config.database).await?;
        let queue = Arc::new(PostgresQueue::new(db.clone()));

        worker::run(queue, config.worker.concurrency).await
    })
}
