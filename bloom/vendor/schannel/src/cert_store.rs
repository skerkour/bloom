//! Bindings to winapi's certificate-store related APIs.

use std::cmp;
use std::ffi::OsStr;
use std::fmt;
use std::io;
use std::mem;
use std::os::windows::prelude::*;
use std::ptr;
use winapi::shared::minwindef as winapi;
use winapi::shared::ntdef;
use winapi::um::wincrypt;

use crate::cert_context::CertContext;
use crate::ctl_context::CtlContext;

use crate::Inner;

/// Representation of certificate store on Windows, wrapping a `HCERTSTORE`.
pub struct CertStore(wincrypt::HCERTSTORE);

unsafe impl Sync for CertStore {}
unsafe impl Send for CertStore {}

impl fmt::Debug for CertStore {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		fmt.debug_struct("CertStore").finish()
	}
}

impl Drop for CertStore {
    fn drop(&mut self) {
        unsafe {
            wincrypt::CertCloseStore(self.0, 0);
        }
    }
}

impl Clone for CertStore {
    fn clone(&self) -> CertStore {
        unsafe { CertStore(wincrypt::CertDuplicateStore(self.0)) }
    }
}

inner!(CertStore, wincrypt::HCERTSTORE);

/// Argument to the `add_cert` function indicating how a certificate should be
/// added to a `CertStore`.
pub enum CertAdd {
    /// The function makes no check for an existing matching certificate or link
    /// to a matching certificate. A new certificate is always added to the
    /// store. This can lead to duplicates in a store.
    Always = wincrypt::CERT_STORE_ADD_ALWAYS as isize,

    /// If a matching certificate or a link to a matching certificate exists,
    /// the operation fails.
    New = wincrypt::CERT_STORE_ADD_NEW as isize,

    /// If a matching certificate or a link to a matching certificate exists and
    /// the NotBefore time of the existing context is equal to or greater than
    /// the NotBefore time of the new context being added, the operation fails.
    ///
    /// If the NotBefore time of the existing context is less than the NotBefore
    /// time of the new context being added, the existing certificate or link is
    /// deleted and a new certificate is created and added to the store. If a
    /// matching certificate or a link to a matching certificate does not exist,
    /// a new link is added.
    Newer = wincrypt::CERT_STORE_ADD_NEWER as isize,

    /// If a matching certificate or a link to a matching certificate exists and
    /// the NotBefore time of the existing context is equal to or greater than
    /// the NotBefore time of the new context being added, the operation fails.
    ///
    /// If the NotBefore time of the existing context is less than the NotBefore
    /// time of the new context being added, the existing context is deleted
    /// before creating and adding the new context. The new added context
    /// inherits properties from the existing certificate.
    NewerInheritProperties = wincrypt::CERT_STORE_ADD_NEWER_INHERIT_PROPERTIES as isize,

    /// If a link to a matching certificate exists, that existing certificate or
    /// link is deleted and a new certificate is created and added to the store.
    /// If a matching certificate or a link to a matching certificate does not
    /// exist, a new link is added.
    ReplaceExisting = wincrypt::CERT_STORE_ADD_REPLACE_EXISTING as isize,

    /// If a matching certificate exists in the store, the existing context is
    /// not replaced. The existing context inherits properties from the new
    /// certificate.
    ReplaceExistingInheritProperties =
        wincrypt::CERT_STORE_ADD_REPLACE_EXISTING_INHERIT_PROPERTIES as isize,

    /// If a matching certificate or a link to a matching certificate exists,
    /// that existing certificate or link is used and properties from the
    /// new certificate are added. The function does not fail, but it does
    /// not add a new context. The existing context is duplicated and returned.
    ///
    /// If a matching certificate or a link to a matching certificate does
    /// not exist, a new certificate is added.
    UseExisting = wincrypt::CERT_STORE_ADD_USE_EXISTING as isize,
}

