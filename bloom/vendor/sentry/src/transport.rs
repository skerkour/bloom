#![allow(unused_imports)]

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, SystemTime};

#[cfg(any(feature = "reqwest", feature = "curl", feature = "surf"))]
use httpdate::parse_http_date;

#[cfg(feature = "reqwest")]
use reqwest_::{blocking::Client as ReqwestClient, header::RETRY_AFTER, Proxy};

#[cfg(feature = "curl")]
use crate::types::Scheme;
#[cfg(feature = "curl")]
use curl_ as curl;
#[cfg(feature = "curl")]
use std::io::{Cursor, Read};

#[cfg(feature = "surf")]
use futures::executor;
#[cfg(feature = "surf")]
use surf_::Client as SurfClient;

use sentry_core::sentry_debug;

use crate::protocol::Event;
use crate::{ClientOptions, Envelope, Transport, TransportFactory};

/// Creates the default HTTP transport.
///
/// This is the default value for `transport` on the client options.  It
/// creates a `HttpTransport`.  If no http transport was compiled into the
/// library it will panic on transport creation.
#[derive(Clone)]
pub struct DefaultTransportFactory;

impl TransportFactory for DefaultTransportFactory {
    fn create_transport(&self, options: &ClientOptions) -> Arc<dyn Transport> {
        #[cfg(any(feature = "reqwest", feature = "curl", feature = "surf"))]
        {
            Arc::new(HttpTransport::new(options))
        }
        #[cfg(not(any(feature = "reqwest", feature = "curl", feature = "surf")))]
        {
            let _ = options;
            panic!("sentry crate was compiled without transport")
        }
    }
}

#[cfg(any(feature = "reqwest", feature = "curl", feature = "surf"))]
fn parse_retry_after(s: &str) -> Option<SystemTime> {
    if let Ok(value) = s.parse::<f64>() {
        Some(SystemTime::now() + Duration::from_secs(value.ceil() as u64))
    } else if let Ok(value) = parse_http_date(s) {
        Some(value)
    } else {
        None
    }
}

