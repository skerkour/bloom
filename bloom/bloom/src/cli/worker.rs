use env_logger::Builder;
use kernel::{
    config::{Config, Env},
    drivers::{mailer::ses::SesMailer, queue::postgres::PostgresQueue, storage::s3::S3Storage},
    Error,
};
use std::sync::Arc;
use stdx::log::LevelFilter;

pub fn run() -> Result<(), Error> {
    let config = Config::load()?;
    let log_level = if config.env == Env::Production {
        LevelFilter::Info
    } else {
        LevelFilter::Debug
    };
    Builder::new().filter_level(log_level).init();

    let mut runtime = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async move {
        let db = kernel::db::connect(&config.database).await?;
        let queue = Arc::new(PostgresQueue::new(db.clone()));
        let mailer = Arc::new(SesMailer::new());
        let storage = Arc::new(S3Storage::new());

        let kernel_service = Arc::new(kernel::Service::new(config, db.clone(), queue.clone(), mailer, storage));
        let analytics_service = Arc::new(analytics::Service::new(kernel_service.clone(), db, queue.clone()));

        worker::run(kernel_service, analytics_service, queue).await
    })
}