impl CertStore {
    /// Opens up the specified key store within the context of the current user.
    ///
    /// Common valid values for `which` are "My", "Root", "Trust", "CA".
    /// Additonal MSDN docs https://docs.microsoft.com/en-us/windows/win32/api/wincrypt/nf-wincrypt-certopenstore#remarks
    pub fn open_current_user(which: &str) -> io::Result<CertStore> {
        unsafe {
            let data = OsStr::new(which)
                             .encode_wide()
                             .chain(Some(0))
                             .collect::<Vec<_>>();
            let store = wincrypt::CertOpenStore(wincrypt::CERT_STORE_PROV_SYSTEM_W as ntdef::LPCSTR,
                                                0,
                                                0,
                                                wincrypt::CERT_SYSTEM_STORE_CURRENT_USER,
                                                data.as_ptr() as *mut _);
            if store.is_null() {
                Err(io::Error::last_os_error())
            } else {
                Ok(CertStore(store))
            }
        }
    }

    /// Opens up the specified key store within the context of the local machine.
    ///
    /// Common valid values for `which` are "My", "Root", "Trust", "CA".
    /// Additonal MSDN docs https://docs.microsoft.com/en-us/windows/win32/api/wincrypt/nf-wincrypt-certopenstore#remarks
    pub fn open_local_machine(which: &str) -> io::Result<CertStore> {
        unsafe {
            let data = OsStr::new(which)
                             .encode_wide()
                             .chain(Some(0))
                             .collect::<Vec<_>>();
            let store = wincrypt::CertOpenStore(wincrypt::CERT_STORE_PROV_SYSTEM_W as ntdef::LPCSTR,
                                                0,
                                                0,
                                                wincrypt::CERT_SYSTEM_STORE_LOCAL_MACHINE,
                                                data.as_ptr() as *mut _);
            if store.is_null() {
                Err(io::Error::last_os_error())
            } else {
                Ok(CertStore(store))
            }
        }
    }

    /// Imports a PKCS#12-encoded key/certificate pair, returned as a
    /// `CertStore` instance.
    ///
    /// The password must also be provided to decrypt the encoded data.
    pub fn import_pkcs12(data: &[u8],
                         password: Option<&str>)
                         -> io::Result<CertStore> {
        unsafe {
            let mut blob = wincrypt::CRYPT_INTEGER_BLOB {
                cbData: data.len() as winapi::DWORD,
                pbData: data.as_ptr() as *mut u8,
            };
            let password = password.map(|s| {
                OsStr::new(s).encode_wide()
                             .chain(Some(0))
                             .collect::<Vec<_>>()
            });
            let password = password.as_ref().map(|s| s.as_ptr());
            let password = password.unwrap_or(ptr::null());
            let res = wincrypt::PFXImportCertStore(&mut blob,
                                                   password,
                                                   0);
            if res.is_null() {
                Err(io::Error::last_os_error())
            } else {
                Ok(CertStore(res))
            }
        }
    }

    /// Returns an iterator over the certificates in this certificate store.
    pub fn certs(&self) -> Certs {
        Certs { store: self, cur: None }
    }

    /// Adds a certificate context to this store.
    ///
    /// This function will add the certificate specified in `cx` to this store.
    /// A copy of the added certificate is returned.
    pub fn add_cert(&mut self,
                    cx: &CertContext,
                    how: CertAdd) -> io::Result<CertContext> {
        unsafe {
            let how = how as winapi::DWORD;
            let mut ret = ptr::null();
            let res = wincrypt::CertAddCertificateContextToStore(self.0,
                                                                 cx.as_inner(),
                                                                 how,
                                                                 &mut ret);
            if res != winapi::TRUE {
                Err(io::Error::last_os_error())
            } else {
                Ok(CertContext::from_inner(ret))
            }
        }
    }

