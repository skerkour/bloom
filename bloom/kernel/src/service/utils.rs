use super::{DecodedSessionToken, Service};
use crate::{
    entities::{Session, User},
    errors::kernel::Error,
    Actor,
};
use stdx::uuid::Uuid;

impl Service {
    pub async fn new_session(&self, _user_id: Uuid) -> Result<Session, crate::Error> {
        unimplemented!(); // TODO
    }

    pub fn current_user(&self, actor: Actor) -> Result<User, crate::Error> {
        match actor {
            Actor::User(user) => Ok(user),
            _ => Err(Error::AuthenticationRequired.into()),
        }
    }

    pub fn current_anonymous_id(&self, actor: Actor) -> Result<Uuid, crate::Error> {
        match actor {
            Actor::Anonymous(anonymous_id) => Ok(anonymous_id),
            _ => Err(Error::AuthenticationRequired.into()),
        }
    }

    pub fn decode_session_token(&self, _token: String) -> Result<DecodedSessionToken, crate::Error> {
        unimplemented!(); // TODO
    }
}
