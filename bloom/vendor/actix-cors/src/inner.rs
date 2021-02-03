use std::{collections::HashSet, convert::TryFrom, convert::TryInto, fmt, rc::Rc};

use actix_web::{
    dev::RequestHead,
    error::Result,
    http::{
        header::{self, HeaderName, HeaderValue},
        Method,
    },
};
use once_cell::sync::Lazy;
use tinyvec::TinyVec;

use crate::{AllOrSome, CorsError};

#[derive(Clone)]
pub(crate) struct OriginFn {
    pub(crate) boxed_fn: Rc<dyn Fn(&HeaderValue, &RequestHead) -> bool>,
}

impl Default for OriginFn {
    /// Dummy default for use in tiny_vec. Do not use.
    fn default() -> Self {
        let boxed_fn: Rc<dyn Fn(&_, &_) -> _> = Rc::new(|_origin, _req_head| false);
        Self { boxed_fn }
    }
}

impl fmt::Debug for OriginFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("origin_fn")
    }
}

/// Try to parse header value as HTTP method.
fn header_value_try_into_method(hdr: &HeaderValue) -> Option<Method> {
    hdr.to_str()
        .ok()
        .and_then(|meth| Method::try_from(meth).ok())
}

#[derive(Debug, Clone)]
pub(crate) struct Inner {
    pub(crate) allowed_origins: AllOrSome<HashSet<HeaderValue>>,
    pub(crate) allowed_origins_fns: TinyVec<[OriginFn; 4]>,

    pub(crate) allowed_methods: HashSet<Method>,
    pub(crate) allowed_methods_baked: Option<HeaderValue>,

    pub(crate) allowed_headers: AllOrSome<HashSet<HeaderName>>,
    pub(crate) allowed_headers_baked: Option<HeaderValue>,

    /// `All` will echo back `Access-Control-Request-Header` list.
    pub(crate) expose_headers: AllOrSome<HashSet<HeaderName>>,
    pub(crate) expose_headers_baked: Option<HeaderValue>,

    pub(crate) max_age: Option<usize>,
    pub(crate) preflight: bool,
    pub(crate) send_wildcard: bool,
    pub(crate) supports_credentials: bool,
    pub(crate) vary_header: bool,
}

static EMPTY_ORIGIN_SET: Lazy<HashSet<HeaderValue>> = Lazy::new(HashSet::new);

impl Inner {
    pub(crate) fn validate_origin(&self, req: &RequestHead) -> Result<(), CorsError> {
        // return early if all origins are allowed or get ref to allowed origins set
        #[allow(clippy::mutable_key_type)]
        let allowed_origins = match &self.allowed_origins {
            AllOrSome::All if self.allowed_origins_fns.is_empty() => return Ok(()),
            AllOrSome::Some(allowed_origins) => allowed_origins,
            // only function origin validators are defined
            _ => &EMPTY_ORIGIN_SET,
        };

        // get origin header and try to parse as string
        match req.headers().get(header::ORIGIN) {
            // origin header exists and is a string
            Some(origin) => {
                if allowed_origins.contains(origin)
                    || self.validate_origin_fns(origin, req)
                {
                    Ok(())
                } else {
                    Err(CorsError::OriginNotAllowed)
                }
            }

            // origin header is missing
            // note: with our implementation, the origin header is required for OPTIONS request or
            // else this would be unreachable
            None => Err(CorsError::MissingOrigin),
        }
    }

    /// Accepts origin if _ANY_ functions return true. Only called when Origin exists.
    fn validate_origin_fns(&self, origin: &HeaderValue, req: &RequestHead) -> bool {
        self.allowed_origins_fns
            .iter()
            .any(|origin_fn| (origin_fn.boxed_fn)(origin, req))
    }

    /// Only called if origin exists and always after it's validated.
    pub(crate) fn access_control_allow_origin(
        &self,
        req: &RequestHead,
    ) -> Option<HeaderValue> {
        let origin = req.headers().get(header::ORIGIN);

        match self.allowed_origins {
            AllOrSome::All => {
                if self.send_wildcard {
                    Some(HeaderValue::from_static("*"))
                } else {
                    // see note below about why `.cloned()` is correct
                    origin.cloned()
                }
            }

            AllOrSome::Some(_) => {
                // since origin (if it exists) is known to be allowed if this method is called
                // then cloning the option is all that is required to be used as an echoed back
                // header value (or omitted if None)
                origin.cloned()
            }
        }
    }

    /// Use in preflight checks and therefore operates on header list in
    /// `Access-Control-Request-Headers` not the actual header set.
    pub(crate) fn validate_allowed_method(
        &self,
        req: &RequestHead,
    ) -> Result<(), CorsError> {
        // extract access control header and try to parse as method
        let request_method = req
            .headers()
            .get(header::ACCESS_CONTROL_REQUEST_METHOD)
            .map(header_value_try_into_method);

        match request_method {
            // method valid and allowed
            Some(Some(method)) if self.allowed_methods.contains(&method) => Ok(()),

            // method valid but not allowed
            Some(Some(_)) => Err(CorsError::MethodNotAllowed),

            // method invalid
            Some(_) => Err(CorsError::BadRequestMethod),

            // method missing
            None => Err(CorsError::MissingRequestMethod),
        }
    }

