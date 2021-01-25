#![allow(unused)]

use std::cell::{Cell, UnsafeCell};
use std::error::Error;
use std::mem::drop;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, PoisonError, RwLock, TryLockError};
use std::thread;
use std::time::Duration;

use crate::protocol::{Breadcrumb, Event, Level, SessionStatus};
use crate::types::Uuid;
use crate::{event_from_error, Integration, IntoBreadcrumbs, Scope, ScopeGuard};
#[cfg(feature = "client")]
use crate::{scope::Stack, session::Session, Client, Envelope};

#[cfg(feature = "client")]
lazy_static::lazy_static! {
    static ref PROCESS_HUB: (Arc<Hub>, thread::ThreadId) = (
        Arc::new(Hub::new(None, Arc::new(Default::default()))),
        thread::current().id()
    );
}

#[cfg(feature = "client")]
thread_local! {
    static THREAD_HUB: UnsafeCell<Arc<Hub>> = UnsafeCell::new(
        Arc::new(Hub::new_from_top(&PROCESS_HUB.0)));
    static USE_PROCESS_HUB: Cell<bool> = Cell::new(PROCESS_HUB.1 == thread::current().id());
}

#[cfg(feature = "client")]
#[derive(Debug)]
pub(crate) struct HubImpl {
    stack: Arc<RwLock<Stack>>,
}

#[cfg(feature = "client")]
impl HubImpl {
    pub(crate) fn with<F: FnOnce(&Stack) -> R, R>(&self, f: F) -> R {
        let guard = self.stack.read().unwrap_or_else(PoisonError::into_inner);
        f(&*guard)
    }

    fn with_mut<F: FnOnce(&mut Stack) -> R, R>(&self, f: F) -> R {
        let mut guard = self.stack.write().unwrap_or_else(PoisonError::into_inner);
        f(&mut *guard)
    }

    fn is_active_and_usage_safe(&self) -> bool {
        let guard = match self.stack.read() {
            Err(err) => err.into_inner(),
            Ok(guard) => guard,
        };

        guard
            .top()
            .client
            .as_ref()
            .map_or(false, |c| c.is_enabled())
    }
}

/// The central object that can manages scopes and clients.
///
/// This can be used to capture events and manage the scope.  This object is
/// internally synchronized so it can be used from multiple threads if needed.
///
/// Each thread has its own thread-local (`Hub::current()`) hub, which is
/// automatically derived from the main hub (`Hub::main()`).
///
/// In most situations developers do not need to interface with the hub directly.  Instead
/// toplevel convenience functions are expose that will automatically dispatch
/// to the thread-local (`Hub::current()`) hub.  In some situations this might not be
/// possible in which case it might become necessary to manually work with the
/// hub.  This is for instance the case when working with async code.
///
/// Hubs that are wrapped in `Arc`s can be bound to the current thread with
/// the `run` static method.
///
/// Most common operations:
///
/// * `Hub::new`: creates a brand new hub
/// * `Hub::current`: returns the thread local hub
/// * `Hub::with`: invoke a callback with the thread local hub
/// * `Hub::with_active`: like `Hub::with` but does not invoke the callback if
///   the client is not in a supported state or not bound
/// * `Hub::new_from_top`: creates a new hub with just the top scope of another hub.
#[derive(Debug)]
pub struct Hub {
    #[cfg(feature = "client")]
    pub(crate) inner: HubImpl,
    last_event_id: RwLock<Option<Uuid>>,
}

impl Hub {
    /// Creates a new hub from the given client and scope.
    #[cfg(feature = "client")]
    pub fn new(client: Option<Arc<Client>>, scope: Arc<Scope>) -> Hub {
        Hub {
            inner: HubImpl {
                stack: Arc::new(RwLock::new(Stack::from_client_and_scope(client, scope))),
            },
            last_event_id: RwLock::new(None),
        }
    }

    /// Creates a new hub based on the top scope of the given hub.
    #[cfg(feature = "client")]
    pub fn new_from_top<H: AsRef<Hub>>(other: H) -> Hub {
        let hub = other.as_ref();
        hub.inner.with(|stack| {
            let top = stack.top();
            Hub::new(top.client.clone(), top.scope.clone())
        })
    }

    /// Returns the current hub.
    ///
    /// By default each thread gets a different thread local hub.  If an
    /// atomically reference counted hub is available it can override this
    /// one here by calling `Hub::run` with a closure.
    ///
    /// This method is unavailable if the client implementation is disabled.
    /// When using the minimal API set use `Hub::with_active` instead.
    #[cfg(feature = "client")]
    pub fn current() -> Arc<Hub> {
        Hub::with(Arc::clone)
    }

    /// Returns the main thread's hub.
    ///
    /// This is similar to `current` but instead of picking the current
    /// thread's hub it returns the main thread's hub instead.
    #[cfg(feature = "client")]
    pub fn main() -> Arc<Hub> {
        PROCESS_HUB.0.clone()
    }

