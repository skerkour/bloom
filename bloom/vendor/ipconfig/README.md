# Ipconfig

**Get network adapters information and network configuration for windows.**

[![Build status](https://ci.appveyor.com/api/projects/status/tiwjo6q4eete0nmh/branch/master?svg=true)](https://ci.appveyor.com/project/liran-ringel/ipconfig/branch/master)
[![Crates.io](https://img.shields.io/crates/v/ipconfig.svg)](https://crates.io/crates/ipconfig)

[Documentation](https://docs.rs/ipconfig/0.2/x86_64-pc-windows-msvc/ipconfig/)

## Examples

```rust
// Print the ip addresses and dns servers of all adapters:
for adapter in ipconfig::get_adapters()? {
    println!("Ip addresses: {:#?}", adapter.ip_addresses());
    println!("Dns servers: {:#?}", adapter.dns_servers());
}
```

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
