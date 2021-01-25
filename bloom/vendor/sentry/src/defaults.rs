#![cfg_attr(feature = "error-chain", allow(deprecated))]
#![cfg_attr(feature = "failure", allow(deprecated))]

use std::env;
use std::{borrow::Cow, sync::Arc};

use crate::transports::DefaultTransportFactory;
use crate::types::Dsn;
use crate::{ClientOptions, Integration};

/// Apply default client options.
///
/// Extends the given `ClientOptions` with default options such as a default
/// transport, a set of default integrations if not requested otherwise, and
/// also sets the `dsn`, `release`, `environment`, and proxy settings based on
/// environment variables.
///
/// When the `default_integrations` option is set to `true` (by default), the
/// following integrations will be added *before* any manually defined
/// integrations, depending on enabled feature flags:
///
/// 1. [`AttachStacktraceIntegration`] (`feature = "backtrace"`)
/// 2. [`DebugImagesIntegration`] (`feature = "debug-images"`)
/// 3. [`ErrorChainIntegration`] (`feature = "error-chain"`)
/// 4. [`ContextIntegration`] (`feature = "contexts"`)
/// 5. [`FailureIntegration`] (`feature = "failure"`)
/// 6. [`PanicIntegration`] (`feature = "panic"`)
/// 7. [`ProcessStacktraceIntegration`] (`feature = "backtrace"`)
///
/// Some integrations can be used multiple times, however, the
/// [`PanicIntegration`] can not, and it will not pick up custom panic
/// extractors when it is defined multiple times.
///
/// # Examples
/// ```
/// std::env::set_var("SENTRY_RELEASE", "release-from-env");
///
/// let options = sentry::ClientOptions::default();
/// assert_eq!(options.release, None);
/// assert!(options.transport.is_none());
///
/// let options = sentry::apply_defaults(options);
/// assert_eq!(options.release, Some("release-from-env".into()));
/// assert!(options.transport.is_some());
/// ```
///
/// [`AttachStacktraceIntegration`]: integrations/backtrace/struct.AttachStacktraceIntegration.html
/// [`DebugImagesIntegration`]: integrations/debug_images/struct.DebugImagesIntegration.html
/// [`ErrorChainIntegration`]: integrations/error_chain/struct.ErrorChainIntegration.html
/// [`ContextIntegration`]: integrations/contexts/struct.ContextIntegration.html
/// [`FailureIntegration`]: integrations/failure/struct.FailureIntegration.html
/// [`PanicIntegration`]: integrations/panic/struct.PanicIntegration.html
/// [`ProcessStacktraceIntegration`]: integrations/backtrace/struct.ProcessStacktraceIntegration.html
pub fn apply_defaults(mut opts: ClientOptions) -> ClientOptions {
    if opts.transport.is_none() {
        opts.transport = Some(Arc::new(DefaultTransportFactory));
    }
    if opts.default_integrations {
        // default integrations need to be ordered *before* custom integrations,
        // since they also process events in order
        let mut integrations: Vec<Arc<dyn Integration>> = vec![];
        #[cfg(feature = "backtrace")]
        {
            integrations.push(Arc::new(
                sentry_backtrace::AttachStacktraceIntegration::default(),
            ));
        }
        #[cfg(feature = "debug-images")]
        {
            integrations.push(Arc::new(
                sentry_debug_images::DebugImagesIntegration::default(),
            ))
        }
        #[cfg(feature = "error-chain")]
        {
            integrations.push(Arc::new(
                sentry_error_chain::ErrorChainIntegration::default(),
            ))
        }
        #[cfg(feature = "contexts")]
        {
            integrations.push(Arc::new(sentry_contexts::ContextIntegration::default()));
        }
        #[cfg(feature = "failure")]
        {
            integrations.push(Arc::new(sentry_failure::FailureIntegration::default()));
        }
        #[cfg(feature = "panic")]
        {
            #[allow(unused_mut)]
            let mut integration = sentry_panic::PanicIntegration::default();
            #[cfg(feature = "failure")]
            {
                integration = integration.add_extractor(sentry_failure::panic_extractor);
            }
            integrations.push(Arc::new(integration));
        }
        #[cfg(feature = "backtrace")]
        {
            integrations.push(Arc::new(
                sentry_backtrace::ProcessStacktraceIntegration::default(),
            ));
        }
        integrations.extend(opts.integrations.into_iter());
        opts.integrations = integrations;
    }
    if opts.dsn.is_none() {
        opts.dsn = env::var("SENTRY_DSN")
            .ok()
            .and_then(|dsn| dsn.parse::<Dsn>().ok());
    }
    if opts.release.is_none() {
        opts.release = env::var("SENTRY_RELEASE").ok().map(Cow::Owned);
    }
    if opts.environment.is_none() {
        opts.environment = env::var("SENTRY_ENVIRONMENT")
            .ok()
            .map(Cow::Owned)
            .or_else(|| {
                Some(Cow::Borrowed(if cfg!(debug_assertions) {
                    "debug"
                } else {
                    "release"
                }))
            });
    }
    if opts.http_proxy.is_none() {
        opts.http_proxy = std::env::var("HTTP_PROXY")
            .ok()
            .map(Cow::Owned)
            .or_else(|| std::env::var("http_proxy").ok().map(Cow::Owned));
    }
    if opts.https_proxy.is_none() {
        opts.https_proxy = std::env::var("HTTPS_PROXY")
            .ok()
            .map(Cow::Owned)
            .or_else(|| std::env::var("https_proxy").ok().map(Cow::Owned))
            .or_else(|| opts.http_proxy.clone());
    }
    opts
}
