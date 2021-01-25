use actix_web::{web::Json, HttpRequest, HttpResponse, Responder};
use futures_util::future::Ready;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T: Serialize> {
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<Vec<Error>>,
}

impl<T: Serialize> Response<T> {
    pub fn ok(data: T) -> Response<T> {
        return Response {
            data: Some(data),
            errors: None,
        };
    }

    pub fn err(err: crate::Error) -> Response<()> {
        match &err {
            err @ crate::Error::Internal(_) => {
                sentry::capture_error(err);
            }
            _ => {}
        }

        return Response::<()> {
            data: None,
            errors: Some(vec![err.into()]),
        };
    }
}

impl<T: Serialize> Responder for Response<T> {
    type Error = actix_web::error::Error;
    type Future = Ready<Result<HttpResponse, actix_web::error::Error>>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        Json(self).respond_to(req)
    }
}

const EXTENSION_KEY_CODE: &str = "code";
const CODE_NOT_FOUND: &str = "NOT_FOUND";
const CODE_INTERNAL: &str = "INTERNAL";

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Error {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<HashMap<String, String>>,
    // Path       ast.Path               `json:"path,omitempty"`
    // Locations  []Location             `json:"locations,omitempty"`
    // Extensions map[string]interface{} `json:"extensions,omitempty"`
    // Rule       string                 `json:"-"`
}

// TODO: match different error types
impl std::convert::From<crate::Error> for Error {
    fn from(err: crate::Error) -> Self {
        match err {
            crate::Error::NotFound(err) => {
                let mut extensions = HashMap::new();
                extensions.insert(EXTENSION_KEY_CODE.into(), CODE_NOT_FOUND.into());

                Error {
                    message: err.to_string(),
                    extensions: Some(extensions),
                }
            }
            crate::Error::Internal(_) => {
                let mut extensions = HashMap::new();
                extensions.insert(EXTENSION_KEY_CODE.into(), CODE_INTERNAL.into());

                Error {
                    message: err.to_string(),
                    extensions: Some(extensions),
                }
            }
            _ => Error {
                message: err.to_string(),
                extensions: None,
            },
        }
    }
}
