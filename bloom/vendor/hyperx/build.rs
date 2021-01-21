use std::env;
use std::process::Command;

fn main() {
    static PACKAGE: &'static str = "hyperx";
    let msrv = vec![1, 39];

    static VERSION: &'static str = env!("CARGO_PKG_VERSION");
    static M_V: &'static str = "minimum supported rust version (MSRV)";

    let rustv = rustc_version();

    if rustv < msrv {
        panic!(
            "{} v{} {} is {} > {} (this rustc)",
            PACKAGE, VERSION, M_V, join(&msrv), join(&rustv));
    }
}

fn join(ver: &Vec<u16>) -> String {
    let mut out = String::new();
    for v in ver {
        if !out.is_empty() { out.push('.'); }
        out.push_str(&v.to_string());
    }
    out
}

// Parse `rustc --version` and return as vector of integers, or panic.
fn rustc_version() -> Vec<u16> {
    let rustc = env::var("RUSTC").unwrap_or("rustc".to_owned());
    let out = Command::new(rustc).arg("--version").output().unwrap();
    let out = String::from_utf8(out.stdout).unwrap();
    for l in out.lines() {
        if l.starts_with("rustc ") {
            let mut v = &l[6..];
            if let Some(e) = v.find(" ") {
                v = &v[..e];
            }
            let mut vp = v.split("-");
            if let Some(v) = vp.next() {
                let vs: Vec<u16> = v.split(".")
                    .filter_map(|vss| vss.parse().ok())
                    .collect();
                if !vs.is_empty() {
                    return vs;
                }
            }
        }
    }
    panic!("rustc version not found")
}
