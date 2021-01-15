use clap::ArgMatches;
use kernel::{
    config::{Config, Env},
    drivers::{mailer::ses::SesMailer, queue::postgres::PostgresQueue, storage::s3::S3Storage},
};
use std::sync::Arc;
use stdx::env_logger::Builder;
use stdx::log::debug;
use stdx::log::LevelFilter;
use stdx::tokio::task;

// #[tokio::main]
pub fn run(cli_matches: &ArgMatches) -> Result<(), kernel::Error> {
    let config = Config::load()?;
    let log_level = if config.env == Env::Production {
        LevelFilter::Info
    } else {
        LevelFilter::Debug
    };
    Builder::new().filter_level(log_level).init();

    let worker_flag = cli_matches.is_present("worker");
    let scheduler_flag = cli_matches.is_present("scheduler");
    debug!(
        "server.run: worker_flag={}, scheduler_flag={}",
        worker_flag, scheduler_flag
    );

    // see here for how to run actix-web in a tokio runtime https://github.com/actix/actix-web/issues/1283
    let mut runtime = stdx::tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()
        .expect("cli/server/run: building tokio runtime");

    runtime.block_on(async move {
        let actix_system_local_set = task::LocalSet::new();
        let sys = actix_web::rt::System::run_in_tokio("server::run", &actix_system_local_set);

        let db = kernel::db::connect(&config.database).await?;
        let queue = Arc::new(PostgresQueue::new(db.clone()));
        let mailer = Arc::new(SesMailer::new());
        let storage = Arc::new(S3Storage::new());

        let kernel_service = Arc::new(kernel::Service::new(
            config,
            db.clone(),
            queue.clone(),
            mailer,
            storage.clone(),
        ));
        let files_service = Arc::new(files::Service::new(kernel_service.clone(), db.clone(), storage));
        let analytics_service = Arc::new(analytics::Service::new(
            kernel_service.clone(),
            db.clone(),
            queue.clone(),
        ));
        let inbox_service = Arc::new(inbox::Service::new(kernel_service.clone(), db, queue.clone()));
        kernel_service.inject_missing_dependencies(files_service.clone(), inbox_service.clone());

        if worker_flag {
            let kernel_service = kernel_service.clone();
            let analytics_service = analytics_service.clone();
            stdx::tokio::spawn(async move { worker::run(kernel_service, analytics_service, queue).await });
            // TODO: handle error?
        }

        if scheduler_flag {
            let kernel_service = kernel_service.clone();
            stdx::tokio::spawn(async move { scheduler::run(kernel_service).await });
            // TODO: handle error ?
        }

        http_server::run(kernel_service.clone(), files_service, analytics_service, inbox_service).await?;

        Ok(sys.await?)
    })
}
