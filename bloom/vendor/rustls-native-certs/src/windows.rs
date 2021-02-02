use schannel;
use rustls::RootCertStore;
use std::io::{Error, ErrorKind};

use crate::PartialResult;

static PKIX_SERVER_AUTH: &str = "1.3.6.1.5.5.7.3.1";

fn usable_for_rustls(uses: schannel::cert_context::ValidUses) -> bool {
    match uses {
        schannel::cert_context::ValidUses::All => true,
        schannel::cert_context::ValidUses::Oids(strs) => {
            strs.iter().any(|x| x == PKIX_SERVER_AUTH)
        }
    }
}

/// Loads root certificates found in the platform's native certificate
/// store.
///
/// On success, this returns a `rustls::RootCertStore` loaded with a
/// snapshop of the root certificates found on this platform.  This
/// function fails in a platform-specific way, expressed in a `std::io::Error`.
///
/// This function can be expensive: on some platforms it involves loading
/// and parsing a ~300KB disk file.  It's therefore prudent to call
/// this sparingly.
pub fn load_native_certs() -> PartialResult<RootCertStore, Error> {
    let mut store = RootCertStore::empty();
    let mut first_error = None;

    let current_user_store = schannel::cert_store::CertStore::open_current_user("ROOT")
        .map_err(|err| (None, err))?;

    for cert in current_user_store.certs() {
        if !usable_for_rustls(cert.valid_uses().unwrap()) {
            continue;
        }

        match store.add(&rustls::Certificate(cert.to_der().to_vec())) {
            Err(err) => {
                first_error = first_error
                    .or_else(|| Some(Error::new(ErrorKind::InvalidData, err)));
            }
            _ => {}
        };
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