#[allow(unused)]
macro_rules! implement_http_transport {
    (
        $(#[$attr:meta])*
        pub struct $typename:ident;
        fn spawn($($argname:ident: $argty:ty,)*) $body:block
        fn http_client($hc_options:ident: &ClientOptions, $hc_client:ident: Option<$hc_client_ty:ty>) -> $hc_ret:ty $hc_body:block
    ) => {
        $(#[$attr])*
        pub struct $typename {
            sender: Mutex<SyncSender<Option<Envelope>>>,
            shutdown_signal: Arc<Condvar>,
            shutdown_immediately: Arc<AtomicBool>,
            queue_size: Arc<Mutex<usize>>,
            handle: Option<JoinHandle<()>>,
        }

        impl $typename {
            /// Creates a new transport.
            pub fn new(options: &ClientOptions) -> Self {
                Self::new_internal(options, None)
            }

            /// Creates a new transport that uses the passed HTTP client.
            pub fn with_client(options: &ClientOptions, $hc_client: $hc_client_ty) -> Self {
                Self::new_internal(options, Some($hc_client))
            }

            /// Creates a new transport that uses the passed HTTP client or builds a new one.
            fn new_internal(options: &ClientOptions, $hc_client: Option<$hc_client_ty>) -> Self {
                fn spawn($($argname: $argty,)*) -> JoinHandle<()> { $body }

                fn http_client($hc_options: &ClientOptions, $hc_client: Option<$hc_client_ty>) -> $hc_ret { $hc_body }

                let (sender, receiver) = sync_channel(30);
                let shutdown_signal = Arc::new(Condvar::new());
                let shutdown_immediately = Arc::new(AtomicBool::new(false));
                #[allow(clippy::mutex_atomic)]
                let queue_size = Arc::new(Mutex::new(0));
                let http_client = http_client(options, $hc_client);
                let handle = Some(spawn(
                    options,
                    receiver,
                    shutdown_signal.clone(),
                    shutdown_immediately.clone(),
                    queue_size.clone(),
                    http_client,
                ));
                $typename {
                    sender: Mutex::new(sender),
                    shutdown_signal,
                    shutdown_immediately,
                    queue_size,
                    handle,
                }
            }
        }

        impl Transport for $typename {
            fn send_envelope(&self, envelope: Envelope) {
                // we count up before we put the item on the queue and in case the
                // queue is filled with too many items or we shut down, we decrement
                // the count again as there is nobody that can pick it up.
                *self.queue_size.lock().unwrap() += 1;
                if self.sender.lock().unwrap().try_send(Some(envelope)).is_err() {
                    *self.queue_size.lock().unwrap() -= 1;
                }
            }

            fn shutdown(&self, timeout: Duration) -> bool {
                sentry_debug!("shutting down http transport");
                if *self.queue_size.lock().unwrap() == 0 {
                    true
                } else {
                    if let Ok(sender) = self.sender.lock() {
                        sender.send(None).ok();
                    }
                    let guard = self.queue_size.lock().unwrap();
                    if *guard > 0 {
                        self.shutdown_signal.wait_timeout(guard, timeout).is_ok()
                    } else {
                        true
                    }
                }
            }
        }

        impl Drop for $typename {
            fn drop(&mut self) {
                sentry_debug!("dropping http transport");
                self.shutdown_immediately.store(true, Ordering::SeqCst);
                if let Ok(sender) = self.sender.lock() {
                    if sender.send(None).is_ok() {
                        if let Some(handle) = self.handle.take() {
                            handle.join().ok();
                        }
                    }
                }
            }
        }
    }
}

#[cfg(feature = "reqwest")]
implement_http_transport! {
    /// A transport can send events via HTTP to sentry via `reqwest`.
    ///
    /// When the `transport` feature is enabled this will currently
    /// be the default transport.  This is separately enabled by the
    /// `reqwest` flag.
    pub struct ReqwestHttpTransport;

    fn spawn(
        options: &ClientOptions,
        receiver: Receiver<Option<Envelope>>,
        signal: Arc<Condvar>,
        shutdown_immediately: Arc<AtomicBool>,
        queue_size: Arc<Mutex<usize>>,
        http_client: Option<ReqwestClient>,
    ) {
        let dsn = options.dsn.clone().unwrap();
        let user_agent = options.user_agent.to_string();

        let mut disabled = None::<SystemTime>;
        let http_proxy = options.http_proxy.as_ref().map(ToString::to_string);
        let https_proxy = options.https_proxy.as_ref().map(ToString::to_string);

        thread::Builder::new()
            .name("sentry-transport".to_string())
            .spawn(move || {
                sentry_debug!("spawning reqwest transport");
                let http_client = http_client.unwrap_or_else(|| {
                    let mut builder = ReqwestClient::builder();
                    if let Some(url) = http_proxy {
                        builder = builder.proxy(Proxy::http(&url).unwrap());
                    };
                    if let Some(url) = https_proxy {
                        builder = builder.proxy(Proxy::https(&url).unwrap());
                    };
                    builder.build().unwrap()
                });

                let url = dsn.envelope_api_url().to_string();

                while let Some(envelope) = receiver.recv().unwrap_or(None) {
                    // on drop we want to not continue processing the queue.
                    if shutdown_immediately.load(Ordering::SeqCst) {
                        let mut size = queue_size.lock().unwrap();
                        *size = 0;
                        signal.notify_all();
                        break;
                    }

                    // while we are disabled due to rate limits, skip
                    if let Some(ts) = disabled {
                        if let Ok(time_left) = ts.duration_since(SystemTime::now()) {
                            sentry_debug!(
                                "Skipping event send because we're disabled due to rate limits for {}s",
                                time_left.as_secs()
                            );
                            continue;
                        } else {
                            disabled = None;
                        }
                    }

                    let mut body = Vec::new();
                    envelope.to_writer(&mut body).unwrap();

                    sentry_debug!("Sending envelope");
                    match http_client
                        .post(url.as_str())
                        .body(body)
                        .header("X-Sentry-Auth", dsn.to_auth(Some(&user_agent)).to_string())
                        .send()
                    {
                        Ok(resp) => {
                            if resp.status() == 429 {
                                if let Some(retry_after) = resp
                                    .headers()
                                    .get(RETRY_AFTER)
                                    .and_then(|x| x.to_str().ok())
                                    .and_then(parse_retry_after)
                                {
                                    disabled = Some(retry_after);
                                }
                            }
                            match resp.text() {
                                Err(err) => { sentry_debug!("Failed to read sentry response: {}", err); },
                                Ok(text) => { sentry_debug!("Get response: `{}`", text); },
                            }
                        }
                        Err(err) => {
                            sentry_debug!("Failed to send envelope: {}", err);
                        }
                    }

                    let mut size = queue_size.lock().unwrap();
                    *size -= 1;
                    if *size == 0 {
                        signal.notify_all();
                    }
                }
            }).unwrap()
    }

    fn http_client(
        _options: &ClientOptions,
        client: Option<ReqwestClient>
    ) -> Option<ReqwestClient> {
        client
    }
}

#[cfg(feature = "curl")]
implement_http_transport! {
    /// A transport can send events via HTTP to sentry via `curl`.
    ///
    /// This is enabled by the `curl` flag.
    pub struct CurlHttpTransport;

    fn spawn(
        options: &ClientOptions,
        receiver: Receiver<Option<Envelope>>,
        signal: Arc<Condvar>,
        shutdown_immediately: Arc<AtomicBool>,
        queue_size: Arc<Mutex<usize>>,
        http_client: curl::easy::Easy,
    ) {
        let dsn = options.dsn.clone().unwrap();
        let user_agent = options.user_agent.to_string();
        let http_proxy = options.http_proxy.as_ref().map(ToString::to_string);
        let https_proxy = options.https_proxy.as_ref().map(ToString::to_string);

        let mut disabled = None::<SystemTime>;
        let mut handle = http_client;

        thread::spawn(move || {
            sentry_debug!("spawning curl transport");
            let url = dsn.envelope_api_url().to_string();

            while let Some(envelope) = receiver.recv().unwrap_or(None) {
                // on drop we want to not continue processing the queue.
                if shutdown_immediately.load(Ordering::SeqCst) {
                    let mut size = queue_size.lock().unwrap();
                    *size = 0;
                    signal.notify_all();
                    break;
                }

                // while we are disabled due to rate limits, skip
                if let Some(ts) = disabled {
                    if let Ok(time_left) = ts.duration_since(SystemTime::now()) {
                        sentry_debug!(
                            "Skipping event send because we're disabled due to rate limits for {}s",
                            time_left.as_secs()
                        );
                        continue;
                    } else {
                        disabled = None;
                    }
                }

                handle.reset();
                handle.url(&url).unwrap();
                handle.custom_request("POST").unwrap();

                match (dsn.scheme(), &http_proxy, &https_proxy) {
                    (Scheme::Https, _, &Some(ref proxy)) => {
                        handle.proxy(&proxy).unwrap();
                    }
                    (_, &Some(ref proxy), _) => {
                        handle.proxy(&proxy).unwrap();
                    }
                    _ => {}
                }

                let mut body = Vec::new();
                envelope.to_writer(&mut body).unwrap();
                let mut body = Cursor::new(body);

                let mut retry_after = None;
                let mut headers = curl::easy::List::new();
                headers.append(&format!("X-Sentry-Auth: {}", dsn.to_auth(Some(&user_agent)))).unwrap();
                headers.append("Expect:").unwrap();
                headers.append("Content-Type: application/json").unwrap();
                handle.http_headers(headers).unwrap();
                handle.upload(true).unwrap();
                handle.in_filesize(body.get_ref().len() as u64).unwrap();
                handle.read_function(move |buf| Ok(body.read(buf).unwrap_or(0))).unwrap();
                handle.verbose(true).unwrap();
                handle.debug_function(move |info, data| {
                    let prefix = match info {
                        curl::easy::InfoType::HeaderIn => "< ",
                        curl::easy::InfoType::HeaderOut => "> ",
                        curl::easy::InfoType::DataOut => "",
                        _ => return
                    };
                    sentry_debug!("curl: {}{}", prefix, String::from_utf8_lossy(data).trim());
                }).unwrap();

                {
                    let mut handle = handle.transfer();
                    let retry_after_setter = &mut retry_after;
                    handle.header_function(move |data| {
                        if let Ok(data) = std::str::from_utf8(data) {
                            let mut iter = data.split(':');
                            if let Some(key) = iter.next().map(str::to_lowercase) {
                                if key == "retry-after" {
                                    *retry_after_setter = iter.next().map(|x| x.trim().to_string());
                                }
                            }
                        }
                        true
                    }).unwrap();
                    handle.perform().ok();
                }

                match handle.response_code() {
                    Ok(429) => {
                        if let Some(retry_after) = retry_after
                            .as_deref()
                            .and_then(parse_retry_after)
                        {
                            disabled = Some(retry_after);
                        }
                    }
                    Ok(200) | Ok(201) => {}
                    _ => {
                        sentry_debug!("Failed to send event");
                    }
                }

                let mut size = queue_size.lock().unwrap();
                *size -= 1;
                if *size == 0 {
                    signal.notify_all();
                }
            }
        })
    }

    fn http_client(_options: &ClientOptions, client: Option<curl::easy::Easy>) -> curl::easy::Easy {
        client.unwrap_or_else(curl::easy::Easy::new)
    }
}

#[cfg(feature = "surf")]
implement_http_transport! {
    /// A transport can send events via HTTP to sentry via `surf`.
    ///
    /// This is enabled by the `surf` flag.
    pub struct SurfHttpTransport;

    fn spawn(
        options: &ClientOptions,
        receiver: Receiver<Option<Envelope>>,
        signal: Arc<Condvar>,
        shutdown_immediately: Arc<AtomicBool>,
        queue_size: Arc<Mutex<usize>>,
        http_client: SurfClient,
    ) {
        let dsn = options.dsn.clone().unwrap();
        let user_agent = options.user_agent.to_string();
        let mut disabled = None::<SystemTime>;

        thread::Builder::new()
            .name("sentry-transport".to_string())
            .spawn(move || {
                sentry_debug!("spawning surf transport");
                let http_client = http_client;
                let url = dsn.envelope_api_url().to_string();

                while let Some(envelope) = receiver.recv().unwrap_or(None) {
                    // on drop we want to not continue processing the queue.
                    if shutdown_immediately.load(Ordering::SeqCst) {
                        let mut size = queue_size.lock().unwrap();
                        *size = 0;
                        signal.notify_all();
                        break;
                    }

                    // while we are disabled due to rate limits, skip
                    if let Some(ts) = disabled {
                        if let Ok(time_left) = ts.duration_since(SystemTime::now()) {
                            sentry_debug!(
                                "Skipping event send because we're disabled due to rate limits for {}s",
                                time_left.as_secs()
                            );
                            continue;
                        } else {
                            disabled = None;
                        }
                    }

                    let mut body = Vec::new();
                    envelope.to_writer(&mut body).unwrap();

                    let fut = http_client
                        .post(url.as_str())
                        .header(
                            "X-Sentry-Auth",
                            dsn.to_auth(Some(&user_agent)).to_string()
                        )
                        .body(body);

                    match executor::block_on(fut) {
                        Ok(resp) => {
                            if resp.status() == 429 {
                                if let Some(retry_after) = resp
                                    .header("Retry-After")
                                    .and_then(|x| x.iter().next())
                                    .map(|x| x.as_str())
                                    .and_then(parse_retry_after)
                                {
                                    disabled = Some(retry_after);
                                }
                            }
                        }
                        Err(err) => {
                            sentry_debug!("Failed to send event: {}", err);
                        }
                    }

                    let mut size = queue_size.lock().unwrap();
                    *size -= 1;
                    if *size == 0 {
                        signal.notify_all();
                    }
                }
            }).unwrap()
    }

    fn http_client(
        _options: &ClientOptions,
        client: Option<SurfClient>
    ) -> SurfClient {
        client.unwrap_or_else(SurfClient::new)
    }
}

#[cfg(feature = "reqwest")]
type DefaultTransport = ReqwestHttpTransport;

#[cfg(all(feature = "curl", not(feature = "reqwest"), not(feature = "surf")))]
type DefaultTransport = CurlHttpTransport;

#[cfg(all(feature = "surf", not(feature = "reqwest"), not(feature = "curl")))]
type DefaultTransport = SurfHttpTransport;

/// The default http transport.
#[cfg(any(feature = "reqwest", feature = "curl", feature = "surf"))]
pub type HttpTransport = DefaultTransport;
