extern crate security_framework;
#[cfg(target_os = "mac_os")]
use security_framework::os::macos::keychain::SecKeychain;
#[cfg(target_os = "mac_os")]
use security_framework::os::macos::passwords::*;

fn main() {
    #[cfg(target_os = "mac_os")] {
    let hostname = "example.com";
    let username = "rusty";
    let password = b"oxidize";

    let res = SecKeychain::default().unwrap().set_internet_password(
        hostname,
        None,
        username,
        "",
        None,
        SecProtocolType::HTTPS,
        SecAuthenticationType::HTMLForm,
        password,
    );
    match res {
        Ok(_) => {
            println!(
                "Password set for {}@{}. You can read it using find_internet_password example",
                username, hostname
            );
        }
        Err(err) => {
            eprintln!("Could not set password: {:?}", err);
        }
    }
}}
