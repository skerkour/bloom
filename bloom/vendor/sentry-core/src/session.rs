//! Release Health Sessions
//!
//! https://develop.sentry.dev/sdk/sessions/

use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use crate::client::TransportArc;
use crate::protocol::{
    EnvelopeItem, Event, Level, SessionAttributes, SessionStatus, SessionUpdate,
};
use crate::scope::StackLayer;
use crate::types::{Utc, Uuid};
use crate::{Client, Envelope};

#[derive(Clone, Debug)]
pub struct Session {
    client: Arc<Client>,
    session_update: SessionUpdate<'static>,
    started: Instant,
    dirty: bool,
}

impl Drop for Session {
    fn drop(&mut self) {
        self.close(SessionStatus::Exited);
        if self.dirty {
            self.client.enqueue_session(self.session_update.clone());
        }
    }
}

impl Session {
    pub fn from_stack(stack: &StackLayer) -> Option<Self> {
        let client = stack.client.as_ref()?;
        let options = client.options();
        let user = stack.scope.user.as_ref();
        let distinct_id = user
            .and_then(|user| {
                user.id
                    .as_ref()
                    .or_else(|| user.email.as_ref())
                    .or_else(|| user.username.as_ref())
            })
            .cloned();
        Some(Self {
            client: client.clone(),
            session_update: SessionUpdate {
                session_id: Uuid::new_v4(),
                distinct_id,
                sequence: None,
                timestamp: None,
                started: Utc::now(),
                init: true,
                duration: None,
                status: SessionStatus::Ok,
                errors: 0,
                attributes: SessionAttributes {
                    release: options.release.clone()?,
                    environment: options.environment.clone(),
                    ip_address: None,
                    user_agent: None,
                },
            },
            started: Instant::now(),
            dirty: true,
        })
    }

    pub(crate) fn update_from_event(&mut self, event: &Event<'static>) {
        if self.session_update.status != SessionStatus::Ok {
            // a session that has already transitioned to a "terminal" state
            // should not receive any more updates
            return;
        }
        let mut has_error = event.level >= Level::Error;
        let mut is_crash = false;
        for exc in &event.exception.values {
            has_error = true;
            if let Some(mechanism) = &exc.mechanism {
                if let Some(false) = mechanism.handled {
                    is_crash = true;
                    break;
                }
            }
        }

        if is_crash {
            self.session_update.status = SessionStatus::Crashed;
        }
        if has_error {
            self.session_update.errors += 1;
            self.dirty = true;
        }
    }

    pub(crate) fn close(&mut self, status: SessionStatus) {
        if self.session_update.status == SessionStatus::Ok {
            let status = match status {
                SessionStatus::Ok => SessionStatus::Exited,
                s => s,
            };
            self.session_update.duration = Some(self.started.elapsed().as_secs_f64());
            self.session_update.status = status;
            self.dirty = true;
        }
    }

    pub(crate) fn create_envelope_item(&mut self) -> Option<EnvelopeItem> {
        if self.dirty {
            let item = self.session_update.clone().into();
            self.session_update.init = false;
            self.dirty = false;
            return Some(item);
        }
        None
    }
}

// as defined here: https://develop.sentry.dev/sdk/envelopes/#size-limits
const MAX_SESSION_ITEMS: usize = 100;
const FLUSH_INTERVAL: Duration = Duration::from_secs(60);

type SessionQueue = Arc<Mutex<Vec<SessionUpdate<'static>>>>;

/// Background Session Flusher
///
/// The background flusher queues session updates for delayed batched sending.
/// It has its own background thread that will flush its queue once every
/// `FLUSH_INTERVAL`.
///
/// For now it just batches all the session updates together into one envelope,
/// but in the future it will also pre-aggregate session numbers.
pub(crate) struct SessionFlusher {
    transport: TransportArc,
    queue: SessionQueue,
    shutdown: Arc<(Mutex<bool>, Condvar)>,
    worker: Option<JoinHandle<()>>,
}

impl SessionFlusher {
    /// Creates a new Flusher that will submit envelopes to the given `transport`.
    pub fn new(transport: TransportArc) -> Self {
        let queue = Arc::new(Mutex::new(Vec::new()));
        #[allow(clippy::mutex_atomic)]
        let shutdown = Arc::new((Mutex::new(false), Condvar::new()));

        let worker_transport = transport.clone();
        let worker_queue = queue.clone();
        let worker_shutdown = shutdown.clone();
        let worker = std::thread::Builder::new()
            .name("sentry-session-flusher".into())
            .spawn(move || {
                let (lock, cvar) = worker_shutdown.as_ref();
                let mut shutdown = lock.lock().unwrap();
                // check this immediately, in case the main thread is already shutting down
                if *shutdown {
                    return;
                }
                let mut last_flush = Instant::now();
                loop {
                    let timeout = FLUSH_INTERVAL - last_flush.elapsed();
                    shutdown = cvar.wait_timeout(shutdown, timeout).unwrap().0;
                    if *shutdown {
                        return;
                    }
                    if last_flush.elapsed() < FLUSH_INTERVAL {
                        continue;
                    }
                    SessionFlusher::flush(worker_queue.lock().unwrap(), &worker_transport);
                    last_flush = Instant::now();
                }
            })
            .unwrap();

        Self {
            transport,
            queue,
            shutdown,
            worker: Some(worker),
        }
    }

