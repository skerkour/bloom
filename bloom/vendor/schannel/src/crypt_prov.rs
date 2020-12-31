//! CryptoAPI key providers.
use std::io;
use std::ptr;
use std::slice;
use winapi::shared::minwindef as winapi;
use winapi::shared::ntdef;
use winapi::um::winbase;
use winapi::um::wincrypt;

use crate::Inner;
use crate::crypt_key::CryptKey;

/// A CryptoAPI handle to a provider of a key.
pub struct CryptProv(wincrypt::HCRYPTPROV);

impl Drop for CryptProv {
    fn drop(&mut self) {
        unsafe {
            wincrypt::CryptReleaseContext(self.0, 0);
        }
    }
}

inner!(CryptProv, wincrypt::HCRYPTPROV);

impl CryptProv {
    /// Imports a key into this provider.
    pub fn import<'a>(&'a mut self) -> ImportOptions<'a> {
        ImportOptions {
            prov: self,
            flags: 0,
        }
    }
}

/// A builder for `CryptProv`s.
pub struct AcquireOptions {
    container: Option<Vec<u16>>,
    provider: Option<Vec<u16>>,
    flags: winapi::DWORD,
}

impl AcquireOptions {
    /// Returns a new builder with default settings.
    pub fn new() -> AcquireOptions {
        AcquireOptions {
            container: None,
            provider: None,
            flags: 0,
        }
    }

    /// Sets the name for this key container.
    ///
    /// This should not be set if `verify_context` is set.
    pub fn container(&mut self, container: &str) -> &mut AcquireOptions {
        self.container = Some(container.encode_utf16().chain(Some(0)).collect());
        self
    }

    /// Sets the name of the CSP to be used.
    pub fn provider(&mut self, provider: &str) -> &mut AcquireOptions {
        self.provider = Some(provider.encode_utf16().chain(Some(0)).collect());
        self
    }

    /// If set, private keys will not be accessible or persisted.
    pub fn verify_context(&mut self, verify_context: bool) -> &mut AcquireOptions {
        self.flag(wincrypt::CRYPT_VERIFYCONTEXT, verify_context)
    }

    /// If set, the container will be created.
    pub fn new_keyset(&mut self, new_keyset: bool) -> &mut AcquireOptions {
        self.flag(wincrypt::CRYPT_NEWKEYSET, new_keyset)
    }

    /// If set, the container will be stored as a machine rather than user keys.
    pub fn machine_keyset(&mut self, machine_keyset: bool) -> &mut AcquireOptions {
        self.flag(wincrypt::CRYPT_MACHINE_KEYSET, machine_keyset)
    }

    /// If set, an error will be returned if user intervention is required
    /// rather than displaying a dialog.
    pub fn silent(&mut self, silent: bool) -> &mut AcquireOptions {
        self.flag(wincrypt::CRYPT_SILENT, silent)
    }

    fn flag(&mut self, flag: winapi::DWORD, on: bool) -> &mut AcquireOptions {
        if on {
            self.flags |= flag;
        } else {
            self.flags &= !flag;
        }

        self
    }

    /// Acquires a container.
    pub fn acquire(&self, type_: ProviderType) -> io::Result<CryptProv> {
        unsafe {
            let container = self.container.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null());
            let provider = self.provider.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null());

            let mut prov = 0;
            let res = wincrypt::CryptAcquireContextW(&mut prov,
                                                     container as *mut _,
                                                     provider as *mut _,
                                                     type_.0,
                                                     self.flags);
            if res == winapi::TRUE {
                Ok(CryptProv(prov))
            } else {
                Err(io::Error::last_os_error())
            }
        }
    }
}

/// An identifier of the type of cryptography provider to be used with a
/// container.
#[derive(Copy, Clone)]
pub struct ProviderType(winapi::DWORD);

#[allow(missing_docs)]
impl ProviderType {
    pub fn rsa_full() -> ProviderType {
        ProviderType(wincrypt::PROV_RSA_FULL)
    }

    pub fn rsa_aes() -> ProviderType {
        ProviderType(wincrypt::PROV_RSA_AES)
    }

    pub fn rsa_sig() -> ProviderType {
        ProviderType(wincrypt::PROV_RSA_SIG)
    }

    pub fn rsa_schannel() -> ProviderType {
        ProviderType(wincrypt::PROV_RSA_SCHANNEL)
    }

    pub fn dss() -> ProviderType {
        ProviderType(wincrypt::PROV_DSS)
    }

    pub fn dss_dh() -> ProviderType {
        ProviderType(wincrypt::PROV_DSS_DH)
    }

