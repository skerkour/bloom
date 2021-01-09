use kernel::{config::Config, Error};

pub fn run() -> Result<(), Error> {
    let mut runtime = stdx::tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap();

    let config = Config::load()?;

    runtime.block_on(async move {
        let db = kernel::db::connect(&config.database).await?;
        kernel::db::migrate(&db).await?;
        Ok(())
    })
}