    /// Invokes the callback with the default hub.
    ///
    /// This is a slightly more efficient version than `Hub::current()` and
    /// also unavailable in minimal mode.
    #[cfg(feature = "client")]
    pub fn with<F, R>(f: F) -> R
    where
        F: FnOnce(&Arc<Hub>) -> R,
    {
        if USE_PROCESS_HUB.with(Cell::get) {
            f(&PROCESS_HUB.0)
        } else {
            // not on safety: this is safe because even though we change the Arc
            // by temorary binding we guarantee that the original Arc stays alive.
            // For more information see: run
            THREAD_HUB.with(|stack| unsafe {
                let ptr = stack.get();
                f(&*ptr)
            })
        }
    }

    /// Like `Hub::with` but only calls the function if a client is bound.
    ///
    /// This is useful for integrations that want to do efficiently nothing if there is no
    /// client bound.  Additionally this internally ensures that the client can be safely
    /// synchronized.  This prevents accidental recursive calls into the client.
    pub fn with_active<F, R>(f: F) -> R
    where
        F: FnOnce(&Arc<Hub>) -> R,
        R: Default,
    {
        with_client_impl! {{
            Hub::with(|hub| {
                if hub.is_active_and_usage_safe() {
                    f(hub)
                } else {
                    Default::default()
                }
            })
        }}
    }

    /// Binds a hub to the current thread for the duration of the call.
    #[cfg(feature = "client")]
    pub fn run<F: FnOnce() -> R, R>(hub: Arc<Hub>, f: F) -> R {
        let mut restore_process_hub = false;
        let did_switch = THREAD_HUB.with(|ctx| unsafe {
            let ptr = ctx.get();
            if &**ptr as *const _ == &*hub as *const _ {
                None
            } else {
                USE_PROCESS_HUB.with(|x| {
                    if x.get() {
                        restore_process_hub = true;
                        x.set(false);
                    }
                });
                let old = (*ptr).clone();
                *ptr = hub.clone();
                Some(old)
            }
        });

        match did_switch {
            None => {
                // None means no switch happened.  We can invoke the function
                // just like that, no changes necessary.
                f()
            }
            Some(old_hub) => {
                use std::panic;

                // this is for the case where we just switched the hub.  This
                // means we need to catch the panic, restore the
                // old context and resume the panic if needed.
                let rv = panic::catch_unwind(panic::AssertUnwindSafe(f));
                THREAD_HUB.with(|ctx| unsafe { *ctx.get() = old_hub });
                if restore_process_hub {
                    USE_PROCESS_HUB.with(|x| x.set(true));
                }
                match rv {
                    Err(err) => panic::resume_unwind(err),
                    Ok(rv) => rv,
                }
            }
        }
    }

    /// Looks up an integration on the hub.
    ///
    /// Calls the given function with the requested integration instance when it
    /// is active on the currently active client.
    ///
    /// See the global [`capture_event`](fn.capture_event.html)
    /// for more documentation.
    pub fn with_integration<I, F, R>(&self, f: F) -> R
    where
        I: Integration,
        F: FnOnce(&I) -> R,
        R: Default,
    {
        with_client_impl! {{
            if let Some(client) = self.client() {
                if let Some(integration) = client.get_integration::<I>() {
                    return f(integration);
                }
            }
            Default::default()
        }}
    }

    /// Returns the last event id.
    pub fn last_event_id(&self) -> Option<Uuid> {
        *self.last_event_id.read().unwrap()
    }

    /// Sends the event to the current client with the current scope.
    ///
    /// In case no client is bound this does nothing instead.
    ///
    /// See the global [`capture_event`](fn.capture_event.html)
    /// for more documentation.
    pub fn capture_event(&self, event: Event<'static>) -> Uuid {
        with_client_impl! {{
            self.inner.with(|stack| {
                let top = stack.top();
                if let Some(ref client) = top.client {
                    let event_id = client.capture_event(event, Some(&top.scope));
                    *self.last_event_id.write().unwrap() = Some(event_id);
                    event_id
                } else {
                    Default::default()
                }
            })
        }}
    }

    /// Captures an arbitrary message.
    ///
    /// See the global [`capture_message`](fn.capture_message.html)
    /// for more documentation.
    pub fn capture_message(&self, msg: &str, level: Level) -> Uuid {
        with_client_impl! {{
            self.inner.with(|stack| {
                let top = stack.top();
                if let Some(ref client) = top.client {
                    let mut event = Event {
                        message: Some(msg.to_string()),
                        level,
                        ..Default::default()
                    };
                    self.capture_event(event)
                } else {
                    Uuid::nil()
                }
            })
        }}
    }

    /// Returns the currently bound client.
    #[cfg(feature = "client")]
    pub fn client(&self) -> Option<Arc<Client>> {
        self.inner.with(|stack| stack.top().client.clone())
    }