    pub fn dh_schannel() -> ProviderType {
        ProviderType(wincrypt::PROV_DH_SCHANNEL)
    }

    pub fn fortezza() -> ProviderType {
        ProviderType(wincrypt::PROV_FORTEZZA)
    }

    pub fn ms_exchange() -> ProviderType {
        ProviderType(wincrypt::PROV_MS_EXCHANGE)
    }

    pub fn ssl() -> ProviderType {
        ProviderType(wincrypt::PROV_SSL)
    }

    pub fn as_raw(&self) -> winapi::DWORD {
        self.0
    }
}

/// A builder for key imports.
pub struct ImportOptions<'a> {
    prov: &'a mut CryptProv,
    flags: winapi::DWORD,
}

impl<'a> ImportOptions<'a> {
    /// Imports a DER-encoded PKCS1 private key.
    pub fn import(&mut self, der: &[u8]) -> io::Result<CryptKey> {
        unsafe {
            assert!(der.len() <= winapi::DWORD::max_value() as usize);
            let mut buf = ptr::null_mut();
            let mut len = 0;
            let res = wincrypt::CryptDecodeObjectEx(wincrypt::X509_ASN_ENCODING |
                                                    wincrypt::PKCS_7_ASN_ENCODING,
                                                    wincrypt::PKCS_RSA_PRIVATE_KEY,
                                                    der.as_ptr(),
                                                    der.len() as winapi::DWORD,
                                                    wincrypt::CRYPT_DECODE_ALLOC_FLAG,
                                                    ptr::null_mut(),
                                                    &mut buf as *mut _ as winapi::LPVOID,
                                                    &mut len);
            if res == winapi::FALSE {
                return Err(io::Error::last_os_error());
            }

            let mut key = 0;
            let res = wincrypt::CryptImportKey(self.prov.0, buf, len, 0, self.flags, &mut key);
            winbase::LocalFree(buf as *mut _);

            if res == winapi::TRUE {
                Ok(CryptKey::from_inner(key))
            } else {
                Err(io::Error::last_os_error())
            }
        }
    }

    /// Imports a DER-encoded PKCS8 private key.
    pub fn import_pkcs8(&mut self, der: &[u8]) -> io::Result<CryptKey> {
        unsafe {
            assert!(der.len() <= winapi::DWORD::max_value() as usize);

            // Decode the der format into a CRYPT_PRIVATE_KEY_INFO struct
            let mut buf = ptr::null_mut();
            let mut len = 0;
            let res = wincrypt::CryptDecodeObjectEx(wincrypt::X509_ASN_ENCODING |
                                                    wincrypt::PKCS_7_ASN_ENCODING,
                                                    wincrypt::PKCS_PRIVATE_KEY_INFO,
                                                    der.as_ptr(),
                                                    der.len() as winapi::DWORD,
                                                    wincrypt::CRYPT_DECODE_ALLOC_FLAG,
                                                    ptr::null_mut(),
                                                    &mut buf as *mut _ as winapi::LPVOID,
                                                    &mut len);
            if res == winapi::FALSE {
                return Err(io::Error::last_os_error());
            }
            let pkey: wincrypt::CRYPT_PRIVATE_KEY_INFO = *buf;
            let pkey = pkey.PrivateKey;

            let res = self.import(&slice::from_raw_parts(pkey.pbData, pkey.cbData as usize));
            winbase::LocalFree(buf as *mut _);
            res
        }
    }

