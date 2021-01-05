use env_logger::Builder;
use kernel::{config::Config, Error};
use stdx::log::{error, LevelFilter};

pub fn run() -> Result<(), Error> {
    let config = Config::load()?;
    let log_level = if config.debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    Builder::new().filter_level(log_level).init();

    error!("scheduler: TODO");
    Ok(())
}
