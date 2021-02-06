use clap::ArgMatches;
use kernel::{
    config::Config,
    drivers::{
        mailer::ses::SesMailer, queue::postgres::PostgresQueue, storage::s3::S3Storage, xss::stdx::StdxXssSanitizer,
    },
};
use std::sync::Arc;
use stdx::log::debug;
use stdx::tokio::task;

// #[tokio::main]
pub fn run(cli_matches: &ArgMatches) -> Result<(), kernel::Error> {
    let config = Config::load()?;

    super::init_logger(&config);

    let sentry_env = super::string_to_static_str(config.env.to_string());
    let _sentry_guard = if let Some(ref sentry_dsn) = config.sentry.dsn {
        Some(sentry::init((
            sentry_dsn.as_str(),
            sentry::ClientOptions {
                environment: Some(sentry_env.into()),
                release: sentry::release_name!(),
                ..Default::default()
            },
        )))
    } else {
        None
    };
    std::env::set_var("RUST_BACKTRACE", "1");

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
        let analytics_service = Arc::new(analytics::Service::new(
            kernel_service.clone(),
            db.clone(),
            queue.clone(),
        ));
        let inbox_service = Arc::new(inbox::Service::new(
            kernel_service.clone(),
            db.clone(),
            queue.clone(),
            stdx_xss_sanitizer,
            mailer.clone(),
        ));
        let calendar_service = Arc::new(calendar::Service::new(kernel_service.clone(), db.clone()));

        kernel_service.inject_missing_dependencies(files_service.clone(), inbox_service.clone());

        if worker_flag {
            let kernel_service = kernel_service.clone();
            let analytics_service = analytics_service.clone();
            let inbox_service = inbox_service.clone();
            stdx::tokio::spawn(
                async move { worker::run(kernel_service, analytics_service, inbox_service, queue).await },
            );
            // TODO: handle error?
        }

        if scheduler_flag {
            let kernel_service = kernel_service.clone();
            stdx::tokio::spawn(async move { scheduler::run(kernel_service).await });
            // TODO: handle error ?
        }

        http_server::run(
            kernel_service.clone(),
            files_service,
            analytics_service,
            inbox_service,
            calendar_service,
        )
        .await?;

        Ok(sys.await?)
    })
}
