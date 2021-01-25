use kernel::config::{Config, Env};
use stdx::{env_logger::Builder, log::LevelFilter};

pub mod masterkey;
pub mod release;
pub mod scheduler;
pub mod server;
pub mod version;
pub mod worker;

pub const SERVER_SUBCOMMAND: &str = "server";
pub const SERVER_DESCRIPTION: &str = "Run the server";

pub const VERSION_SUBCOMMAND: &str = "version";
pub const VERSION_DESCRIPTION: &str = "Display the version and build information";

pub const MASTERKEY_SUBCOMMAND: &str = "masterkey";
pub const MASTERKEY_DESCRIPTION: &str = "Generates a base64 encoded master key";

pub const RELEASE_SUBCOMMAND: &str = "release";
pub const RELEASE_DESCRIPTION: &str = "Run all that is required for a new release. Database migrations and so...";

pub const SCHEDULER_SUBCOMMAND: &str = "scheduler";
pub const SCHEDULER_DESCRIPTION: &str = "Run the scheduler";

pub const WORKER_SUBCOMMAND: &str = "worker";
pub const WORKER_DESCRIPTION: &str = "Run the worker";

fn init_logger(config: &Config) {
    let log_level = if config.env == Env::Production {
        LevelFilter::Info
    } else {
        LevelFilter::Debug
    };

    Builder::new()
        .filter_level(log_level)
        .filter_module("sqlx::query", LevelFilter::Error)
        .init();
}

// need by sentry sdk...
fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
