use actix_web::Error;
use actix_web::Result;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse};
use actix_web::{
    dev::{Service, Transform},
    http::{HeaderName, HeaderValue},
};
use kernel::config::Config;
use std::{
    sync::Arc,
    task::{Context, Poll},
};
use stdx::futures::{
    future::{ok, LocalBoxFuture, Ready},
    FutureExt,
};

/// Security headers middleware.
/// sets the `X-Content-Type-Options`, `X-Frame-Options`,
/// `Strict-Transport-Security` security headers
/// we need script-src 'unsafe-inline' 'unsafe-eval' because of VueJS
/// img-src data: is for 2fa QR codes
pub struct SecurityHeadersMiddleware {
    config: Arc<Config>,
}

impl<S, B> Transform<S> for SecurityHeadersMiddleware
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SecurityHeadersMiddleware2<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let mut content_security_policy_value = format!("default-src 'self' https://js.stripe.com; img-src 'self' data:; script-src 'self' 'unsafe-eval' https://js.stripe.com; style-src 'self' 'unsafe-inline'; object-src 'none'; connect-src 'self' https://s3.{}.amazonaws.com https://{}.s3.{}.amazonaws.com", &self.config.s3.region, &self.config.s3.bucket, &self.config.s3.region);
        let mut expect_ct_value = String::from("max-age=86400, enforce");

        if let Some(ref ingest_domain) = self.config.sentry.ingest_domain {
            if let Some(ref security_report_uri) = self.config.sentry.security_report_uri {
                content_security_policy_value = content_security_policy_value
                    + format!(" {}; report-uri {}", ingest_domain, security_report_uri).as_str();
                expect_ct_value = expect_ct_value + format!(", report-uri=\"{}\"", security_report_uri).as_str();
            }
        }

        ok(SecurityHeadersMiddleware2 {
            service,
            content_security_policy_value,
            expect_ct_value,
        })
    }
}

impl SecurityHeadersMiddleware {
    pub fn new(config: Arc<Config>) -> Self {
        SecurityHeadersMiddleware {
            config,
        }
    }
}

/// Actual actix-web middleware
pub struct SecurityHeadersMiddleware2<S> {
    service: S,
    content_security_policy_value: String,
    expect_ct_value: String,
}

impl<S, B> Service for SecurityHeadersMiddleware2<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let req_fut = self.service.call(req);
        let content_security_policy_value = self.content_security_policy_value.clone();
        let expect_ct_value = self.expect_ct_value.clone();

        async move {
            let mut res = req_fut.await?;

            let headers = res.headers_mut();
            headers.insert(
                HeaderName::from_static("x-content-type-options"),
                HeaderValue::from_static("nosniff"),
            );
            headers.insert(
                HeaderName::from_static("x-frame-options"),
                HeaderValue::from_static("Deny"),
            );
            headers.insert(
                HeaderName::from_static("x-xss-protection"),
                HeaderValue::from_static("1; mode=block"),
            );
            headers.insert(
                HeaderName::from_static("x-download-options"),
                HeaderValue::from_static("noopen"),
            );
            headers.insert(
                HeaderName::from_static("strict-transport-security"),
                HeaderValue::from_static("max-age=63072000; includeSubDomains; preload"),
            );
            headers.insert(
                HeaderName::from_static("content-security-policy"),
                HeaderValue::from_str(&content_security_policy_value)
                    .expect("middlewares/security_headers: generating Content-Security-Policy header"),
            );
            headers.insert(
                HeaderName::from_static("expect-ct"),
                HeaderValue::from_str(&expect_ct_value)
                    .expect("middlewares/security_headers: generating Expect-CT header"),
            );
            Ok(res)
        }
        .boxed_local()
    }
}
