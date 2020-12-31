//! Bindings to Certificate Trust Lists (CTL) in winapi.

#![allow(dead_code)]

use std::io;
use std::mem;
use std::ptr;
use winapi::shared::minwindef as winapi;
use winapi::shared::ntdef;
use winapi::um::wincrypt;

use crate::cert_context::CertContext;
use crate::Inner;

lazy_static! {
	static ref szOID_OIWSEC_sha1: Vec<u8> =
		wincrypt::szOID_OIWSEC_sha1.bytes().chain(Some(0)).collect();
}

/// Wrapped `PCCTL_CONTEXT` which represents a certificate trust list to
/// Windows.
pub struct CtlContext(wincrypt::PCCTL_CONTEXT);

unsafe impl Send for CtlContext {}
unsafe impl Sync for CtlContext {}

impl Drop for CtlContext {
    fn drop(&mut self) {
        unsafe {
            wincrypt::CertFreeCTLContext(self.0);
        }
    }
}

impl Inner<wincrypt::PCCTL_CONTEXT> for CtlContext {
    unsafe fn from_inner(t: wincrypt::PCCTL_CONTEXT) -> CtlContext {
        CtlContext(t)
    }

    fn as_inner(&self) -> wincrypt::PCCTL_CONTEXT {
        self.0
    }

    fn get_mut(&mut self) -> &mut wincrypt::PCCTL_CONTEXT {
        &mut self.0
    }
}

impl CtlContext {
    /// Returns a builder reader to create an encoded `CtlContext`.
    pub fn builder() -> Builder {
        Builder {
            certificates: vec![],
            usages: vec![],
        }
    }
}

/// Used to build an encoded `CtlContext` which can be added to a `Memory` store
/// to get back the actual `CtlContext`.
pub struct Builder {
    certificates: Vec<CertContext>,
    usages: Vec<Vec<u8>>,
}

impl Builder {
    /// Adds a certificate to be passed to `CryptMsgEncodeAndSignCTL` later on.
    pub fn certificate(&mut self, cert: CertContext) -> &mut Builder {
        self.certificates.push(cert);
        self
    }

    /// Adds a usage string to be passed in the `SubjectUsage` field to
    /// `CryptMsgEncodeAndSignCTL` later on.
    pub fn usage(&mut self, usage: &str) -> &mut Builder {
        let mut usage = usage.as_bytes().to_owned();
        usage.push(0);
        self.usages.push(usage);
        self
    }

    /// Calls `CryptMsgEncodeAndSignCTL` to encode this list of certificates
    /// into a CTL.
    ///
    /// This can later be passed to `Memory::add_encoded_ctl`.
    pub fn encode_and_sign(&self) -> io::Result<Vec<u8>> {
        unsafe {
            let encoding = wincrypt::X509_ASN_ENCODING | wincrypt::PKCS_7_ASN_ENCODING;

            let mut usages = self.usages.iter().map(|u| u.as_ptr()).collect::<Vec<_>>();
            let mut entry_data = vec![];
            let mut entries = vec![];
            for certificate in &self.certificates {
                let data = cert_entry(certificate)?;
                entries.push(*(data.as_ptr() as *const wincrypt::CTL_ENTRY));
                entry_data.push(data);
            }

            let mut ctl_info: wincrypt::CTL_INFO = mem::zeroed();
            ctl_info.dwVersion = wincrypt::CTL_V1;
            ctl_info.SubjectUsage.cUsageIdentifier = usages.len() as winapi::DWORD;
            ctl_info.SubjectUsage.rgpszUsageIdentifier = usages.as_mut_ptr() as *mut ntdef::LPSTR;
            ctl_info.SubjectAlgorithm.pszObjId = szOID_OIWSEC_sha1.as_ptr() as ntdef::LPSTR;
            ctl_info.cCTLEntry = entries.len() as winapi::DWORD;
            ctl_info.rgCTLEntry = entries.as_mut_ptr();

            let mut sign_info: wincrypt::CMSG_SIGNED_ENCODE_INFO = mem::zeroed();
            sign_info.cbSize = mem::size_of_val(&sign_info) as winapi::DWORD;
            let mut encoded_certs = self.certificates
                .iter()
                .map(|c| {
                    wincrypt::CERT_BLOB {
                        cbData: (*c.as_inner()).cbCertEncoded,
                        pbData: (*c.as_inner()).pbCertEncoded,
                    }
                })
                .collect::<Vec<_>>();
            sign_info.rgCertEncoded = encoded_certs.as_mut_ptr();
            sign_info.cCertEncoded = encoded_certs.len() as winapi::DWORD;

            let flags = wincrypt::CMSG_ENCODE_SORTED_CTL_FLAG |
                        wincrypt::CMSG_ENCODE_HASHED_SUBJECT_IDENTIFIER_FLAG;

            let mut size = 0;

            let res = wincrypt::CryptMsgEncodeAndSignCTL(encoding,
                                                         &mut ctl_info,
                                                         &mut sign_info,
                                                         flags,
                                                         ptr::null_mut(),
                                                         &mut size);
            if res == winapi::FALSE {
                return Err(io::Error::last_os_error())
            }

            let mut encoded = vec![0; size as usize];

            let res = wincrypt::CryptMsgEncodeAndSignCTL(encoding,
                                                         &mut ctl_info,
                                                         &mut sign_info,
                                                         flags,
                                                         encoded.as_mut_ptr() as *mut winapi::BYTE,
                                                         &mut size);
            if res == winapi::FALSE {
                return Err(io::Error::last_os_error())
            }

            Ok(encoded)
        }
    }
}

fn cert_entry(cert: &CertContext) -> io::Result<Vec<u8>> {
    // FIXME: Seems to be missing since the winapi 0.3 upgrade?
    const CTL_ENTRY_FROM_PROP_CHAIN_FLAG: winapi::DWORD = 1;

    unsafe {
        let mut size = 0;

        let res = wincrypt::CertCreateCTLEntryFromCertificateContextProperties(
			cert.as_inner(),
			0,
			ptr::null_mut(),
			CTL_ENTRY_FROM_PROP_CHAIN_FLAG,
			ptr::null_mut(),
			ptr::null_mut(),
			&mut size);
        if res == winapi::FALSE {
            return Err(io::Error::last_os_error());
        }

        let mut entry = vec![0u8; size as usize];
        let res = wincrypt::CertCreateCTLEntryFromCertificateContextProperties(
			cert.as_inner(),
			0,
			ptr::null_mut(),
			CTL_ENTRY_FROM_PROP_CHAIN_FLAG,
			ptr::null_mut(),
			entry.as_mut_ptr() as wincrypt::PCTL_ENTRY,
			&mut size);
        if res == winapi::FALSE {
            Err(io::Error::last_os_error())
        } else {
            Ok(entry)
        }
    }
}
