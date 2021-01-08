use super::{DecodedSessionToken, Service};
use crate::{
    entities::{Session, User},
    errors::kernel::Error,
};
use stdx::uuid::Uuid;

impl Service {
    pub async fn new_session(&self, _user_id: Uuid) -> Result<Session, crate::Error> {
        unimplemented!(); // TODO
    }

    pub fn current_user(&self, actor: Option<User>) -> Result<User, crate::Error> {
        actor.ok_or(Error::AuthenticationRequired.into())
    }

    pub fn decode_session_token(&self, _token: String) -> Result<DecodedSessionToken, crate::Error> {
        unimplemented!(); // TODO
    }
}