    /// Imports a PEM-encoded PKCS8 private key.
    /// This functions decodes PEM blocks with or without "-----BEGIN PRIVATE KEY-----"
    /// and "-----END PRIVATE KEY-----" headers, but if PEM guards are present they must be exactly
    /// these.
    pub fn import_pkcs8_pem(&mut self, pem: &[u8]) -> io::Result<CryptKey> {
        let pem_str = std::str::from_utf8(pem)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid utf-8"))?
            .trim();

        if pem_str.starts_with("-----") {
            if !pem_str.starts_with("-----BEGIN PRIVATE KEY-----") ||
               !pem_str.ends_with("-----END PRIVATE KEY-----") {
                return Err(io::Error::new(io::ErrorKind::InvalidData,
                                          "expected '-----BEGIN PRIVATE KEY-----'\
                                          and '-----END PRIVATE KEY-----' PEM guards"));
            }
        }
        unsafe {
            assert!(pem.len() <= winapi::DWORD::max_value() as usize);

            // Decode the pem wrapper before passing it to import_pkcs8
            // Call once first to figure out the necessary buffer size
            let mut len = 0;
            let res = wincrypt::CryptStringToBinaryA(pem.as_ptr() as ntdef::LPCSTR,
                                                    pem.len() as winapi::DWORD,
                                                    wincrypt::CRYPT_STRING_BASE64_ANY,
                                                    ptr::null_mut(),
                                                    &mut len,
                                                    ptr::null_mut(),
                                                    ptr::null_mut());
            if res == winapi::FALSE {
                return Err(io::Error::last_os_error());
            }

            // Call second time to actually get the DER bytes
            let mut der_buf = vec![0; len as usize];
            let res = wincrypt::CryptStringToBinaryA(pem.as_ptr() as ntdef::LPCSTR,
                                                    pem.len() as winapi::DWORD,
                                                    wincrypt::CRYPT_STRING_BASE64_ANY,
                                                    der_buf.as_mut_ptr(),
                                                    &mut len,
                                                    ptr::null_mut(),
                                                    ptr::null_mut());
            if res == winapi::FALSE {
                return Err(io::Error::last_os_error());
            }
            self.import_pkcs8(&der_buf)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use winapi::shared::ntdef;

    #[test]
    fn rsa_key() {
        let key = include_bytes!("../test/key.key");

        let mut context = AcquireOptions::new()
            .verify_context(true)
            .acquire(ProviderType::rsa_full())
            .unwrap();
        context.import()
            .import(key)
            .unwrap();
    }

    #[test]
    fn pkcs8_key() {
        let key = include_str!("../test/key.pem");
        let der = unsafe {
            let mut len = 0;
            assert!(wincrypt::CryptStringToBinaryA(key.as_ptr() as ntdef::LPCSTR,
                                                   key.len() as winapi::DWORD,
                                                   wincrypt::CRYPT_STRING_BASE64HEADER,
                                                   ptr::null_mut(),
                                                   &mut len,
                                                   ptr::null_mut(),
                                                   ptr::null_mut()) == winapi::TRUE);
            let mut buf = vec![0; len as usize];
            assert!(wincrypt::CryptStringToBinaryA(key.as_ptr() as ntdef::LPCSTR,
                                                   key.len() as winapi::DWORD,
                                                   wincrypt::CRYPT_STRING_BASE64HEADER,
                                                   buf.as_mut_ptr(),
                                                   &mut len,
                                                   ptr::null_mut(),
                                                   ptr::null_mut()) == winapi::TRUE);
            buf
        };
        let mut context = AcquireOptions::new()
            .verify_context(true)
            .acquire(ProviderType::rsa_full())
            .unwrap();
        context.import()
            .import_pkcs8(&der)
            .unwrap();
    }

    #[test]
    // this also covers rejecting a pkcs1 key through import_pkcs8_pem
    fn pkcs8_key_reject_pkcs1() {
        let key = include_bytes!("../test/key.key");
        let mut context = AcquireOptions::new()
            .verify_context(true)
            .acquire(ProviderType::rsa_full())
            .unwrap();
        assert!(context.import()
            .import_pkcs8(&key[..])
            .is_err());
    }

    #[test]
    fn pkcs8_key_pem() {
        let key = include_bytes!("../test/key.pem");
        let mut context = AcquireOptions::new()
            .verify_context(true)
            .acquire(ProviderType::rsa_full())
            .unwrap();
        context.import()
            .import_pkcs8_pem(key)
            .unwrap();
    }

    #[test]
    fn pkcs8_key_pem_no_headers() {
        let key = include_bytes!("../test/key_no_headers.pem");
        let mut context = AcquireOptions::new()
            .verify_context(true)
            .acquire(ProviderType::rsa_full())
            .unwrap();
        context.import()
            .import_pkcs8_pem(key)
            .unwrap();
    }

    #[test]
    fn pkcs8_key_pem_no_end_header() {
        let key = include_bytes!("../test/key_no_end_header.pem");
        let mut context = AcquireOptions::new()
            .verify_context(true)
            .acquire(ProviderType::rsa_full())
            .unwrap();
        assert!(context.import()
            .import_pkcs8_pem(key)
            .is_err());
    }

    #[test]
    fn pkcs8_key_pem_wrong_header() {
        let key = include_bytes!("../test/key_wrong_header.pem");
        let mut context = AcquireOptions::new()
            .verify_context(true)
            .acquire(ProviderType::rsa_full())
            .unwrap();
        assert!(context.import()
            .import_pkcs8_pem(key)
            .is_err());
    }

    #[test]
    fn pkcs8_key_pem_invalid_header() {
        let key = include_bytes!("../test/key_invalid_header.pem");
        let mut context = AcquireOptions::new()
            .verify_context(true)
            .acquire(ProviderType::rsa_full())
            .unwrap();
        assert!(context.import()
            .import_pkcs8_pem(key)
            .is_err());
    }
}
