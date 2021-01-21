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
        todo!(); // TODO
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
        todo!(); // TODO
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
        let unsafe_html = stdx::markdown::render_markdown(markdown);
        let safe_html = self.xss.sanitize(&unsafe_html);
        Ok(safe_html)
    }

    pub fn format_code_hyphen(&self, code: String) -> String {
        let mut ret = String::with_capacity(code.len() + 4);

        for (i, c) in code.chars().into_iter().enumerate() {
            if i != 0 && i % 4 == 0 {
                ret.push('-');
            }
            ret.push(c)
        }

        ret
    }

    pub fn format_code_html(&self, code: String) -> String {
        let mut ret = String::with_capacity(code.len() * 45 + 15);
        ret.push_str("span");

        for c in code.chars().into_iter() {
            if c.is_alphabetic() || c == '-' {
                ret.push(c);
            } else if c.is_numeric() {
                ret.push_str(r#"<span style="color: red">"#);
                ret.push(c);
                ret.push_str("</span>");
            } else {
                ret.push_str(r#"<span style="color: blue">"#);
                ret.push(c);
                ret.push_str("</span>");
            }
        }

        ret
    }

    pub fn contact_url(&self) -> String {
        format!("{}/contact", &self.config.base_url)
    }

    pub fn namespace_url(&self, namespace: &str) -> String {
        format!("{}/{}", &self.config.base_url, namespace)
    }

    pub fn group_invitations_url(&self) -> String {
        format!("{}/preferences/invitations", &self.config.base_url)
    }
}
