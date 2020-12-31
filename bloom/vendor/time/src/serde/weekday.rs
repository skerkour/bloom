use crate::Weekday::*;
use standback::convert::TryFrom;

// 1-indexed day from Monday
#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct Weekday(u8);

impl From<crate::Weekday> for Weekday {
    fn from(original: crate::Weekday) -> Self {
        Self(original.iso_weekday_number())
    }
}

impl TryFrom<Weekday> for crate::Weekday {
    type Error = &'static str;

    fn try_from(original: Weekday) -> Result<Self, Self::Error> {
        match original {
            Weekday(1) => Ok(Monday),
            Weekday(2) => Ok(Tuesday),
            Weekday(3) => Ok(Wednesday),
            Weekday(4) => Ok(Thursday),
            Weekday(5) => Ok(Friday),
            Weekday(6) => Ok(Saturday),
            Weekday(7) => Ok(Sunday),
            _ => Err("invalid value"),
        }
    }
}
