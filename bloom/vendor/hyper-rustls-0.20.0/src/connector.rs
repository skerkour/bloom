use futures_util::FutureExt;
#[cfg(feature = "tokio-runtime")]
use hyper::client::connect::HttpConnector;
use hyper::{client::connect::Connection, service::Service, Uri};
use rustls::ClientConfig;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::{fmt, io};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_rustls::TlsConnector;
use webpki::DNSNameRef;
use log::warn;

use crate::stream::MaybeHttpsStream;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

/// A Connector for the `https` scheme.
#[derive(Clone)]
pub struct HttpsConnector<T> {
    http: T,
    tls_config: Arc<ClientConfig>,
}

#[cfg(all(any(feature = "rustls-native-certs", feature = "webpki-roots"), feature = "tokio-runtime"))]
impl HttpsConnector<HttpConnector> {
    /// Construct a new `HttpsConnector`.
    ///
    /// Takes number of DNS worker threads.
    pub fn new() -> Self {
        let mut http = HttpConnector::new();
        http.enforce_http(false);
        let mut config = ClientConfig::new();
        config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
        #[cfg(feature = "rustls-native-certs")] 
        {
            config.root_store = match rustls_native_certs::load_native_certs() {
                Ok(store) => store,
                Err((Some(store), err)) => {
                    warn!("Could not load all certificates: {:?}", err);
                    store
                }
                Err((None, err)) => {
                    Err(err).expect("cannot access native cert store")
                }
            };
        }
        #[cfg(feature = "webpki-roots")] 
        {
            config
                .root_store
                .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
        }
        config.ct_logs = Some(&ct_logs::LOGS);
        (http, config).into()
    }
}

#[cfg(all(any(feature = "rustls-native-certs", feature = "webpki-roots"), feature = "tokio-runtime"))]
impl Default for HttpsConnector<HttpConnector> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> fmt::Debug for HttpsConnector<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("HttpsConnector").finish()
    }
}

impl<H, C> From<(H, C)> for HttpsConnector<H>
where
    C: Into<Arc<ClientConfig>> 
{
    fn from((http, cfg): (H, C)) -> Self {
        HttpsConnector {
            http,
            tls_config: cfg.into(),
        }
    }
}

impl<T> Service<Uri> for HttpsConnector<T>
where
    T: Service<Uri>,
    T::Response: Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    T::Future: Send + 'static,
    T::Error: Into<BoxError>,
{
    type Response = MaybeHttpsStream<T::Response>;
    type Error = BoxError;

    #[allow(clippy::type_complexity)]
    type Future =
        Pin<Box<dyn Future<Output = Result<MaybeHttpsStream<T::Response>, BoxError>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        match self.http.poll_ready(cx) {
            Poll::Ready(Ok(())) => Poll::Ready(Ok(())),
            Poll::Ready(Err(e)) => Poll::Ready(Err(e.into())),
            Poll::Pending => Poll::Pending,
        }
    }

    fn call(&mut self, dst: Uri) -> Self::Future {
        let is_https = dst.scheme_str() == Some("https");

        if !is_https {
            let connecting_future = self.http.call(dst);

            let f = async move {
                let tcp = connecting_future.await.map_err(Into::into)?;

                Ok(MaybeHttpsStream::Http(tcp))
            };
            f.boxed()
        } else {
            let cfg = self.tls_config.clone();
            let hostname = dst.host().unwrap_or_default().to_string();
            let connecting_future = self.http.call(dst);

            let f = async move {
                let tcp = connecting_future.await.map_err(Into::into)?;
                let connector = TlsConnector::from(cfg);
                let dnsname = DNSNameRef::try_from_ascii_str(&hostname)
                    .map_err(|_| io::Error::new(io::ErrorKind::Other, "invalid dnsname"))?;
                let tls = connector
                    .connect(dnsname, tcp)
                    .await
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                Ok(MaybeHttpsStream::Https(tls))
            };
            f.boxed()
        }
    }
}
