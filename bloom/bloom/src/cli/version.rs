use std::env;

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
struct VersionInfo {
    pub name: String,
    pub version: String,
    pub git_commit: String,
    pub utc_build_time: String,
    pub os: String,
    pub architecture: String,
    pub rust_version: String,
}

pub fn run() {
    let info = VersionInfo {
        name: clap::crate_name!().to_string(),
        version: clap::crate_version!().to_string(),
        git_commit: env!("BLOOM_GIT_COMMIT").to_string(),
        utc_build_time: env!("BLOOM_UTC_BUILD_TIME").to_string(),
        os: env::consts::OS.to_string(),
        architecture: env::consts::ARCH.to_string(),
        rust_version: env!("BLOOM_RUST_VERSION").to_string(),
    };

    println!("Name           : {}", info.name);
    println!("Version        : {}", info.version);
    println!("Git commit     : {}", info.git_commit);
    println!("UTC build time : {}", info.utc_build_time);
    println!("OS             : {}", info.os);
    println!("Architecture   : {}", info.architecture);
    println!("Rust Version   : {}", info.rust_version);
}
