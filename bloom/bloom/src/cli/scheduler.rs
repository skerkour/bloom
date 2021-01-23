use std::sync::Arc;

use kernel::{
    config::{Config, Env},
    drivers::{
        mailer::ses::SesMailer, queue::postgres::PostgresQueue, storage::s3::S3Storage, xss::stdx::StdxXssSanitizer,
    },
    Error,
};
use stdx::env_logger::Builder;
use stdx::log::LevelFilter;

pub fn run() -> Result<(), Error> {
    let config = Config::load()?;
    let log_level = if config.env == Env::Production {
        LevelFilter::Info
    } else {
        LevelFilter::Debug
    };
    Builder::new().filter_level(log_level).init();

    let mut runtime = stdx::tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async move {
        let db = kernel::db::connect(&config.database).await?;
        let queue = Arc::new(PostgresQueue::new(db.clone()));
        let mailer = Arc::new(SesMailer::new(&config));
        let storage = Arc::new(S3Storage::new(&config).await);
        let stdx_xss_sanitizer = Arc::new(StdxXssSanitizer::new());

        let kernel_service = Arc::new(kernel::Service::new(
            config,
            db.clone(),
            queue.clone(),
            mailer.clone(),
            storage.clone(),
            stdx_xss_sanitizer.clone(),
        ));
        let files_service = Arc::new(files::Service::new(kernel_service.clone(), db.clone(), storage));
        let inbox_service = Arc::new(inbox::Service::new(
            kernel_service.clone(),
            db,
            queue.clone(),
            stdx_xss_sanitizer,
            mailer.clone(),
        ));
        kernel_service.inject_missing_dependencies(files_service.clone(), inbox_service.clone());

        scheduler::run(kernel_service).await
    })
}
