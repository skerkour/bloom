use super::api::Response;
use crate::Error;
use actix_web::{http::StatusCode, web::HttpResponse, ResponseError};

impl ResponseError for Error {
    // builds the actual response to send back when an error occurs
    fn error_response(&self) -> HttpResponse {
        let res = Response::<()>::err(self.clone());
        HttpResponse::build(self.status_code()).json(res)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Error::InvalidArgument(_) => StatusCode::BAD_REQUEST,      // 400
            Error::AuthenticationRequired => StatusCode::UNAUTHORIZED, // 401
            Error::PermissionDenied(_) => StatusCode::FORBIDDEN,       // 403
            Error::NotFound(_) => StatusCode::NOT_FOUND,               // 404
            Error::AlreadyExists(_) => StatusCode::CONFLICT,           // 409
            Error::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,   // 500
        }
    }
}