    /// Exports this certificate store as a PKCS#12-encoded blob.
    ///
    /// The password specified will be the password used to unlock the returned
    /// data.
    pub fn export_pkcs12(&self, password: &str) -> io::Result<Vec<u8>> {
        unsafe {
            let password = password.encode_utf16().chain(Some(0)).collect::<Vec<_>>();
            let mut blob = wincrypt::CRYPT_DATA_BLOB {
                cbData: 0,
                pbData: 0 as *mut _,
            };
            let res = wincrypt::PFXExportCertStore(self.0,
                                                   &mut blob,
                                                   password.as_ptr(),
                                                   wincrypt::EXPORT_PRIVATE_KEYS);
            if res != winapi::TRUE {
                return Err(io::Error::last_os_error())
            }
            let mut ret = Vec::with_capacity(blob.cbData as usize);
            blob.pbData = ret.as_mut_ptr();
            let res = wincrypt::PFXExportCertStore(self.0,
                                                   &mut blob,
                                                   password.as_ptr(),
                                                   wincrypt::EXPORT_PRIVATE_KEYS);
            if res != winapi::TRUE {
                return Err(io::Error::last_os_error())
            }
            ret.set_len(blob.cbData as usize);
            Ok(ret)
        }
    }
}

/// An iterator over the certificates contained in a `CertStore`, returned by
/// `CertStore::iter`
pub struct Certs<'a> {
    store: &'a CertStore,
    cur: Option<CertContext>,
}

impl<'a> Iterator for Certs<'a> {
    type Item = CertContext;

    fn next(&mut self) -> Option<CertContext> {
        unsafe {
            let cur = self.cur.take().map(|p| {
                let ptr = p.as_inner();
                mem::forget(p);
                ptr
            });
            let cur = cur.unwrap_or(ptr::null_mut());
            let next = wincrypt::CertEnumCertificatesInStore(self.store.0, cur);

            if next.is_null() {
                self.cur = None;
                None
            } else {
                let next = CertContext::from_inner(next);
                self.cur = Some(next.clone());
                Some(next)
            }
        }
    }
}

/// A builder type for imports of PKCS #12 archives.
#[derive(Default)]
pub struct PfxImportOptions {
    password: Option<Vec<u16>>,
    flags: winapi::DWORD,
}

impl PfxImportOptions {
    /// Returns a new `PfxImportOptions` with default settings.
    pub fn new() -> PfxImportOptions {
        PfxImportOptions::default()
    }

    /// Sets the password to be used to decrypt the archive.
    pub fn password(&mut self, password: &str) -> &mut PfxImportOptions {
        self.password = Some(password.encode_utf16().chain(Some(0)).collect());
        self
    }

    /// If set, the private key in the archive will not be persisted.
    ///
    /// If not set, private keys are persisted on disk and must be manually deleted.
    pub fn no_persist_key(&mut self, no_persist_key: bool) -> &mut PfxImportOptions {
        self.flag(wincrypt::PKCS12_NO_PERSIST_KEY, no_persist_key)
    }

    /// If set, all extended properties of the certificate will be imported.
    pub fn include_extended_properties(&mut self,
                                       include_extended_properties: bool)
                                       -> &mut PfxImportOptions {
        self.flag(wincrypt::PKCS12_INCLUDE_EXTENDED_PROPERTIES, include_extended_properties)
    }

    fn flag(&mut self, flag: winapi::DWORD, set: bool) -> &mut PfxImportOptions {
        if set {
            self.flags |= flag;
        } else {
            self.flags &= !flag;
        }
        self
    }

    /// Imports certificates from a PKCS #12 archive, returning a `CertStore` containing them.
    pub fn import(&self, data: &[u8]) -> io::Result<CertStore> {
        unsafe {
            let mut blob = wincrypt::CRYPT_DATA_BLOB {
                cbData: cmp::min(data.len(), winapi::DWORD::max_value() as usize) as winapi::DWORD,
                pbData: data.as_ptr() as *const _ as *mut _,
            };
            let password = self.password.as_ref().map_or(ptr::null(), |p| p.as_ptr());

            let store = wincrypt::PFXImportCertStore(&mut blob, password, self.flags);
            if store.is_null() {
                return Err(io::Error::last_os_error());
            }
            Ok(CertStore(store))
        }
    }
}

