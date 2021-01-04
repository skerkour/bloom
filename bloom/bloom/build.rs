use std::process::Command;

fn main() {
    let output = Command::new("git").args(&["rev-parse", "HEAD"]).output().unwrap();
    let git_commit = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=BLOOM_GIT_COMMIT={}", git_commit);

    let build_time = stdx::chrono::Utc::now();
    println!("cargo:rustc-env=BLOOM_UTC_BUILD_TIME={}", build_time.to_rfc3339());

    let output = Command::new("rustc").args(&["--version"]).output().unwrap();
    let rust_version: Vec<String> = String::from_utf8(output.stdout)
        .unwrap()
        .split(' ')
        .map(|part| part.to_string())
        .collect();
    println!("cargo:rustc-env=BLOOM_RUST_VERSION={}", rust_version[1]);
}
