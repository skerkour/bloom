#![cfg(feature = "test")]

use std::panic;
use std::sync::Arc;

#[test]
fn test_into_client() {
    let c: sentry::Client = sentry::Client::from_config("https://public@example.com/42");
    {
        let dsn = c.dsn().unwrap();
        assert_eq!(dsn.public_key(), "public");
        assert_eq!(dsn.host(), "example.com");
        assert_eq!(dsn.scheme(), sentry::types::Scheme::Https);
        assert_eq!(dsn.project_id().value(), 42);
    }

    let c: sentry::Client = sentry::Client::from_config((
        "https://public@example.com/42",
        sentry::ClientOptions {
            release: Some("foo@1.0".into()),
            ..Default::default()
        },
    ));
    {
        let dsn = c.dsn().unwrap();
        assert_eq!(dsn.public_key(), "public");
        assert_eq!(dsn.host(), "example.com");
        assert_eq!(dsn.scheme(), sentry::types::Scheme::Https);
        assert_eq!(dsn.project_id().value(), 42);
        assert_eq!(&c.options().release.as_ref().unwrap(), &"foo@1.0");
    }

    assert!(sentry::Client::from_config(()).options().dsn.is_none());
}

#[test]
fn test_unwind_safe() {
    let transport = sentry::test::TestTransport::new();
    let options = sentry::ClientOptions {
        dsn: Some("https://public@example.com/1".parse().unwrap()),
        transport: Some(Arc::new(transport.clone())),
        ..sentry::ClientOptions::default()
    };

    let client: Arc<sentry::Client> = Arc::new(options.into());

    panic::catch_unwind(|| {
        sentry::Hub::current().bind_client(Some(client));
        sentry::capture_message("Hello World!", sentry::Level::Warning);
    })
    .unwrap();

    sentry::Hub::current().bind_client(None);

    let events = transport.fetch_and_clear_events();

    assert_eq!(events.len(), 1);
}
