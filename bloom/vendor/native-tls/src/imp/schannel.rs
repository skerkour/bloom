extern crate schannel;

use self::schannel::cert_context::{CertContext, HashAlgorithm};
use self::schannel::cert_store::{CertAdd, CertStore, Memory, PfxImportOptions};
use self::schannel::schannel_cred::{Direction, Protocol, SchannelCred};
use self::schannel::tls_stream;
use std::error;
use std::fmt;
use std::io;
use std::str;

use {TlsAcceptorBuilder, TlsConnectorBuilder};

const SEC_E_NO_CREDENTIALS: u32 = 0x8009030E;

static PROTOCOLS: &'static [Protocol] = &[
    Protocol::Ssl3,
    Protocol::Tls10,
    Protocol::Tls11,
    Protocol::Tls12,
];

fn convert_protocols(min: Option<::Protocol>, max: Option<::Protocol>) -> &'static [Protocol] {
    let mut protocols = PROTOCOLS;
    if let Some(p) = max.and_then(|max| protocols.get(..max as usize)) {
        protocols = p;
    }
    if let Some(p) = min.and_then(|min| protocols.get(min as usize..)) {
        protocols = p;
    }
    protocols
}

pub struct Error(io::Error);

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        error::Error::source(&self.0)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, fmt)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, fmt)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error(error)
    }
}

#[derive(Clone)]
pub struct Identity {
    cert: CertContext,
}

impl Identity {
    pub fn from_pkcs12(buf: &[u8], pass: &str) -> Result<Identity, Error> {
        let store = PfxImportOptions::new().password(pass).import(buf)?;
        let mut identity = None;

        for cert in store.certs() {
            if cert
                .private_key()
                .silent(true)
                .compare_key(true)
                .acquire()
                .is_ok()
            {
                identity = Some(cert);
                break;
            }
        }

        let identity = match identity {
            Some(identity) => identity,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "No identity found in PKCS #12 archive",
                )
                .into());
            }
        };

        Ok(Identity { cert: identity })
    }
}

#[derive(Clone)]
pub struct Certificate(CertContext);

impl Certificate {
    pub fn from_der(buf: &[u8]) -> Result<Certificate, Error> {
        let cert = CertContext::new(buf)?;
        Ok(Certificate(cert))
    }

    pub fn from_pem(buf: &[u8]) -> Result<Certificate, Error> {
        match str::from_utf8(buf) {
            Ok(s) => {
                let cert = CertContext::from_pem(s)?;
                Ok(Certificate(cert))
            }
            Err(_) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "PEM representation contains non-UTF-8 bytes",
            )
            .into()),
        }
    }

    pub fn to_der(&self) -> Result<Vec<u8>, Error> {
        Ok(self.0.to_der().to_vec())
    }
}

pub struct MidHandshakeTlsStream<S>(tls_stream::MidHandshakeTlsStream<S>);

impl<S> fmt::Debug for MidHandshakeTlsStream<S>
where
    S: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, fmt)
    }
}

impl<S> MidHandshakeTlsStream<S> {
    pub fn get_ref(&self) -> &S {
        self.0.get_ref()
    }

    pub fn get_mut(&mut self) -> &mut S {
        self.0.get_mut()
    }
}

impl<S> MidHandshakeTlsStream<S>
where
    S: io::Read + io::Write,
{
    pub fn handshake(self) -> Result<TlsStream<S>, HandshakeError<S>> {
        match self.0.handshake() {
            Ok(s) => Ok(TlsStream(s)),
            Err(e) => Err(e.into()),
        }
    }
}

pub enum HandshakeError<S> {
    Failure(Error),
    WouldBlock(MidHandshakeTlsStream<S>),
}

impl<S> From<tls_stream::HandshakeError<S>> for HandshakeError<S> {
    fn from(e: tls_stream::HandshakeError<S>) -> HandshakeError<S> {
        match e {
            tls_stream::HandshakeError::Failure(e) => HandshakeError::Failure(e.into()),
            tls_stream::HandshakeError::Interrupted(s) => {
                HandshakeError::WouldBlock(MidHandshakeTlsStream(s))
            }
        }
    }
}

impl<S> From<io::Error> for HandshakeError<S> {
    fn from(e: io::Error) -> HandshakeError<S> {
        HandshakeError::Failure(e.into())
    }
}

#[derive(Clone, Debug)]
pub struct TlsConnector {
    cert: Option<CertContext>,
    roots: CertStore,
    min_protocol: Option<::Protocol>,
    max_protocol: Option<::Protocol>,
    use_sni: bool,
    accept_invalid_hostnames: bool,
    accept_invalid_certs: bool,
    disable_built_in_roots: bool,
    #[cfg(feature = "alpn")]
    alpn: Vec<String>,
}

impl TlsConnector {
    pub fn new(builder: &TlsConnectorBuilder) -> Result<TlsConnector, Error> {
        let cert = builder.identity.as_ref().map(|i| i.0.cert.clone());
        let mut roots = Memory::new()?.into_store();
        for cert in &builder.root_certificates {
            roots.add_cert(&(cert.0).0, CertAdd::ReplaceExisting)?;
        }

        Ok(TlsConnector {
            cert,
            roots,
            min_protocol: builder.min_protocol,
            max_protocol: builder.max_protocol,
            use_sni: builder.use_sni,
            accept_invalid_hostnames: builder.accept_invalid_hostnames,
            accept_invalid_certs: builder.accept_invalid_certs,
            disable_built_in_roots: builder.disable_built_in_roots,
            #[cfg(feature = "alpn")]
            alpn: builder.alpn.clone(),
        })
    }

