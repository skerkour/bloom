extern crate tempdir;

use std::error::Error;
use std::fs::{File, create_dir_all};
use std::path::PathBuf;
use tempdir::TempDir;

fn create_files(files: &[&str]) -> Result<TempDir, Box<Error>> {
    let tmp_dir = TempDir::new("globwalk")?;

    for f in files {
        let file_path = PathBuf::from(f);
        if let Some(dir) = file_path.parent() {
            create_dir_all(tmp_dir.path().join(dir))?;
        }
        File::create(tmp_dir.path().join(file_path))?;
    }

    Ok(tmp_dir)
}