    /// Enqueues a session update for delayed sending.
    ///
    /// When the queue is full, it will be flushed immediately.
    pub fn enqueue(&self, session_update: SessionUpdate<'static>) {
        let mut queue = self.queue.lock().unwrap();
        queue.push(session_update);
        if queue.len() >= MAX_SESSION_ITEMS {
            SessionFlusher::flush(queue, &self.transport);
        }
    }

    /// Flushes the queue to the transport.
    ///
    /// This is a static method as it will be called from both the background
    /// thread and the main thread on drop.
    fn flush(mut queue_lock: MutexGuard<Vec<SessionUpdate<'static>>>, transport: &TransportArc) {
        let queue: Vec<_> = std::mem::take(queue_lock.as_mut());
        drop(queue_lock);

        if queue.is_empty() {
            return;
        }

        let mut envelope = Envelope::new();
        let mut items = 0;

        for session_update in queue {
            if items >= MAX_SESSION_ITEMS {
                if let Some(ref transport) = *transport.read().unwrap() {
                    transport.send_envelope(envelope);
                }
                envelope = Envelope::new();
                items = 0;
            }
            envelope.add_item(session_update);
            items += 1;
        }

        if let Some(ref transport) = *transport.read().unwrap() {
            transport.send_envelope(envelope);
        }
    }
}

impl Drop for SessionFlusher {
    fn drop(&mut self) {
        let (lock, cvar) = self.shutdown.as_ref();
        *lock.lock().unwrap() = true;
        cvar.notify_one();

        if let Some(worker) = self.worker.take() {
            worker.join().ok();
        }
        SessionFlusher::flush(self.queue.lock().unwrap(), &self.transport);
    }
}

#[cfg(all(test, feature = "test"))]
mod tests {
    use super::*;
    use crate as sentry;
    use crate::protocol::{Envelope, EnvelopeItem, SessionStatus};

    fn capture_envelopes<F>(f: F) -> Vec<Envelope>
    where
        F: FnOnce(),
    {
        crate::test::with_captured_envelopes_options(
            f,
            crate::ClientOptions {
                release: Some("some-release".into()),
                ..Default::default()
            },
        )
    }

    #[test]
    fn test_session_startstop() {
        let envelopes = capture_envelopes(|| {
            sentry::start_session();
            std::thread::sleep(std::time::Duration::from_millis(10));
        });
        assert_eq!(envelopes.len(), 1);

        let mut items = envelopes[0].items();
        if let Some(EnvelopeItem::SessionUpdate(session)) = items.next() {
            assert_eq!(session.status, SessionStatus::Exited);
            assert!(session.duration.unwrap() > 0.01);
            assert_eq!(session.errors, 0);
            assert_eq!(session.attributes.release, "some-release");
            assert_eq!(session.init, true);
        } else {
            panic!("expected session");
        }
        assert_eq!(items.next(), None);
    }

    #[test]
    fn test_session_batching() {
        #![allow(clippy::match_like_matches_macro)]
        let envelopes = capture_envelopes(|| {
            for _ in 0..(MAX_SESSION_ITEMS * 2) {
                sentry::start_session();
            }
        });
        // we only want *two* envelope for all the sessions
        assert_eq!(envelopes.len(), 2);

        let items = envelopes[0].items().chain(envelopes[1].items());
        assert_eq!(items.clone().count(), MAX_SESSION_ITEMS * 2);
        for item in items {
            assert!(match item {
                EnvelopeItem::SessionUpdate(_) => true,
                _ => false,
            });
        }
    }

    #[test]
    fn test_session_error() {
        let envelopes = capture_envelopes(|| {
            sentry::start_session();

            let err = "NaN".parse::<usize>().unwrap_err();
            sentry::capture_error(&err);
        });
        assert_eq!(envelopes.len(), 2);

        let mut items = envelopes[0].items();
        assert!(matches!(items.next(), Some(EnvelopeItem::Event(_))));
        if let Some(EnvelopeItem::SessionUpdate(session)) = items.next() {
            assert_eq!(session.status, SessionStatus::Ok);
            assert_eq!(session.errors, 1);
            assert_eq!(session.attributes.release, "some-release");
            assert_eq!(session.init, true);
        } else {
            panic!("expected session");
        }
        assert_eq!(items.next(), None);

        let mut items = envelopes[1].items();
        if let Some(EnvelopeItem::SessionUpdate(session)) = items.next() {
            assert_eq!(session.status, SessionStatus::Exited);
            assert_eq!(session.errors, 1);
            assert_eq!(session.init, false);
        } else {
            panic!("expected session");
        }
        assert_eq!(items.next(), None);
    }