    pub fn connect<S>(&self, domain: &str, stream: S) -> Result<TlsStream<S>, HandshakeError<S>>
    where
        S: io::Read + io::Write,
    {
        let mut builder = SchannelCred::builder();
        builder.enabled_protocols(convert_protocols(self.min_protocol, self.max_protocol));
        if let Some(cert) = self.cert.as_ref() {
            builder.cert(cert.clone());
        }
        let cred = builder.acquire(Direction::Outbound)?;
        let mut builder = tls_stream::Builder::new();
        builder
            .cert_store(self.roots.clone())
            .domain(domain)
            .use_sni(self.use_sni)
            .accept_invalid_hostnames(self.accept_invalid_hostnames);
        if self.accept_invalid_certs {
            builder.verify_callback(|_| Ok(()));
        } else if self.disable_built_in_roots {
            let roots_copy = self.roots.clone();
            builder.verify_callback(move |res| {
                if let Err(err) = res.result() {
                    // Propagate previous error encountered during normal cert validation.
                    return Err(err);
                }

                if let Some(chain) = res.chain() {
                    if chain
                        .certificates()
                        .any(|cert| roots_copy.certs().any(|root_cert| root_cert == cert))
                    {
                        return Ok(());
                    }
                }

                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "unable to find any user-specified roots in the final cert chain",
                ))
            });
        }
        #[cfg(feature = "alpn")]
        {
            if !self.alpn.is_empty() {
                builder.request_application_protocols(
                    &self.alpn.iter().map(|s| s.as_bytes()).collect::<Vec<_>>(),
                );
            }
        }
        match builder.connect(cred, stream) {
            Ok(s) => Ok(TlsStream(s)),
            Err(e) => Err(e.into()),
        }
    }
}

#[derive(Clone)]
pub struct TlsAcceptor {
    cert: CertContext,
    min_protocol: Option<::Protocol>,
    max_protocol: Option<::Protocol>,
}

impl TlsAcceptor {
    pub fn new(builder: &TlsAcceptorBuilder) -> Result<TlsAcceptor, Error> {
        Ok(TlsAcceptor {
            cert: builder.identity.0.cert.clone(),
            min_protocol: builder.min_protocol,
            max_protocol: builder.max_protocol,
        })
    }

    pub fn accept<S>(&self, stream: S) -> Result<TlsStream<S>, HandshakeError<S>>
    where
        S: io::Read + io::Write,
    {
        let mut builder = SchannelCred::builder();
        builder.enabled_protocols(convert_protocols(self.min_protocol, self.max_protocol));
        builder.cert(self.cert.clone());
        // FIXME we're probably missing the certificate chain?
        let cred = builder.acquire(Direction::Inbound)?;
        match tls_stream::Builder::new().accept(cred, stream) {
            Ok(s) => Ok(TlsStream(s)),
            Err(e) => Err(e.into()),
        }
    }
}

pub struct TlsStream<S>(tls_stream::TlsStream<S>);

impl<S: fmt::Debug> fmt::Debug for TlsStream<S> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, fmt)
    }
}

impl<S> TlsStream<S> {
    pub fn get_ref(&self) -> &S {
        self.0.get_ref()
    }

    pub fn get_mut(&mut self) -> &mut S {
        self.0.get_mut()
    }
}

impl<S: io::Read + io::Write> TlsStream<S> {
    pub fn buffered_read_size(&self) -> Result<usize, Error> {
        Ok(self.0.get_buf().len())
    }

    pub fn peer_certificate(&self) -> Result<Option<Certificate>, Error> {
        match self.0.peer_certificate() {
            Ok(cert) => Ok(Some(Certificate(cert))),
            Err(ref e) if e.raw_os_error() == Some(SEC_E_NO_CREDENTIALS as i32) => Ok(None),
            Err(e) => Err(Error(e)),
        }
    }

    #[cfg(feature = "alpn")]
    pub fn negotiated_alpn(&self) -> Result<Option<Vec<u8>>, Error> {
        Ok(self.0.negotiated_application_protocol()?)
    }

    pub fn tls_server_end_point(&self) -> Result<Option<Vec<u8>>, Error> {
        let cert = if self.0.is_server() {
            self.0.certificate()
        } else {
            self.0.peer_certificate()
        };

        let cert = match cert {
            Ok(cert) => cert,
            Err(ref e) if e.raw_os_error() == Some(SEC_E_NO_CREDENTIALS as i32) => return Ok(None),
            Err(e) => return Err(Error(e)),
        };

        let signature_algorithms = cert.sign_hash_algorithms()?;
        let hash = match signature_algorithms.rsplit('/').next().unwrap() {
            "MD5" | "SHA1" | "SHA256" => HashAlgorithm::sha256(),
            "SHA384" => HashAlgorithm::sha384(),
            "SHA512" => HashAlgorithm::sha512(),
            _ => return Ok(None),
        };

        let digest = cert.fingerprint(hash)?;
        Ok(Some(digest))
    }

    pub fn shutdown(&mut self) -> io::Result<()> {
        self.0.shutdown()?;
        Ok(())
    }
}

impl<S: io::Read + io::Write> io::Read for TlsStream<S> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl<S: io::Read + io::Write> io::Write for TlsStream<S> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}
