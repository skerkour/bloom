use std::sync::Arc;
use clap::ArgMatches;
use env_logger::Builder;
use kernel::{
    config::{Config, Env},
    drivers::{mailer::ses::SesMailer, queue::postgres::PostgresQueue, storage::s3::S3Storage},
};
use stdx::log::LevelFilter;
use tokio::{try_join};
// use tokio::task;

pub fn run(cli_matches: ArgMatches) -> Result<(), kernel::Error> {
    let config = Config::load()?;
    let log_level = if config.env == Env::Production {
        LevelFilter::Info
    } else {
        LevelFilter::Debug
    };
    Builder::new().filter_level(log_level).init();

    let worker_flag = cli_matches.is_present("worker");
    let scheduler_flag = cli_matches.is_present("scheduler");

    actix_web::rt::System::new("server::run").block_on(async move {
        // let mut runtime = tokio::runtime::Builder::new()
        //     .threaded_scheduler()
        //     .enable_all()
        //     .build()
        //     .unwrap();

        // // see here for how to run actix-web in a tokio runtime https://github.com/actix/actix-web/issues/1283
        // let actix_system_local_set = task::LocalSet::new();
        // let sys = actix_web::rt::System::run_in_tokio("server::run", &actix_system_local_set);

        // runtime.block_on(async move {
        let db = kernel::db::connect(&config.database).await?;
        let queue = Arc::new(PostgresQueue::new(db.clone()));
        let mailer = Arc::new(SesMailer::new());
        let storage = Arc::new(S3Storage::new());
        let mut worker_task = None;
        let mut scheduler_task = None;

        let kernel_service = Arc::new(kernel::Service::new(config, db, queue, mailer, storage));


            if worker_flag {
                worker_task = Some(tokio::spawn(async move {
                    // run worker
                }));
            }

            if scheduler_flag {
                scheduler_task = Some(tokio::spawn(async move {
                    // run scheduler
                }));
            }

        let http_server_task = http_server::run(kernel_service);

        match (scheduler_task, worker_task) {
            (Some(scheduler_task), Some(worker_task)) => {
                let _res = try_join!(http_server_task, scheduler_task, worker_task);
            },
            (Some(scheduler_task), None) => {
                let _res = try_join!(http_server_task, scheduler_task);
            },
            (None, Some(worker_task)) => {
                let _res = try_join!(http_server_task, worker_task);
            },
            (None, None) => {
                let _res = http_server_task.await;
            },
        }

        Ok(())
        // Ok(sys.await?)
    })
}
