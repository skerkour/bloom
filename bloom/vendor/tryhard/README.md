# 💫 tryhard

[![Embark](https://img.shields.io/badge/embark-open%20source-blueviolet.svg)](https://embark.dev)
[![Embark](https://img.shields.io/badge/discord-ark-%237289da.svg?logo=discord)](https://discord.gg/dAuKfZS)
[![Crates.io](https://img.shields.io/crates/v/tryhard.svg)](https://crates.io/crates/tryhard)
[![Docs](https://docs.rs/tryhard/badge.svg)](https://docs.rs/tryhard)
[![dependency status](https://deps.rs/repo/github/EmbarkStudios/tryhard/status.svg)](https://deps.rs/repo/github/EmbarkStudios/tryhard)
[![Build status](https://github.com/EmbarkStudios/tryhard/workflows/CI/badge.svg)](https://github.com/EmbarkStudios/tryhard/actions)

`tryhard` makes it easy to retry futures that might fail. You can control the number of retries, the backoff strategy, and the max duration.

## Examples

First imagine you have some async function that can fail:

```rust
async fn read_file(path: &str) -> Result<String, std::io::Error> {
    // ...
}
```

Calling that function and retrying at most 10 times with no delay between attempts can be done like so:

```rust
tryhard::retry_fn(|| read_file("Cargo.toml"))
    .retries(10)
    .await?;
```

You can also retry with a fixed delay between attempts:

```rust
tryhard::retry_fn(|| read_file("Cargo.toml"))
    .retries(10)
    .fixed_backoff(Duration::from_millis(100))
    .await?;
```

Or exponential backoff, where the delay doubles each time, with a max delay of 1 second:

```rust
tryhard::retry_fn(|| read_file("Cargo.toml"))
    .retries(10)
    .exponential_backoff(Duration::from_millis(10))
    .max_delay(Duration::from_secs(1))
    .await?;
```

See [the docs](https://docs.rs/tryhard) for more details.

## Contributing

[![Contributor Covenant](https://img.shields.io/badge/contributor%20covenant-v1.4-ff69b4.svg)](../main/CODE_OF_CONDUCT.md)

We welcome community contributions to this project.

Please read our [Contributor Guide](CONTRIBUTING.md) for more information on how to get started.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