    /// Binds a new client to the hub.
    #[cfg(feature = "client")]
    pub fn bind_client(&self, client: Option<Arc<Client>>) {
        self.inner.with_mut(|stack| {
            stack.top_mut().client = client;
        })
    }

    /// Start a new session for Release Health.
    ///
    /// See the global [`start_session`](fn.start_session.html)
    /// for more documentation.
    pub fn start_session(&self) {
        with_client_impl! {{
            self.inner.with_mut(|stack| {
                let top = stack.top_mut();
                if let Some(session) = Session::from_stack(top) {
                    // When creating a *new* session, we make sure it is unique,
                    // as to no inherit *backwards* to any parents.
                    let mut scope = Arc::make_mut(&mut top.scope);
                    scope.session = Arc::new(Mutex::new(Some(session)));
                }
            })
        }}
    }

    /// End the current Release Health Session.
    ///
    /// See the global [`end_session`](crate::end_session_with)
    /// for more documentation.
    pub fn end_session(&self) {
        self.end_session_with_status(SessionStatus::Exited)
    }
    /// End the current Release Health Session with the given [`SessionStatus`].
    ///
    /// See the global [`end_session_with_status`](crate::end_session_with_status)
    /// for more documentation.
    pub fn end_session_with_status(&self, status: SessionStatus) {
        with_client_impl! {{
            self.inner.with_mut(|stack| {
                let top = stack.top_mut();
                // drop will close and enqueue the session
                if let Some(mut session) = top.scope.session.lock().unwrap().take() {
                    session.close(status);
                }
            })
        }}
    }

    /// Pushes a new scope.
    ///
    /// This returns a guard that when dropped will pop the scope again.
    pub fn push_scope(&self) -> ScopeGuard {
        with_client_impl! {{
            self.inner.with_mut(|stack| {
                stack.push();
                ScopeGuard(Some((self.inner.stack.clone(), stack.depth())))
            })
        }}
    }

    /// Temporarily pushes a scope for a single call optionally reconfiguring it.
    ///
    /// See the global [`with_scope`](fn.with_scope.html)
    /// for more documentation.
    pub fn with_scope<C, F, R>(&self, scope_config: C, callback: F) -> R
    where
        C: FnOnce(&mut Scope),
        F: FnOnce() -> R,
    {
        #[cfg(feature = "client")]
        {
            let _guard = self.push_scope();
            self.configure_scope(scope_config);
            callback()
        }
        #[cfg(not(feature = "client"))]
        {
            let _scope_config = scope_config;
            callback()
        }
    }

    /// Invokes a function that can modify the current scope.
    ///
    /// See the global [`configure_scope`](fn.configure_scope.html)
    /// for more documentation.
    pub fn configure_scope<F, R>(&self, f: F) -> R
    where
        R: Default,
        F: FnOnce(&mut Scope) -> R,
    {
        with_client_impl! {{
            let mut new_scope = self.with_current_scope(|scope| scope.clone());
            let rv = f(&mut new_scope);
            self.with_current_scope_mut(|ptr| *ptr = new_scope);
            rv
        }}
    }

    /// Adds a new breadcrumb to the current scope.
    ///
    /// See the global [`add_breadcrumb`](fn.add_breadcrumb.html)
    /// for more documentation.
    pub fn add_breadcrumb<B: IntoBreadcrumbs>(&self, breadcrumb: B) {
        with_client_impl! {{
            self.inner.with_mut(|stack| {
                let top = stack.top_mut();
                if let Some(ref client) = top.client {
                    let scope = Arc::make_mut(&mut top.scope);
                    let options = client.options();
                    for breadcrumb in breadcrumb.into_breadcrumbs() {
                        let breadcrumb_opt = match options.before_breadcrumb {
                            Some(ref callback) => callback(breadcrumb),
                            None => Some(breadcrumb)
                        };
                        if let Some(breadcrumb) = breadcrumb_opt {
                            scope.breadcrumbs.push_back(breadcrumb);
                        }
                        while scope.breadcrumbs.len() > options.max_breadcrumbs {
                            scope.breadcrumbs.pop_front();
                        }
                    }
                }
            })
        }}
    }

    #[cfg(feature = "client")]
    pub(crate) fn is_active_and_usage_safe(&self) -> bool {
        self.inner.is_active_and_usage_safe()
    }

    #[cfg(feature = "client")]
    pub(crate) fn with_current_scope<F: FnOnce(&Scope) -> R, R>(&self, f: F) -> R {
        self.inner.with(|stack| f(&stack.top().scope))
    }

    #[cfg(feature = "client")]
    pub(crate) fn with_current_scope_mut<F: FnOnce(&mut Scope) -> R, R>(&self, f: F) -> R {
        self.inner
            .with_mut(|stack| f(Arc::make_mut(&mut stack.top_mut().scope)))
    }
}
