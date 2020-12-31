//! This module implements things that are related to the computer, rather than a specific adapter.

use std::string::String;

use winapi::shared::minwindef::HKEY;
use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::enums::KEY_READ;
use winreg::types::FromRegValue;
use winreg::RegKey;

use crate::error::*;

/// Returns a value from the registry, and returns a default if it doesn't exist.
fn get_value<T: FromRegValue>(
    predef: HKEY,
    subkey: &str,
    value_name: &str,
    default: T,
) -> ::std::io::Result<T> {
    let key = RegKey::predef(predef);

    let value: T = match key.open_subkey_with_flags(subkey, KEY_READ) {
        Ok(key) => match key.get_value(value_name) {
            Ok(value) => value,
            Err(err) => match err.kind() {
                ::std::io::ErrorKind::NotFound => default,
                _ => return Err(err),
            },
        },
        Err(err) => match err.kind() {
            ::std::io::ErrorKind::NotFound => default,
            _ => return Err(err),
        },
    };

    Ok(value)
}

const TCPIP_PARAMETERS_KEY_PATH: &str = "SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters";

/// Returns the DNS suffix search list for the network connection used by the computer.
pub fn get_search_list() -> Result<Vec<String>> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let params_key = hklm.open_subkey_with_flags(TCPIP_PARAMETERS_KEY_PATH, KEY_READ)?;
    let search_list: ::std::io::Result<String> = params_key.get_value("SearchList");
    if let Ok(search_list) = search_list {
        let search_list: Vec<String> = search_list.split(',').map(std::string::ToString::to_string).collect();
        Ok(search_list)
    } else {
        Ok(vec![])
    }
}

/// Returns the computer domain name (if any).
/// Returns `None` if the computer does not belong to a domain.
pub fn get_domain() -> Result<Option<String>> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let params_key = hklm.open_subkey_with_flags(TCPIP_PARAMETERS_KEY_PATH, KEY_READ)?;
    let domain: String = params_key.get_value("Domain")?;

    let domain = if domain.is_empty() {
        None
    } else {
        Some(domain)
    };
    Ok(domain)
}

/// Returns `true` if the computer is configured to use the round robin strategy.
/// Otherwise, returns `false`.
pub fn is_round_robin_enabled() -> Result<bool> {
    let rotate: u32 = get_value(
        HKEY_LOCAL_MACHINE,
        "SYSTEM\\CurrentControlSet\\Services\\DNS\\Parameters",
        "RoundRobin",
        1, // The default is 1 according to msdn
    )?;
    Ok(rotate != 0)
}
