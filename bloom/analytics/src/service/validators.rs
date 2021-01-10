use crate::{consts, Error, Service};
use std::collections::HashMap;
use stdx::{log::debug, url::Url};

impl Service {
    pub fn validate_and_parse_event_url(&self, url_str: &str) -> Result<Url, Error> {
        if url_str.len() > consts::EVENT_URL_MAX_LENGTH {
            return Err(Error::EventNotValid);
        }
        let url = Url::parse(url_str).map_err(|err| {
            debug!("analytics.validate_and_parse_event_url: parsing url: {}", &err);
            return Error::EventNotValid;
        })?;
        Ok(url)
    }

    pub fn validate_event_screen_width(&self, screen_width: i64) -> Result<(), Error> {
        if screen_width < 0 || screen_width > consts::MAX_SCREEN_WIDTH {
            return Err(Error::EventNotValid);
        }
        Ok(())
    }

    pub fn validate_event_screen_height(&self, screen_height: i64) -> Result<(), Error> {
        if screen_height < 0 || screen_height > consts::MAX_SCREEN_HEIGHT {
            return Err(Error::EventNotValid);
        }
        Ok(())
    }

    // TODO: validate total properties size
    pub fn validate_event_properties(&self, properties: &HashMap<String, String>) -> Result<(), Error> {
        for value in properties.values() {
            if value.len() > consts::EVENT_PROPERTY_MAX_LENGTH {
                return Err(Error::EventNotValid);
            }
        }
        if properties.len() > 50 {
            return Err(Error::EventNotValid);
        }
        Ok(())
    }
}
