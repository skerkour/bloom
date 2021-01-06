use error::*;
use schedule::{Ordinal, OrdinalSet};
use std::borrow::Cow;
use time_unit::TimeUnitField;

#[derive(Clone, Debug)]
pub struct DaysOfWeek(OrdinalSet);

impl TimeUnitField for DaysOfWeek {
    fn from_ordinal_set(ordinal_set: OrdinalSet) -> Self {
        DaysOfWeek(ordinal_set)
    }
    fn name() -> Cow<'static, str> {
        Cow::from("Days of Week")
    }
    fn inclusive_min() -> Ordinal {
        1
    }
    fn inclusive_max() -> Ordinal {
        7
    }
    fn ordinal_from_name(name: &str) -> Result<Ordinal> {
        //TODO: Use phf crate
        let ordinal = match name.to_lowercase().as_ref() {
            "sun" | "sunday" => 1,
            "mon" | "monday" => 2,
            "tue" | "tues" | "tuesday" => 3,
            "wed" | "wednesday" => 4,
            "thu" | "thurs" | "thursday" => 5,
            "fri" | "friday" => 6,
            "sat" | "saturday" => 7,
            _ => bail!(ErrorKind::Expression(format!(
                "'{}' is not a valid day of the week.",
                name
            ))),
        };
        Ok(ordinal)
    }
    fn ordinals(&self) -> &OrdinalSet {
        &self.0
    }
}
