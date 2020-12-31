//! Get network adapters information for windows.
//!
//!
//! # Examples
//!
//! ```rust
//! # fn foo() -> ipconfig::error::Result<()> {
//! // Print the ip addresses and dns servers of all adapters:
//! for adapter in ipconfig::get_adapters()? {
//!     println!("Ip addresses: {:#?}", adapter.ip_addresses());
//!     println!("Dns servers: {:#?}", adapter.dns_servers());
//! }
//! # Ok(())
//! # }
//! # fn main() {
//!     # foo().unwrap();
//! # }
//! ```

#![cfg(windows)]
#![doc(html_root_url = "https://docs.rs/ipconfig/0.2/x86_64-pc-windows-msvc/ipconfig/")]

extern crate widestring;
extern crate winapi;
mod adapter;
mod bindings;
pub mod computer;
pub mod error;

pub use adapter::{get_adapters, Adapter, IfType, OperStatus};
