use version_check::{Channel, Version};

// We assume that features are never stabilized in patch versions.
// If a "Rust 2.0" is ever released, we'll have to handle that explicitly.
const MSRV_MINOR: u16 = 31;
const CURRENT_MINOR: u16 = 49;

fn main() {
    let msrv = Version::from_mmp(1, MSRV_MINOR, 0);

    let mut minor_used = match Version::read() {
        Some(version) => version,
        None => {
            println!(
                "cargo:warning=Unable to determine rustc version. Assuming rustc {}.",
                msrv
            );
            msrv
        }
    }
    .to_mmp()
    .1;

    // Treat as the stable release, even if not on it.
    let channel = Channel::read();
    match channel {
        Some(channel) if channel.is_beta() => minor_used -= 1,
        Some(channel) if channel.is_nightly() => minor_used -= 2,
        Some(channel) if channel.is_dev() => minor_used -= 3,
        _ => {}
    }

    for minor in (MSRV_MINOR + 1)..=CURRENT_MINOR {
        if minor <= minor_used {
            println!("cargo:rustc-cfg=__standback_since_1_{}", minor);
        } else {
            println!("cargo:rustc-cfg=__standback_before_1_{}", minor);
        }
    }
}
