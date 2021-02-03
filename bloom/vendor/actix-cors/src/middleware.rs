use std::{
    convert::TryInto,
    rc::Rc,
    task::{Context, Poll},
};

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse},
    error::{Error, Result},
    http::{
        header::{self, HeaderValue},
        Method,
    },
    HttpResponse,
};
use futures_util::future::{ok, Either, FutureExt as _, LocalBoxFuture, Ready};
use log::debug;

use crate::Inner;

/// Service wrapper for Cross-Origin Resource Sharing support.
///
/// This struct contains the settings for CORS requests to be validated and for responses to
/// be generated.
#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct CorsMiddleware<S> {
    pub(crate) service: S,
    pub(crate) inner: Rc<Inner>,
}

impl<S> CorsMiddleware<S> {
    fn handle_preflight<B>(inner: &Inner, req: ServiceRequest) -> ServiceResponse<B> {
        if let Err(err) = inner
            .validate_origin(req.head())
            .and_then(|_| inner.validate_allowed_method(req.head()))
            .and_then(|_| inner.validate_allowed_headers(req.head()))
        {
            return req.error_response(err);
        }

        let mut res = HttpResponse::Ok();

        if let Some(origin) = inner.access_control_allow_origin(req.head()) {
            res.header(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin);
        }

        if let Some(ref allowed_methods) = inner.allowed_methods_baked {
            res.header(
                header::ACCESS_CONTROL_ALLOW_METHODS,
                allowed_methods.clone(),
            );
        }

        if let Some(ref headers) = inner.allowed_headers_baked {
            res.header(header::ACCESS_CONTROL_ALLOW_HEADERS, headers.clone());
        } else if let Some(headers) =
            req.headers().get(header::ACCESS_CONTROL_REQUEST_HEADERS)
        {
            // all headers allowed, return
            res.header(header::ACCESS_CONTROL_ALLOW_HEADERS, headers.clone());
        }

        if inner.supports_credentials {
            res.header(
                header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
                HeaderValue::from_static("true"),
            );
        }

        if let Some(max_age) = inner.max_age {
            res.header(header::ACCESS_CONTROL_MAX_AGE, max_age.to_string());
        }

        let res = res.finish();
        let res = res.into_body();
        req.into_response(res)
    }

    fn augment_response<B>(
        inner: &Inner,
        mut res: ServiceResponse<B>,
    ) -> ServiceResponse<B> {
        if let Some(origin) = inner.access_control_allow_origin(res.request().head()) {
            res.headers_mut()
                .insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin);
        };

        if let Some(ref expose) = inner.expose_headers_baked {
            res.headers_mut()
                .insert(header::ACCESS_CONTROL_EXPOSE_HEADERS, expose.clone());
        }

        if inner.supports_credentials {
            res.headers_mut().insert(
                header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
                HeaderValue::from_static("true"),
            );
        }

        if inner.vary_header {
            let value = match res.headers_mut().get(header::VARY) {
                Some(hdr) => {
                    let mut val: Vec<u8> = Vec::with_capacity(hdr.len() + 8);
                    val.extend(hdr.as_bytes());
                    val.extend(b", Origin");
                    val.try_into().unwrap()
                }
                None => HeaderValue::from_static("Origin"),
            };

            res.headers_mut().insert(header::VARY, value);
        }

        res
    }
}

type CorsMiddlewareServiceFuture<B> = Either<
    Ready<Result<ServiceResponse<B>, Error>>,
    LocalBoxFuture<'static, Result<ServiceResponse<B>, Error>>,
>;

impl<S, B> Service for CorsMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = CorsMiddlewareServiceFuture<B>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        if self.inner.preflight && req.method() == Method::OPTIONS {
            let inner = Rc::clone(&self.inner);
            let res = Self::handle_preflight(&inner, req);
            Either::Left(ok(res))
        } else {
            let origin = req.headers().get(header::ORIGIN).cloned();

            if origin.is_some() {
                // Only check requests with a origin header.
                if let Err(err) = self.inner.validate_origin(req.head()) {
                    debug!("origin validation failed; inner service is not called");
                    return Either::Left(ok(req.error_response(err)));
                }
            }

            let inner = Rc::clone(&self.inner);
            let fut = self.service.call(req);

            let res = async move {
                let res = fut.await;

                if origin.is_some() {
                    let res = res?;
                    Ok(Self::augment_response(&inner, res))
                } else {
                    res
                }
            }
            .boxed_local();

            Either::Right(res)
        }
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{
        dev::Transform,
        test::{self, TestRequest},
    };

    use super::*;
    use crate::Cors;

    #[actix_rt::test]
    async fn test_options_no_origin() {
        // Tests case where allowed_origins is All but there are validate functions to run incase.
        // In this case, origins are only allowed when the DNT header is sent.

        let mut cors = Cors::default()
            .allow_any_origin()
            .allowed_origin_fn(|origin, req_head| {
                assert_eq!(&origin, req_head.headers.get(header::ORIGIN).unwrap());

                req_head.headers().contains_key(header::DNT)
            })
            .new_transform(test::ok_service())
            .await
            .unwrap();

        let req = TestRequest::get()
            .header(header::ORIGIN, "http://example.com")
            .to_srv_request();
        let res = cors.call(req).await.unwrap();
        assert_eq!(
            None,
            res.headers()
                .get(header::ACCESS_CONTROL_ALLOW_ORIGIN)
                .map(HeaderValue::as_bytes)
        );

        let req = TestRequest::get()
            .header(header::ORIGIN, "http://example.com")
            .header(header::DNT, "1")
            .to_srv_request();
        let res = cors.call(req).await.unwrap();
        assert_eq!(
            Some(&b"http://example.com"[..]),
            res.headers()
                .get(header::ACCESS_CONTROL_ALLOW_ORIGIN)
                .map(HeaderValue::as_bytes)
        );
    }
}