/// Representation of an in-memory certificate store.
///
/// Internally this contains a `CertStore` which this type can be converted to.
pub struct Memory(CertStore);

impl Memory {
    /// Creates a new in-memory certificate store which certificates and CTLs
    /// can be added to.
    ///
    /// Initially the returned certificate store contains no certificates.
    pub fn new() -> io::Result<Memory> {
        unsafe {
            let store = wincrypt::CertOpenStore(wincrypt::CERT_STORE_PROV_MEMORY as ntdef::LPCSTR,
                                                0,
                                                0,
                                                0,
                                                ptr::null_mut());
            if store.is_null() {
                Err(io::Error::last_os_error())
            } else {
                Ok(Memory(CertStore(store)))
            }
        }
    }

    /// Adds a new certificate to this memory store.
    ///
    /// For example the bytes could be a DER-encoded certificate.
    pub fn add_encoded_certificate(&mut self, cert: &[u8]) -> io::Result<CertContext> {
        unsafe {
            let mut cert_context = ptr::null();

            let res = wincrypt::CertAddEncodedCertificateToStore((self.0).0,
                                                                 wincrypt::X509_ASN_ENCODING |
                                                                 wincrypt::PKCS_7_ASN_ENCODING,
                                                                 cert.as_ptr() as *const _,
                                                                 cert.len() as winapi::DWORD,
                                                                 wincrypt::CERT_STORE_ADD_ALWAYS,
                                                                 &mut cert_context);
            if res == winapi::TRUE {
                Ok(CertContext::from_inner(cert_context))
            } else {
                Err(io::Error::last_os_error())
            }
        }
    }

    /// Adds a new CTL to this memory store, in its encoded form.
    ///
    /// This can be created through the `ctl_context::Builder` type.
    pub fn add_encoded_ctl(&mut self, ctl: &[u8]) -> io::Result<CtlContext> {
        unsafe {
            let mut ctl_context = ptr::null();

            let res = wincrypt::CertAddEncodedCTLToStore((self.0).0,
                                                         wincrypt::X509_ASN_ENCODING |
                                                         wincrypt::PKCS_7_ASN_ENCODING,
                                                         ctl.as_ptr() as *const _,
                                                         ctl.len() as winapi::DWORD,
                                                         wincrypt::CERT_STORE_ADD_ALWAYS,
                                                         &mut ctl_context);
            if res == winapi::TRUE {
                Ok(CtlContext::from_inner(ctl_context))
            } else {
                Err(io::Error::last_os_error())
            }
        }
    }

    /// Consumes this memory store, returning the underlying `CertStore`.
    pub fn into_store(self) -> CertStore {
        self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ctl_context::CtlContext;

    #[test]
    fn load() {
        let cert = include_bytes!("../test/cert.der");
        let mut store = Memory::new().unwrap();
        store.add_encoded_certificate(cert).unwrap();
    }

    #[test]
    fn create_ctl() {
        let cert = include_bytes!("../test/self-signed.badssl.com.cer");
        let mut store = Memory::new().unwrap();
        let cert = store.add_encoded_certificate(cert).unwrap();

        CtlContext::builder()
            .certificate(cert)
            .usage("1.3.6.1.4.1.311.2.2.2")
            .encode_and_sign()
            .unwrap();
    }

    #[test]
    fn pfx_import() {
        let pfx = include_bytes!("../test/identity.p12");
        let store = PfxImportOptions::new()
                        .include_extended_properties(true)
                        .password("mypass")
                        .import(pfx)
                        .unwrap();
        assert_eq!(store.certs().count(), 2);
        let pkeys = store.certs()
                         .filter(|c| {
                             c.private_key().compare_key(true).silent(true).acquire().is_ok()
                         })
                         .count();
        assert_eq!(pkeys, 1);
    }
}
