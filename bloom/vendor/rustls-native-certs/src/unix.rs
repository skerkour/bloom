use openssl_probe;
use rustls::RootCertStore;
use std::io::{Error, ErrorKind};
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

use crate::PartialResult;

fn load_file(store: &mut RootCertStore, path: &Path) -> Result<(), Error> {
    let f = File::open(&path)?;
    let mut f = BufReader::new(f);
    if store.add_pem_file(&mut f).is_err() {
        Err(Error::new(ErrorKind::InvalidData,
                       format!("Could not load PEM file {:?}", path)))
    } else {
        Ok(())
    }
}

/// Loads root certificates found in the platform's native certificate
/// store.
///
/// On success, this returns a `rustls::RootCertStore` loaded with a
/// snapshop of the root certificates found on this platform.  This
/// function fails in a platform-specific way, expressed in a `std::io::Error`.
/// It may produce partial output.
///
/// This function can be expensive: on some platforms it involves loading
/// and parsing a ~300KB disk file.  It's therefore prudent to call
/// this sparingly.
pub fn load_native_certs() -> PartialResult<RootCertStore, Error> {
    let likely_locations = openssl_probe::probe();
    let mut store = RootCertStore::empty();
    let mut first_error = None;

    if let Some(file) = likely_locations.cert_file {
        match load_file(&mut store, &file) {
            Err(err) => {
                first_error = first_error.or_else(|| Some(err));
            }
            _ => {}
        }
    }

    if let Some(err) = first_error {
        if store.is_empty() {
            Err((None, err))
        } else {
            Err((Some(store), err))
        }
    } else {
        Ok(store)
    }
}
