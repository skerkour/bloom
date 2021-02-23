use super::{DecodedSessionToken, Service};
use crate::{
    consts::{self, BillingPlan},
    domain::{files, inbox},
    entities::{Session, User},
    errors::kernel::Error,
    Actor,
};
use consts::{STORAGE_FREE, STORAGE_PRO, STORAGE_STARTER};
use std::sync::Arc;
use stdx::{
    base64, crypto,
    sync::threadpool::spawn_blocking,
    uuid::{self, Uuid},
};

impl Service {
    pub fn current_user(&self, actor: Actor) -> Result<User, crate::Error> {
        match actor {
            Actor::User {
                user, ..
            } => Ok(user),
            _ => Err(Error::AuthenticationRequired.into()),
        }
    }

    pub fn current_user_and_session(&self, actor: Actor) -> Result<(User, Session), crate::Error> {
        match actor {
            Actor::User {
                user,
                session,
            } => Ok((user, session)),
            _ => Err(Error::AuthenticationRequired.into()),
        }
    }

    pub fn current_anonymous_id(&self, actor: Actor) -> Result<Uuid, crate::Error> {
        match actor {
            Actor::Anonymous(anonymous_id) => Ok(anonymous_id),
            _ => Err(Error::AuthenticationRequired.into()),
        }
    }

    pub fn decode_session_token(&self, token: String) -> Result<DecodedSessionToken, crate::Error> {
        if token.len() > 1024 {
            return Err(Error::InvalidSession.into());
        }

        let token = base64::decode(token)?;

        if token.len() != crypto::KEY_SIZE_512 + uuid::SIZE {
            return Err(Error::InvalidSession.into());
        }

        let session_id_bytes = &token[..uuid::SIZE];
        let secret: Vec<u8> = token[uuid::SIZE..].into();
        let session_id = Uuid::from_slice(session_id_bytes).map_err(|_| Error::InvalidSession)?;

        Ok(DecodedSessionToken {
            session_id,
            secret,
        })
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
        if markdown.len() > consts::MARKDOWN_MAX_SIZE {
            return Err(Error::MarkdownIsTooLong.into());
        }

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
        ret.push_str("<span>");

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

        ret.push_str("</span>");

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

    pub fn base_url(&self) -> String {
        self.config.base_url.clone()
    }

    pub async fn verify_session_secret(&self, session: &Session, secret: Vec<u8>) -> Result<(), crate::Error> {
        let hash = self.hash_session(session.id, secret).await?;

        if !crypto::constant_time_compare(&hash, &session.secret_hash) {
            return Err(Error::InvalidSession.into());
        }

        Ok(())
    }

    pub async fn hash_session(&self, session_id: Uuid, secret: Vec<u8>) -> Result<Vec<u8>, crate::Error> {
        let hash =
            spawn_blocking(move || crypto::derive_key_from_key(&secret, session_id.as_bytes(), crypto::KEY_SIZE_512))
                .await?
                .map_err(|err| crate::Error::Internal(err.to_string()))?;

        Ok(hash)
    }

    pub async fn decode_and_validate_anonymous_token(&self, token: String) -> Result<Uuid, crate::Error> {
        let anonymous_id = Uuid::parse_str(&token)?;
        Ok(anonymous_id)
    }

    pub fn encode_session_token(&self, session_id: Uuid, secret: Vec<u8>) -> Result<String, crate::Error> {
        let session_id_bytes = session_id.as_bytes();
        let mut token_bytes: Vec<u8> = session_id_bytes[..].into();
        token_bytes.extend(secret);
        Ok(base64::encode(token_bytes))
    }

    pub fn get_storage_for_plan(&self, plan: BillingPlan) -> i64 {
        match plan {
            BillingPlan::Free => STORAGE_FREE,
            BillingPlan::Starter => STORAGE_STARTER,
            BillingPlan::Pro => STORAGE_PRO,
            // BillingPlan::Ultra => STORAGE_ULTRA,
        }
    }

    pub fn get_avatar_storage_key(&self, avatar_id: &str) -> String {
        format!("/avatars/{}", avatar_id)
    }

    pub fn get_avatar_url(&self, avatar_id: Option<&String>) -> String {
        let base_url = self.base_url();

        match avatar_id {
            Some(avatar_id) => format!("{}/avatars/{}", &base_url, avatar_id),
            None => format!("{}{}", &base_url, consts::DEFAULT_AVATAR.to_string()),
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::{config::test::load_test_config, service::test::new_service_mock};
    use stdx::tokio;

    #[tokio::test]
    async fn format_code_html() {
        let config = load_test_config();
        let service = new_service_mock(config).await;

        let code = "1a2a3a4a5a6a#|";
        let expected_html = concat!(
            "<span>",
            r#"<span style="color: red">1</span>"#,
            "a",
            r#"<span style="color: red">2</span>"#,
            "a",
            r#"<span style="color: red">3</span>"#,
            "a",
            r#"<span style="color: red">4</span>"#,
            "a",
            r#"<span style="color: red">5</span>"#,
            "a",
            r#"<span style="color: red">6</span>"#,
            "a",
            r##"<span style="color: blue">#</span>"##,
            r#"<span style="color: blue">|</span>"#,
            "</span>"
        );

        let res = service.format_code_html(code.to_string());

        assert_eq!(res, expected_html);
    }

    #[tokio::test]
    async fn format_code_hyphen() {
        let config = load_test_config();
        let service = new_service_mock(config).await;

        let codes = ["1a2a3a4a5a6a", "1a2a3a4a5a6a1a2a3a4a5a6a", "1a2a"];
        let expected = ["1a2a-3a4a-5a6a", "1a2a-3a4a-5a6a-1a2a-3a4a-5a6a", "1a2a"];

        for (i, code) in codes.iter().enumerate() {
            let res = service.format_code_hyphen(code.to_string());
            assert_eq!(res, expected[i]);
        }
    }
}
