<p align="center">
    <a href="https://sentry.io" target="_blank" align="center">
        <img src="https://sentry-brand.storage.googleapis.com/sentry-logo-black.png" width="280">
    </a>
</p>

# Sentry Rust SDK: sentry-core

This crate provides the core of the [Sentry](https://sentry.io/) SDK, which
can be used to log events and errors.

This crate is meant for integration authors and third party library authors
that want to instrument their code for sentry.

Regular users who wish to integrate sentry into their applications should
rather use the [`sentry`] crate, which comes with a default transport, and
a large set of integrations for various third-party libraries.

## Core Concepts

This crate follows the [Unified API] guidelines and is centered around
the concepts of [`Client`], [`Hub`] and [`Scope`], as well as the extension
points via the [`Integration`], [`Transport`] and [`TransportFactory`] traits.

## Minimal API

By default, this crate comes with a so-called "minimal" mode. This mode will
provide all the APIs needed to instrument code with sentry, and to write
sentry integrations, but it will blackhole a lot of operations.

In minimal mode some types are restricted in functionality. For instance
the [`Client`] is not available and the [`Hub`] does not retain all API
functionality.

## Features

* `feature = "client"`: Activates the [`Client`] type and certain
  [`Hub`] functionality.
* `feature = "test"`: Activates the [`test`] module, which can be used to
  write integration tests. It comes with a test transport which can capture
  all sent events for inspection.
* `feature = "debug-logs"`: Uses the `log` crate for debug output, instead
  of printing to `stderr`. This feature is **deprecated** and will be
  replaced by a dedicated log callback in the future.

[`sentry`]: https://crates.io/crates/sentry
[Unified API]: https://develop.sentry.dev/sdk/unified-api/
[`Client`]: https://docs.rs/sentry-core/0.21.0/sentry_core/struct.Client.html
[`Hub`]: https://docs.rs/sentry-core/0.21.0/sentry_core/struct.Hub.html
[`Scope`]: https://docs.rs/sentry-core/0.21.0/sentry_core/struct.Scope.html
[`Integration`]: https://docs.rs/sentry-core/0.21.0/sentry_core/trait.Integration.html
[`Transport`]: https://docs.rs/sentry-core/0.21.0/sentry_core/trait.Transport.html
[`TransportFactory`]: https://docs.rs/sentry-core/0.21.0/sentry_core/trait.TransportFactory.html
[`test`]: https://docs.rs/sentry-core/0.21.0/sentry_core/test/index.html

## Resources

License: Apache-2.0

- [Discord](https://discord.gg/ez5KZN7) server for project discussions.
- Follow [@getsentry](https://twitter.com/getsentry) on Twitter for updates
