use std::any::TypeId;
use std::borrow::Cow;
use std::fmt;
use std::panic::RefUnwindSafe;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;

use rand::random;
use sentry_types::protocol::v7::SessionUpdate;

use crate::constants::SDK_INFO;
use crate::protocol::{ClientSdkInfo, Event};
use crate::session::SessionFlusher;
use crate::types::{Dsn, Uuid};
use crate::{ClientOptions, Envelope, Hub, Integration, Scope, Transport};

impl<T: Into<ClientOptions>> From<T> for Client {
    fn from(o: T) -> Client {
        Client::with_options(o.into())
    }
}

pub(crate) type TransportArc = Arc<RwLock<Option<Arc<dyn Transport>>>>;

/// The Sentry Client.
///
/// The Client is responsible for event processing and sending events to the
/// sentry server via the configured [`Transport`]. It can be created from a
/// [`ClientOptions`].
///
/// See the [Unified API] document for more details.
///
/// # Examples
///
/// ```
/// sentry::Client::from(sentry::ClientOptions::default());
/// ```
///
/// [`ClientOptions`]: struct.ClientOptions.html
/// [`Transport`]: trait.Transport.html
/// [Unified API]: https://develop.sentry.dev/sdk/unified-api/
pub struct Client {
    options: ClientOptions,
    transport: TransportArc,
    session_flusher: SessionFlusher,
    integrations: Vec<(TypeId, Arc<dyn Integration>)>,
    sdk_info: ClientSdkInfo,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client")
            .field("dsn", &self.dsn())
            .field("options", &self.options)
            .finish()
    }
}

impl Clone for Client {
    fn clone(&self) -> Client {
        let transport = Arc::new(RwLock::new(self.transport.read().unwrap().clone()));
        let session_flusher = SessionFlusher::new(transport.clone());
        Client {
            options: self.options.clone(),
            transport,
            session_flusher,
            integrations: self.integrations.clone(),
            sdk_info: self.sdk_info.clone(),
        }
    }
}

impl Client {
    /// Creates a new Sentry client from a config.
    ///
    /// # Supported Configs
    ///
    /// The following common values are supported for the client config:
    ///
    /// * `ClientOptions`: configure the client with the given client options.
    /// * `()` or empty string: Disable the client.
    /// * `&str` / `String` / `&OsStr` / `String`: configure the client with the given DSN.
    /// * `Dsn` / `&Dsn`: configure the client with a given DSN.
    /// * `(Dsn, ClientOptions)`: configure the client from the given DSN and optional options.
    ///
    /// The `Default` implementation of `ClientOptions` pulls in the DSN from the
    /// `SENTRY_DSN` environment variable.
    ///
    /// # Panics
    ///
    /// The `Into<ClientOptions>` implementations can panic for the forms where a DSN needs to be
    /// parsed.  If you want to handle invalid DSNs you need to parse them manually by calling
    /// parse on it and handle the error.
    pub fn from_config<O: Into<ClientOptions>>(opts: O) -> Client {
        Client::with_options(opts.into())
    }

    /// Creates a new sentry client for the given options.
    ///
    /// If the DSN on the options is set to `None` the client will be entirely
    /// disabled.
    pub fn with_options(mut options: ClientOptions) -> Client {
        // Create the main hub eagerly to avoid problems with the background thread
        // See https://github.com/getsentry/sentry-rust/issues/237
        Hub::with(|_| {});

        let create_transport = || {
            options.dsn.as_ref()?;
            let factory = options.transport.as_ref()?;
            Some(factory.create_transport(&options))
        };

        let transport = Arc::new(RwLock::new(create_transport()));

        let mut sdk_info = SDK_INFO.clone();

        // NOTE: We do not filter out duplicate integrations based on their
        // TypeId.
        let integrations: Vec<_> = options
            .integrations
            .iter()
            .map(|integration| (integration.as_ref().type_id(), integration.clone()))
            .collect();

        for (_, integration) in integrations.iter() {
            integration.setup(&mut options);
            sdk_info.integrations.push(integration.name().to_string());
        }

        let session_flusher = SessionFlusher::new(transport.clone());
        Client {
            options,
            transport,
            session_flusher,
            integrations,
            sdk_info,
        }
    }

    pub(crate) fn get_integration<I>(&self) -> Option<&I>
    where
        I: Integration,
    {
        let id = TypeId::of::<I>();
        let integration = &self.integrations.iter().find(|(iid, _)| *iid == id)?.1;
        integration.as_ref().as_any().downcast_ref()
    }

