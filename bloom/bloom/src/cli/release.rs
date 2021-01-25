use kernel::{config::Config, Error};

pub fn run() -> Result<(), Error> {
    let mut runtime = stdx::tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap();

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

    runtime.block_on(async move {
        let db = kernel::db::connect(&config.database).await?;
        kernel::db::migrate(&db).await?;
        Ok(())
    })
}
