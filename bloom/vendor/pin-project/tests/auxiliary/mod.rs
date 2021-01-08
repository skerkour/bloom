#![allow(dead_code, unused_macros)]
#![allow(box_pointers, unreachable_pub)]
#![allow(clippy::restriction)]

use std::{env, fs, path::Path, process::Command};
use tempfile::Builder;

macro_rules! assert_unpin {
    ($ty:ty) => {
        static_assertions::assert_impl_all!($ty: Unpin);
    };
}
macro_rules! assert_not_unpin {
    ($ty:ty) => {
        static_assertions::assert_not_impl_all!($ty: Unpin);
    };
}

#[rustversion::attr(since(1.46), track_caller)]
pub fn assert_diff(expected_path: impl AsRef<Path>, actual: impl AsRef<str>) {
    let actual = actual.as_ref();
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let expected_path = &manifest_dir.join(expected_path);
    (|| -> Result<(), Box<dyn std::error::Error>> {
        let expected = fs::read_to_string(expected_path)?;
        if expected != actual {
            if env::var_os("CI").is_some() {
                let outdir = Builder::new().prefix("assert_diff").tempdir()?;
                let actual_path = &outdir.path().join(expected_path.file_name().unwrap());
                fs::write(actual_path, actual)?;
                let status = Command::new("git")
                    .args(&["--no-pager", "diff", "--no-index", "--"])
                    .args(&[expected_path, actual_path])
                    .status()?;
                assert!(!status.success());
                panic!("assertion failed");
            } else {
                fs::write(expected_path, actual)?;
            }
        }
        Ok(())
    })()
    .unwrap_or_else(|e| panic!("{}", e))
}
