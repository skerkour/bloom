use actix_web::Error;
use actix_web::Result;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse};
use actix_web::{
    dev::{Service, Transform},
    http::{HeaderName, HeaderValue},
};
use futures::future::{ok, Ready};
use std::task::{Context, Poll};

/// Security headers middleware.
/// sets the correct headers for CDN caching
pub struct CacheHeadersMiddleware;

impl<S, B> Transform<S> for CacheHeadersMiddleware
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CacheHeadersMiddleware2<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CacheHeadersMiddleware2 { service })
    }
}

/// Actual actix-web middleware
pub struct CacheHeadersMiddleware2<S> {
    service: S,
}

impl<S, B> Service for CacheHeadersMiddleware2<S>
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
            HeaderName::from_static("Cache-Control"),
            HeaderValue::from_static("public, max-age=0, s-maxage=31536000"),
        );
        headers.insert(
            HeaderName::from_static("X-Accel-Expires"),
            HeaderValue::from_static("31536000"),
        );

        // propagate the call
        self.service.call(req)
    }
}
