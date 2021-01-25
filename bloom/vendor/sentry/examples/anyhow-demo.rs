use anyhow_ as anyhow;

fn execute() -> anyhow::Result<usize> {
    let parsed = "NaN".parse()?;
    Ok(parsed)
}

fn main() {
    let _sentry = sentry::init(sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    });

    if let Err(err) = execute() {
        println!("error: {}", err);
        sentry_anyhow::capture_anyhow(&err);
    }
}