    pub(crate) fn validate_allowed_headers(
        &self,
        req: &RequestHead,
    ) -> Result<(), CorsError> {
        // return early if all headers are allowed or get ref to allowed origins set
        #[allow(clippy::mutable_key_type)]
        let allowed_headers = match &self.allowed_headers {
            AllOrSome::All => return Ok(()),
            AllOrSome::Some(allowed_headers) => allowed_headers,
        };

        // extract access control header as string
        // header format should be comma separated header names
        let request_headers = req
            .headers()
            .get(header::ACCESS_CONTROL_REQUEST_HEADERS)
            .map(|hdr| hdr.to_str());

        match request_headers {
            // header list is valid string
            Some(Ok(headers)) => {
                // the set is ephemeral we take care not to mutate the
                // inserted keys so this lint exception is acceptable
                #[allow(clippy::mutable_key_type)]
                let mut request_headers = HashSet::with_capacity(8);

                // try to convert each header name in the comma-separated list
                for hdr in headers.split(',') {
                    match hdr.trim().try_into() {
                        Ok(hdr) => request_headers.insert(hdr),
                        Err(_) => return Err(CorsError::BadRequestHeaders),
                    };
                }

                // header list must contain 1 or more header name
                if request_headers.is_empty() {
                    return Err(CorsError::BadRequestHeaders);
                }

                // request header list must be a subset of allowed headers
                if !request_headers.is_subset(allowed_headers) {
                    return Err(CorsError::HeadersNotAllowed);
                }

                Ok(())
            }

            // header list is not a string
            Some(Err(_)) => Err(CorsError::BadRequestHeaders),

            // header list missing
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use actix_web::{
        dev::Transform,
        http::{header, HeaderValue, Method, StatusCode},
        test::{self, TestRequest},
    };

    use crate::Cors;

    fn val_as_str(val: &HeaderValue) -> &str {
        val.to_str().unwrap()
    }

    #[actix_rt::test]
    async fn test_validate_not_allowed_origin() {
        let cors = Cors::default()
            .allowed_origin("https://www.example.com")
            .new_transform(test::ok_service())
            .await
            .unwrap();

        let req = TestRequest::get()
            .header(header::ORIGIN, "https://www.unknown.com")
            .header(header::ACCESS_CONTROL_REQUEST_HEADERS, "DNT")
            .to_srv_request();

        assert!(cors.inner.validate_origin(req.head()).is_err());
        assert!(cors.inner.validate_allowed_method(req.head()).is_err());
        assert!(cors.inner.validate_allowed_headers(req.head()).is_err());
    }

    #[actix_rt::test]
    async fn test_preflight() {
        let mut cors = Cors::default()
            .allow_any_origin()
            .send_wildcard()
            .max_age(3600)
            .allowed_methods(vec![Method::GET, Method::OPTIONS, Method::POST])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .new_transform(test::ok_service())
            .await
            .unwrap();

        let req = TestRequest::with_header("Origin", "https://www.example.com")
            .method(Method::OPTIONS)
            .header(header::ACCESS_CONTROL_REQUEST_HEADERS, "X-Not-Allowed")
            .to_srv_request();

        assert!(cors.inner.validate_allowed_method(req.head()).is_err());
        assert!(cors.inner.validate_allowed_headers(req.head()).is_err());
        let resp = test::call_service(&mut cors, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

        let req = TestRequest::with_header("Origin", "https://www.example.com")
            .header(header::ACCESS_CONTROL_REQUEST_METHOD, "put")
            .method(Method::OPTIONS)
            .to_srv_request();

        assert!(cors.inner.validate_allowed_method(req.head()).is_err());
        assert!(cors.inner.validate_allowed_headers(req.head()).is_ok());

        let req = TestRequest::with_header("Origin", "https://www.example.com")
            .header(header::ACCESS_CONTROL_REQUEST_METHOD, "POST")
            .header(
                header::ACCESS_CONTROL_REQUEST_HEADERS,
                "AUTHORIZATION,ACCEPT",
            )
            .method(Method::OPTIONS)
            .to_srv_request();

        let resp = test::call_service(&mut cors, req).await;
        assert_eq!(
            Some(&b"*"[..]),
            resp.headers()
                .get(header::ACCESS_CONTROL_ALLOW_ORIGIN)
                .map(HeaderValue::as_bytes)
        );
        assert_eq!(
            Some(&b"3600"[..]),
            resp.headers()
                .get(header::ACCESS_CONTROL_MAX_AGE)
                .map(HeaderValue::as_bytes)
        );

        let hdr = resp
            .headers()
            .get(header::ACCESS_CONTROL_ALLOW_HEADERS)
            .map(val_as_str)
            .unwrap();
        assert!(hdr.contains("authorization"));
        assert!(hdr.contains("accept"));
        assert!(hdr.contains("content-type"));

        let methods = resp
            .headers()
            .get(header::ACCESS_CONTROL_ALLOW_METHODS)
            .unwrap()
            .to_str()
            .unwrap();
        assert!(methods.contains("POST"));
        assert!(methods.contains("GET"));
        assert!(methods.contains("OPTIONS"));

        Rc::get_mut(&mut cors.inner).unwrap().preflight = false;

        let req = TestRequest::with_header("Origin", "https://www.example.com")
            .header(header::ACCESS_CONTROL_REQUEST_METHOD, "POST")
            .header(
                header::ACCESS_CONTROL_REQUEST_HEADERS,
                "AUTHORIZATION,ACCEPT",
            )
            .method(Method::OPTIONS)
            .to_srv_request();

        let resp = test::call_service(&mut cors, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
