use std::sync::Arc;

use kernel::{
    config::Config,
    drivers::{mailer::ses::SesMailer, queue::postgres::PostgresQueue, storage::s3::S3Storage},
};
// use tokio::task;

pub fn run() -> Result<(), kernel::Error> {
    actix_web::rt::System::new("server::run").block_on(async move {
        // let mut runtime = tokio::runtime::Builder::new()
        //     .threaded_scheduler()
        //     .enable_all()
        //     .build()
        //     .unwrap();

        let config = Config::load()?;

        // // see here for how to run actix-web in a tokio runtime https://github.com/actix/actix-web/issues/1283
        // let actix_system_local_set = task::LocalSet::new();
        // let sys = actix_web::rt::System::run_in_tokio("server::run", &actix_system_local_set);

        // runtime.block_on(async move {
        let db = kernel::db::connect(&config.database).await?;
        let queue = Arc::new(PostgresQueue::new(db.clone()));
        let mailer = Arc::new(SesMailer::new());
        let storage = Arc::new(S3Storage::new());

        let kernel_service = Arc::new(kernel::Service::new(config.clone(), db, queue, mailer, storage));

        /*
            if flag_worker {
                let worker_task = tokio::spawn(async move {
                    // run worker
                });
            }

            if flag_scheduler {
                let scheduler_task = tokio::spawn(async move {
                    // run scheduler
                });
            }

            join(worker_task, server_task, scheduler_task)???
        */

        http_server::run(kernel_service).await
        // Ok(sys.await?)
    })
}
