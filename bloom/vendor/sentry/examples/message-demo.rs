fn main() {
    let _sentry = sentry::init(());
    sentry::configure_scope(|scope| {
        scope.set_fingerprint(Some(["a-message"].as_ref()));
        scope.set_tag("foo", "bar");
    });

    sentry::capture_message("This is recorded as a warning now", sentry::Level::Warning);
}
