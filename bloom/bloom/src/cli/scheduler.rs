use env_logger::Builder;
use kernel::{
    config::{Config, Env},
    Error,
};
use stdx::log::{error, LevelFilter};

pub fn run() -> Result<(), Error> {
    let config = Config::load()?;
    let log_level = if config.env == Env::Production {
        LevelFilter::Info
    } else {
        LevelFilter::Debug
    };
    Builder::new().filter_level(log_level).init();

    error!("scheduler: TODO");
    Ok(())
}
