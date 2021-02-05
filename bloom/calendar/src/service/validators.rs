use stdx::chrono::{DateTime, Utc};

use crate::{consts, Error, Service};

impl Service {
    pub fn validate_event_title(&self, title: &str) -> Result<(), Error> {
        if title.len() > consts::EVENT_TITLE_MAX_LENGTH {
            return Err(Error::EventTitleIsTooLong);
        }

        if title.contains('\n') {
            return Err(Error::EventTitleIsInvalid);
        }

        Ok(())
    }

    pub fn validate_event_description(&self, descirption: &str) -> Result<(), Error> {
        if descirption.len() > consts::EVENT_DESCRIPTION_MAX_LENGTH {
            return Err(Error::EventDescriptionIsTooLong);
        }

        Ok(())
    }

    pub fn validate_event_location(&self, location: &str) -> Result<(), Error> {
        if location.len() > consts::EVENT_LOCATION_MAX_LENGTH {
            return Err(Error::EventLocationIsTooLong);
        }

        if location.contains('\n') {
            return Err(Error::EventLocationIsInvalid);
        }

        Ok(())
    }

    pub fn validate_event_dates(&self, start_at: DateTime<Utc>, end_at: DateTime<Utc>) -> Result<(), Error> {
        if end_at < start_at {
            return Err(Error::EventEndDateCantBeBeforeStartDate);
        }

        Ok(())
    }
}
