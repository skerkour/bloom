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

/// Security headers middleware.
/// sets the correct headers for no API caching
pub struct NoCacheHeadersMiddleware;

impl<S, B> Transform<S> for NoCacheHeadersMiddleware
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = NoCacheHeadersMiddleware2<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(NoCacheHeadersMiddleware2 {
            service,
        })
    }
}

/// Actual actix-web middleware
pub struct NoCacheHeadersMiddleware2<S> {
    service: S,
}

impl<S, B> Service for NoCacheHeadersMiddleware2<S>
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
            headers.insert(
                HeaderName::from_static("cache-control"),
                HeaderValue::from_static("no-cache, no-store, no-transform, must-revalidate, private, max-age=0"),
            );
            headers.insert(
                HeaderName::from_static("x-accel-expires"),
                HeaderValue::from_static("0"),
            );
            headers.insert(HeaderName::from_static("pragma"), HeaderValue::from_static("no-cache"));
            // "Expires": time.Unix(0, 0).Format(time.RFC1123),

            Ok(res)
        }
        .boxed_local()
    }
}
