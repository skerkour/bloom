use env_logger::Builder;
use kernel::{config::Config, Error};
use stdx::log::error;

pub fn run() -> Result<(), Error> {
    let config = Config::load()?;
    let log_level = if config.debug {
        stdx::log::LevelFilter::Debug
    } else {
        stdx::log::LevelFilter::Info
    };
    Builder::new().filter_level(log_level).init();

    error!("scheduler: TODO");
    Ok(())
}
