use actix_web::dev::{Service, Transform};
use actix_web::error::ErrorBadRequest;
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::Result;
use actix_web::{dev, Error, FromRequest, HttpMessage, HttpRequest};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse};
use futures::future::{err, ok, Ready};
use std::task::{Context, Poll};
use stdx::{ulid::Ulid, uuid::Uuid};

/// The header set by the middleware
pub const REQUEST_ID_HEADER: &str = "x-bloom-request-id";

/// Request ID middleware.
pub struct RequestIdMiddleware;

impl<S, B> Transform<S> for RequestIdMiddleware
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestIdMiddleware2<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequestIdMiddleware2 { service })
    }
}

/// Actual actix-web middleware
pub struct RequestIdMiddleware2<S> {
    service: S,
}

impl<S, B> Service for RequestIdMiddleware2<S>
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

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        // generate request id token
        let request_id: Uuid = Ulid::new().into();

        // make object mutable (required as the header must be used inside `.call`)
        let mut req = req;

        // add request id header (for using in the log wrapper)
        req.headers_mut().append(
            HeaderName::from_static(REQUEST_ID_HEADER),
            HeaderValue::from_str(&request_id.to_hyphenated().to_string())
                .expect("middlewares/request_id: generating header value"),
        );

        // add request id extension (for extractor)
        req.extensions_mut().insert(RequestId(request_id));

        // propagate the call
        self.service.call(req)
    }
}

/// Request ID extractor
pub struct RequestId(pub Uuid);

impl FromRequest for RequestId {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        if let Some(RequestId(req_id)) = req.extensions().get::<RequestId>() {
            ok(RequestId(req_id.clone()))
        } else {
            err(ErrorBadRequest("middlewares/request_id: request_id is missing"))
        }
    }
}