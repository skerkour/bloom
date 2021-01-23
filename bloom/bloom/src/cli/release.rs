use kernel::{
    config::{Config, Env},
    Error,
};
use stdx::{env_logger::Builder, log::LevelFilter};

pub fn run() -> Result<(), Error> {
    let mut runtime = stdx::tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap();

    let config = Config::load()?;

    let log_level = if config.env == Env::Production {
        LevelFilter::Info
    } else {
        LevelFilter::Debug
    };
    Builder::new().filter_level(log_level).init();

    runtime.block_on(async move {
        let db = kernel::db::connect(&config.database).await?;
        kernel::db::migrate(&db).await?;
        Ok(())
    })
}
