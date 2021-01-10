use crate::{Error, Service};
use stdx::url::Url;

impl Service {
    pub fn validate_and_parse_event_url(&self, _url_str: &str) -> Result<Url, Error> {
        todo!();
    }

    pub fn validate_event_screen_width(&self, screen_width: i64) -> Result<(), Error> {
        todo!();
    }

    pub fn validate_event_screen_height(&self, screen_height: i64) -> Result<(), Error> {
        todo!();
    }
}
