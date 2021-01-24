use crate::entities::{Session, User};
use actix_web::{dev, error::ErrorInternalServerError, Error, FromRequest, HttpRequest};
use stdx::futures::future::{err, ok, Ready};
use stdx::{log::error, uuid::Uuid};

#[derive(Clone)]
pub enum Actor {
    User { user: User, session: Session },
    Anonymous(Uuid),
    None,
}

impl Actor {
    pub fn is_none(&self) -> bool {
        match self {
            Actor::None => true,
            _ => false,
        }
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }
}

/// actix-web Actor extractor
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

#[cfg(test)]
mod tests {
    use super::Actor;
    use stdx::uuid::Uuid;

    #[test]
    fn actor_is_none() {
        let actor = Actor::None;
        assert_eq!(actor.is_none(), true);

        let actor = Actor::Anonymous(Uuid::new_v4());
        assert_eq!(actor.is_none(), false);
    }

    #[test]
    fn actor_is_some() {
        let actor = Actor::None;
        assert_eq!(actor.is_some(), false);

        let actor = Actor::Anonymous(Uuid::new_v4());
        assert_eq!(actor.is_some(), true);
    }
}
