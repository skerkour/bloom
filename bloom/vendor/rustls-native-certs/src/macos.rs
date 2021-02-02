use security_framework::trust_settings::{
    Domain,
    TrustSettings,
    TrustSettingsForCertificate
};
use rustls::RootCertStore;
use std::io::{Error, ErrorKind};
use std::collections::HashMap;

use crate::PartialResult;

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

    // The various domains are designed to interact like this:
    //
    // "Per-user Trust Settings override locally administered
    //  Trust Settings, which in turn override the System Trust
    //  Settings."
    //
    // So we collect the certificates in this order; as a map of
    // their DER encoding to what we'll do with them.  We don't
    // overwrite existing elements, which mean User settings
    // trump Admin trump System, as desired.

    let mut all_certs = HashMap::new();

    for domain in &[Domain::User, Domain::Admin, Domain::System] {
        let ts = TrustSettings::new(*domain);
        let iter = ts.iter()
            .map_err(|err| (None, Error::new(ErrorKind::Other, err)))?;

        for cert in iter {
            let der = cert.to_der();

            // If there are no specific trust settings, the default
            // is to trust the certificate as a root cert.  Weird API but OK.
            // The docs say:
            //
            // "Note that an empty Trust Settings array means "always trust this cert,
            //  with a resulting kSecTrustSettingsResult of kSecTrustSettingsResultTrustRoot".
            let trusted = ts.tls_trust_settings_for_certificate(&cert)
                .map_err(|err| (None, Error::new(ErrorKind::Other, err)))?
                .unwrap_or(TrustSettingsForCertificate::TrustRoot);

            all_certs.entry(der)
                .or_insert(trusted);
        }
    }

    let mut first_error = None;

    // Now we have all the certificates and an idea of whether
    // to use them.
    for (der, trusted) in all_certs.drain() {
        match trusted {
            TrustSettingsForCertificate::TrustRoot |
                TrustSettingsForCertificate::TrustAsRoot => {
                match store.add(&rustls::Certificate(der)) {
                    Err(err) => {
                        first_error = first_error
                            .or_else(|| Some(Error::new(ErrorKind::InvalidData, err)));
                    }
                    _ => {}
                };
            },
            _ => {} // discard
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
