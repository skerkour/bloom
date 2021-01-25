<p align="center">
    <a href="https://sentry.io" target="_blank" align="center">
        <img src="https://sentry-brand.storage.googleapis.com/sentry-logo-black.png" width="280">
    </a>
</p>

# Sentry Rust SDK: sentry

This crate provides support for logging events and errors / panics to the
[Sentry](https://sentry.io/) error logging service.  It integrates with the standard panic
system in Rust as well as a few popular error handling setups.

## Quickstart

The most convenient way to use this library is the [`sentry::init`] function,
which starts a sentry client with a default set of integrations, and binds
it to the current [`Hub`].

The [`sentry::init`] function returns a guard that when dropped will flush Events that were not
yet sent to the sentry service.  It has a two second deadline for this so shutdown of
applications might slightly delay as a result of this.  Keep the guard around or sending events
will not work.

```rust
let _guard = sentry::init("https://key@sentry.io/42");
sentry::capture_message("Hello World!", sentry::Level::Info);
// when the guard goes out of scope here, the client will wait up to two
// seconds to send remaining events to the service.
```

[`sentry::init`]: https://docs.rs/sentry/0.21.0/sentry/fn.init.html
[`Hub`]: https://docs.rs/sentry/0.21.0/sentry/struct.Hub.html

## Integrations

What makes this crate useful are the various integrations that exist.  Some of them are enabled
by default, some uncommon ones or for deprecated parts of the ecosystem a feature flag needs to
be enabled.  For the available integrations and how to use them see
[integrations](https://docs.rs/sentry/0.21.0/sentry/integrations/index.html) and [apply_defaults](https://docs.rs/sentry/0.21.0/sentry/fn.apply_defaults.html).

## Minimal API

This crate comes fully featured. If the goal is to instrument libraries for usage
with sentry, or to extend sentry with a custom [`Integration`] or a [`Transport`],
one should use the [`sentry-core`] crate instead.

[`Integration`]: https://docs.rs/sentry/0.21.0/sentry/trait.Integration.html
[`Transport`]: https://docs.rs/sentry/0.21.0/sentry/trait.Transport.html
[`sentry-core`]: https://crates.io/crates/sentry-core

## Features

Functionality of the crate can be turned on and off by feature flags.  This is the current list
of feature flags:

Default features:

* `backtrace`: Enables backtrace support.
* `contexts`: Enables capturing device, os, and rust contexts.
* `panic`: Enables support for capturing panics.
* `transport`: Enables the default transport, which is currently `reqwest` with `native-tls`.

Additional features:

* `anyhow`: Enables support for the `anyhow` crate.
* `debug-images`: Attaches a list of loaded libraries to events (currently only supported on unix).
* `error-chain`: Enables support for the `error-chain` crate.
* `failure`: Enables support for the `failure` crate.
* `log`: Enables support for the `log` crate.
* `env_logger`: Enables support for the `log` crate with additional `env_logger` support.
* `slog`: Enables support for the `slog` crate.
* `test`: Enables testing support.
* `debug-logs`: Uses the `log` crate for internal logging.
* `reqwest`: Enables the `reqwest` transport, which is currently the default.
* `curl`: Enables the curl transport.
* `surf`: Enables the surf transport.
* `native-tls`: Uses the `native-tls` crate, which is currently the default.
  This only has an effect on the `reqwest` transport.
* `rustls`: Enables the `rustls` support of the `reqwest` transport.
  Please note that `native-tls` is a default feature, and one needs to use
  `default-features = false` to completely disable building `native-tls` dependencies.

## Resources

License: Apache-2.0

- [Discord](https://discord.gg/ez5KZN7) server for project discussions.
- Follow [@getsentry](https://twitter.com/getsentry) on Twitter for updates