    fn prepare_event(
        &self,
        mut event: Event<'static>,
        scope: Option<&Scope>,
    ) -> Option<Event<'static>> {
        if let Some(scope) = scope {
            scope.update_session_from_event(&event);
        }

        if !self.sample_should_send() {
            return None;
        }

        // event_id and sdk_info are set before the processors run so that the
        // processors can poke around in that data.
        if event.event_id.is_nil() {
            event.event_id = Uuid::new_v4();
        }

        if event.sdk.is_none() {
            // NOTE: we need to clone here because `Event` must be `'static`
            event.sdk = Some(Cow::Owned(self.sdk_info.clone()));
        }

        if let Some(scope) = scope {
            event = match scope.apply_to_event(event) {
                Some(event) => event,
                None => return None,
            };
        }

        for (_, integration) in self.integrations.iter() {
            let id = event.event_id;
            event = match integration.process_event(event, &self.options) {
                Some(event) => event,
                None => {
                    sentry_debug!("integration dropped event {:?}", id);
                    return None;
                }
            }
        }

        if event.release.is_none() {
            event.release = self.options.release.clone();
        }
        if event.environment.is_none() {
            event.environment = self.options.environment.clone();
        }
        if event.server_name.is_none() {
            event.server_name = self.options.server_name.clone();
        }
        if &event.platform == "other" {
            event.platform = "native".into();
        }

        if let Some(ref func) = self.options.before_send {
            sentry_debug!("invoking before_send callback");
            let id = event.event_id;
            func(event).or_else(move || {
                sentry_debug!("before_send dropped event {:?}", id);
                None
            })
        } else {
            Some(event)
        }
    }

    /// Returns the options of this client.
    pub fn options(&self) -> &ClientOptions {
        &self.options
    }

    /// Returns the DSN that constructed this client.
    pub fn dsn(&self) -> Option<&Dsn> {
        self.options.dsn.as_ref()
    }

    /// Quick check to see if the client is enabled.
    ///
    /// The Client is enabled if it has a valid DSN and Transport configured.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    ///
    /// let client = sentry::Client::from(sentry::ClientOptions::default());
    /// assert!(!client.is_enabled());
    ///
    /// let dsn = "https://public@example.com/1";
    /// let transport = sentry::test::TestTransport::new();
    /// let client = sentry::Client::from((
    ///     dsn,
    ///     sentry::ClientOptions {
    ///         transport: Some(Arc::new(transport)),
    ///         ..Default::default()
    ///     },
    /// ));
    /// assert!(client.is_enabled());
    /// ```
    pub fn is_enabled(&self) -> bool {
        self.options.dsn.is_some() && self.transport.read().unwrap().is_some()
    }

    /// Captures an event and sends it to sentry.
    pub fn capture_event(&self, event: Event<'static>, scope: Option<&Scope>) -> Uuid {
        if let Some(ref transport) = *self.transport.read().unwrap() {
            if let Some(event) = self.prepare_event(event, scope) {
                let event_id = event.event_id;
                let mut envelope: Envelope = event.into();
                let session_item = scope.and_then(|scope| {
                    scope
                        .session
                        .lock()
                        .unwrap()
                        .as_mut()
                        .and_then(|session| session.create_envelope_item())
                });
                if let Some(session_item) = session_item {
                    envelope.add_item(session_item);
                }
                transport.send_envelope(envelope);
                return event_id;
            }
        }
        Default::default()
    }

    pub(crate) fn enqueue_session(&self, session_update: SessionUpdate<'static>) {
        self.session_flusher.enqueue(session_update)
    }

    /// Drains all pending events and shuts down the transport behind the
    /// client.  After shutting down the transport is removed.
    ///
    /// This returns `true` if the queue was successfully drained in the
    /// given time or `false` if not (for instance because of a timeout).
    /// If no timeout is provided the client will wait for as long a
    /// `shutdown_timeout` in the client options.
    pub fn close(&self, timeout: Option<Duration>) -> bool {
        let transport_opt = self.transport.write().unwrap().take();
        if let Some(transport) = transport_opt {
            sentry_debug!("client close; request transport to shut down");
            transport.shutdown(timeout.unwrap_or(self.options.shutdown_timeout))
        } else {
            sentry_debug!("client close; no transport to shut down");
            true
        }
    }

    fn sample_should_send(&self) -> bool {
        let rate = self.options.sample_rate;
        if rate >= 1.0 {
            true
        } else {
            random::<f32>() <= rate
        }
    }
}

// Make this unwind safe. It's not out of the box because of the
// `BeforeCallback`s inside `ClientOptions`, and the contained Integrations
impl RefUnwindSafe for Client {}
