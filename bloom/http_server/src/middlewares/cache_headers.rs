use actix_web::Error;
use actix_web::Result;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse};
use actix_web::{
    dev::{Service, Transform},
    http::{HeaderName, HeaderValue},
};
use std::task::{Context, Poll};
use stdx::futures::{
    future::{ok, LocalBoxFuture, Ready},
    FutureExt,
};

const CACHE_CONTROL_HEADER: &str = "cache-control";
const X_ACCEL_EXPIRES_HEADER: &str = "x-accel-expires";

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
        ok(CacheHeadersMiddleware2 {
            service,
        })
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
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let req_fut = self.service.call(req);

        async move {
            let mut res = req_fut.await?;
            let headers = res.headers_mut();
            if !headers.contains_key(CACHE_CONTROL_HEADER) {
                headers.insert(
                    HeaderName::from_static(CACHE_CONTROL_HEADER),
                    HeaderValue::from_static("public, max-age=0, s-maxage=31536000"),
                );
            }
            if !headers.contains_key(X_ACCEL_EXPIRES_HEADER) {
                headers.insert(
                    HeaderName::from_static(X_ACCEL_EXPIRES_HEADER),
                    HeaderValue::from_static("31536000"),
                );
            }

            Ok(res)
        }
        .boxed_local()
    }
}
