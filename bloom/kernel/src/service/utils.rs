use super::{DecodedSessionToken, Service};
use crate::{
    domain::{files, inbox},
    entities::{Session, User},
    errors::kernel::Error,
    Actor,
};
use std::sync::Arc;
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

    pub fn self_hosted(&self) -> bool {
        self.config.self_hosted
    }

    pub fn inject_missing_dependencies(
        &self,
        files_service: Arc<dyn files::Service>,
        inbox_service: Arc<dyn inbox::Service>,
    ) -> () {
        // this unsafe block is safe because as this method is called only when setting up services
        // the method is never called concurrently
        let selff = unsafe { &mut *(self as *const Service as *mut Service) };
        selff.files_service = Some(files_service);
        selff.inbox_service = Some(inbox_service);
    }

    pub async fn render_markdown(&self, markdown: &str) -> Result<String, crate::Error> {
        todo!();
    }
}
