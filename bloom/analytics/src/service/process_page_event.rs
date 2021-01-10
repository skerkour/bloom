use crate::{consts, entities};

use super::{FindOrCreateVisitorInput, Service};
use kernel::domain::analytics::events::PageEvent;
use stdx::{
    chrono::{Duration, Utc},
    ulid::Ulid,
};

impl Service {
    // TODO: ignore bots
    pub async fn process_page_event(&self, event: PageEvent) -> Result<(), kernel::Error> {
        let now_less_one_month = Utc::now() - Duration::days(31);
        let now = Utc::now();

        let timestamp = event.timestamp;
        let received_at = event.received_at;
        let sent_at = event.sent_at;
        if timestamp > now
            || sent_at > now
            || received_at > now
            || timestamp < now_less_one_month
            || sent_at < now_less_one_month
            || received_at < now_less_one_month
        {
            // discarding faulty event
            return Ok(());
        }

        let screen_width = event.screen_width;
        match self.validate_event_screen_width(screen_width) {
            Ok(_) => {}
            Err(_) => return Ok(()),
        };

        let screen_height = event.screen_height;
        match self.validate_event_screen_height(screen_height) {
            Ok(_) => {}
            Err(_) => return Ok(()),
        };

        let url = event.url.trim().to_string();
        let parsed_url = match self.validate_and_parse_event_url(&url) {
            Ok(url) => url,
            Err(_) => return Ok(()),
        };

        let mut path = parsed_url.path().trim().trim_end_matches('/').to_string();
        if path == "" {
            path = "/".to_string();
        }

        let mut page_name = event.name.trim().to_string();
        page_name.truncate(consts::PAGE_NAME_MAX_LENGTH);

        let mut referrer = event.referrer.trim().to_string();
        referrer.truncate(consts::REFERRER_MAX_LENGTH);

        let device_type = self.device_type_from_screen_size(screen_width, screen_height);

        let mut user_agent = event.user_agent.trim().to_string();
        user_agent.truncate(consts::USER_AGENT_MAX_LENGTH);

        // TODO
        // parsedUserAgent := service.userAgentParser.Parse(userAgent)

        let find_or_create_input = FindOrCreateVisitorInput {
            anonymous_id: event.anonymous_id,
            namespace_id: event.namespace_id,
        };
        let visitor = self.find_or_create_visitor(&self.db, find_or_create_input).await?;

        let event = entities::PageEvent {
            id: Ulid::new().into(),
            created_at: now,
            timestamp,
            sent_at,
            received_at,
            page_name,
            url,
            user_agent,
            referrer,
            device_type,
            country: String::new(),         // TODO (from ip)
            country_code: String::new(),    // TODO (from ip)
            os_name: String::new(),         // TODO (from user_agent)
            os_version: String::new(),      // TODO (from user_agent)
            browser_name: String::new(),    // TODO (from user_agent)
            browser_version: String::new(), // TODO (from user_agent)
            path,
            screen_width,
            screen_height,
            visitor_id: visitor.id,
            namespace_id: event.namespace_id,
        };
        self.repo.create_page_event(&self.db, &event).await?;

        Ok(())
    }
}
