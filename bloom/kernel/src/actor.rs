use crate::entities;
use actix_web::{dev, error::ErrorInternalServerError, Error, FromRequest, HttpRequest};
use stdx::futures::future::{err, ok, Ready};
use stdx::{log::error, uuid::Uuid};

#[derive(Clone)]
pub enum Actor {
    User(entities::User),
    Anonymous(Uuid),
    None,
}

/// Actor extractor
impl FromRequest for Actor {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        if let Some(actor) = req.extensions().get::<Actor>() {
            ok(actor.clone())
        } else {
            error!("middlewares/auth: auth is missing");
            err(ErrorInternalServerError("auth is missing"))
        }
    }
}
