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
    let mut log_builder = Builder::new();
    log_builder.filter_level(log_level);

    error!("scheduler: TODO");
    Ok(())
}
