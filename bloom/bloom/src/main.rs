use clap::{App, Arg, SubCommand};

mod cli;

// Since Rust no longer uses jemalloc by default, bloom will, by default,
// use the system allocator. On Linux, this would normally be glibc's
// allocator, which is pretty good. In particular, bloom does not have a
// particularly allocation heavy workload, so there really isn't much
// difference (for bloom's purposes) between glibc's allocator and jemalloc.
//
// However, when bloom is built with musl, this means bloom will use musl's
// allocator, which appears to be substantially worse. (musl's goal is not to
// have the fastest version of everything. Its goal is to be small and amenable
// to static compilation.) Therefore,
// when building with musl, we use jemalloc.
//
// We don't unconditionally use jemalloc because it can be nice to use the
// system's default allocator by default. Moreover, jemalloc seems to increase
// compilation times by a bit.
//
// Moreover, we only do this on 64-bit systems since jemalloc doesn't support
// i686.
#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() -> Result<(), kernel::Error> {
    stdx::crypto::init()?;

    let cli = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .subcommand(SubCommand::with_name(cli::MASTERKEY_SUBCOMMAND).about(cli::MASTERKEY_DESCRIPTION))
        .subcommand(SubCommand::with_name(cli::RELEASE_SUBCOMMAND).about(cli::RELEASE_DESCRIPTION))
        .subcommand(SubCommand::with_name(cli::SCHEDULER_SUBCOMMAND).about(cli::SCHEDULER_DESCRIPTION))
        .subcommand(SubCommand::with_name(cli::VERSION_SUBCOMMAND).about(cli::VERSION_DESCRIPTION))
        .subcommand(
            SubCommand::with_name(cli::SERVER_SUBCOMMAND)
                .arg(
                    Arg::with_name("scheduler")
                        .long("scheduler")
                        .help("Also run the scheduler in parallel"),
                )
                .arg(
                    Arg::with_name("worker")
                        .long("worker")
                        .help("Also run the worker in parallel"),
                )
                .about(cli::SERVER_DESCRIPTION),
        )
        .subcommand(SubCommand::with_name(cli::WORKER_SUBCOMMAND).about(cli::WORKER_DESCRIPTION))
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .setting(clap::AppSettings::DisableVersion)
        .setting(clap::AppSettings::VersionlessSubcommands)
        .get_matches();

    if let Some(_) = cli.subcommand_matches(cli::VERSION_SUBCOMMAND) {
        cli::version::run();
    } else if let Some(server_matches) = cli.subcommand_matches(cli::SERVER_SUBCOMMAND) {
        cli::server::run(server_matches)?;
    } else if let Some(_) = cli.subcommand_matches(cli::MASTERKEY_SUBCOMMAND) {
        cli::masterkey::run()?;
    } else if let Some(_) = cli.subcommand_matches(cli::RELEASE_SUBCOMMAND) {
        cli::release::run()?;
    } else if let Some(_) = cli.subcommand_matches(cli::WORKER_SUBCOMMAND) {
        cli::worker::run()?;
    } else if let Some(_) = cli.subcommand_matches(cli::SCHEDULER_SUBCOMMAND) {
        cli::scheduler::run()?;
    }

    Ok(())
}