    #[test]
    fn test_session_abnormal() {
        let envelopes = capture_envelopes(|| {
            sentry::start_session();
            sentry::end_session_with_status(SessionStatus::Abnormal);
        });
        assert_eq!(envelopes.len(), 1);

        let mut items = envelopes[0].items();
        if let Some(EnvelopeItem::SessionUpdate(session)) = items.next() {
            assert_eq!(session.status, SessionStatus::Abnormal);
            assert_eq!(session.init, true);
        } else {
            panic!("expected session");
        }
        assert_eq!(items.next(), None);
    }
    #[test]
    fn test_session_sampled_errors() {
        let mut envelopes = crate::test::with_captured_envelopes_options(
            || {
                sentry::start_session();

                for _ in 0..100 {
                    let err = "NaN".parse::<usize>().unwrap_err();
                    sentry::capture_error(&err);
                }
            },
            crate::ClientOptions {
                release: Some("some-release".into()),
                sample_rate: 0.5,
                ..Default::default()
            },
        );
        assert!(envelopes.len() > 25);
        assert!(envelopes.len() < 75);

        let envelope = envelopes.pop().unwrap();
        let mut items = envelope.items();
        if let Some(EnvelopeItem::SessionUpdate(session)) = items.next() {
            assert_eq!(session.status, SessionStatus::Exited);
            assert_eq!(session.errors, 100);
        } else {
            panic!("expected session");
        }
        assert_eq!(items.next(), None);
    }

    /// For _user-mode_ sessions, we want to inherit the session for any _new_
    /// Hub that is spawned from the main thread Hub which already has a session
    /// attached
    #[test]
    fn test_inherit_session_from_top() {
        let envelopes = capture_envelopes(|| {
            sentry::start_session();

            let err = "NaN".parse::<usize>().unwrap_err();
            sentry::capture_error(&err);

            // create a new Hub which should have the same session
            let hub = std::sync::Arc::new(sentry::Hub::new_from_top(sentry::Hub::current()));

            sentry::Hub::run(hub, || {
                let err = "NaN".parse::<usize>().unwrap_err();
                sentry::capture_error(&err);

                sentry::with_scope(
                    |_| {},
                    || {
                        let err = "NaN".parse::<usize>().unwrap_err();
                        sentry::capture_error(&err);
                    },
                );
            });
        });

        assert_eq!(envelopes.len(), 4); // 3 errors and one session end

        let mut items = envelopes[3].items();
        if let Some(EnvelopeItem::SessionUpdate(session)) = items.next() {
            assert_eq!(session.status, SessionStatus::Exited);
            assert_eq!(session.errors, 3);
            assert_eq!(session.init, false);
        } else {
            panic!("expected session");
        }
        assert_eq!(items.next(), None);
    }

    /// We want to forward-inherit sessions as the previous test asserted, but
    /// not *backwards*. So any new session created in a derived Hub and scope
    /// will only get updates from that particular scope.
    #[test]
    fn test_dont_inherit_session_backwards() {
        let envelopes = capture_envelopes(|| {
            let hub = std::sync::Arc::new(sentry::Hub::new_from_top(sentry::Hub::current()));

            sentry::Hub::run(hub, || {
                sentry::with_scope(
                    |_| {},
                    || {
                        sentry::start_session();

                        let err = "NaN".parse::<usize>().unwrap_err();
                        sentry::capture_error(&err);
                    },
                );

                let err = "NaN".parse::<usize>().unwrap_err();
                sentry::capture_error(&err);
            });

            let err = "NaN".parse::<usize>().unwrap_err();
            sentry::capture_error(&err);
        });

        assert_eq!(envelopes.len(), 4); // 3 errors and one session end

        let mut items = envelopes[0].items();
        assert!(matches!(items.next(), Some(EnvelopeItem::Event(_))));
        if let Some(EnvelopeItem::SessionUpdate(session)) = items.next() {
            assert_eq!(session.status, SessionStatus::Ok);
            assert_eq!(session.errors, 1);
            assert_eq!(session.init, true);
        } else {
            panic!("expected session");
        }
        assert_eq!(items.next(), None);

        // the other two events should not have session updates
        let mut items = envelopes[1].items();
        assert!(matches!(items.next(), Some(EnvelopeItem::Event(_))));
        assert_eq!(items.next(), None);

        let mut items = envelopes[2].items();
        assert!(matches!(items.next(), Some(EnvelopeItem::Event(_))));
        assert_eq!(items.next(), None);

        // the session end is sent last as it is possibly batched
        let mut items = envelopes[3].items();
        if let Some(EnvelopeItem::SessionUpdate(session)) = items.next() {
            assert_eq!(session.status, SessionStatus::Exited);
            assert_eq!(session.errors, 1);
            assert_eq!(session.init, false);
        } else {
            panic!("expected session");
        }
        assert_eq!(items.next(), None);
    }
}
