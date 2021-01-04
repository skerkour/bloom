use actix_web::Error;
use actix_web::Result;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse};
use actix_web::{
    dev::{Service, Transform},
    http::{HeaderName, HeaderValue},
};
use futures::future::{ok, Ready};
use kernel::config::Config;
use std::{
    sync::Arc,
    task::{Context, Poll},
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
        let mut content_security_policy_value = String::from("default-src 'self' https://js.stripe.com; img-src 'self' data:; script-src 'self' 'unsafe-eval' https://js.stripe.com; style-src 'self' 'unsafe-inline'; object-src 'none'; connect-src 'self'");
        let mut expect_ct_value = String::from("max-age=86400, enforce");

        if let Some(ref s3_bucket) = self.config.s3.bucket {
            content_security_policy_value = content_security_policy_value
                + format!("https://{}.s3.{}.amazonaws.com", s3_bucket, &self.config.s3.region).as_str();
        }

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
        SecurityHeadersMiddleware { config }
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
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        let headers = req.headers_mut();
        headers.insert(
            HeaderName::from_static("X-Content-Type-Options"),
            HeaderValue::from_static("nosniff"),
        );
        headers.insert(
            HeaderName::from_static("X-Frame-Options"),
            HeaderValue::from_static("Deny"),
        );
        headers.insert(
            HeaderName::from_static("X-XSS-Protection"),
            HeaderValue::from_static("1; mode=block"),
        );
        headers.insert(
            HeaderName::from_static("X-Download-Options"),
            HeaderValue::from_static("noopen"),
        );
        headers.insert(
            HeaderName::from_static("Strict-Transport-Security"),
            HeaderValue::from_static("max-age=63072000; includeSubDomains; preload"),
        );
        headers.insert(
            HeaderName::from_static("Content-Security-Policy"),
            HeaderValue::from_str(&self.content_security_policy_value)
                .expect("middlewares/security_headers: generating Content-Security-Policy header"),
        );
        headers.insert(
            HeaderName::from_static("Expect-CT"),
            HeaderValue::from_str(&self.expect_ct_value)
                .expect("middlewares/security_headers: generating Expect-CT header"),
        );

        // propagate the call
        self.service.call(req)
    }
}
