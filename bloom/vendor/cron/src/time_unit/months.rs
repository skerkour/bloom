use error::*;
use schedule::{Ordinal, OrdinalSet};
use std::borrow::Cow;
use time_unit::TimeUnitField;

#[derive(Clone, Debug)]
pub struct Months(OrdinalSet);

impl TimeUnitField for Months {
    fn from_ordinal_set(ordinal_set: OrdinalSet) -> Self {
        Months(ordinal_set)
    }
    fn name() -> Cow<'static, str> {
        Cow::from("Months")
    }
    fn inclusive_min() -> Ordinal {
        1
    }
    fn inclusive_max() -> Ordinal {
        12
    }
    fn ordinal_from_name(name: &str) -> Result<Ordinal> {
        //TODO: Use phf crate
        let ordinal = match name.to_lowercase().as_ref() {
            "jan" | "january" => 1,
            "feb" | "february" => 2,
            "mar" | "march" => 3,
            "apr" | "april" => 4,
            "may" => 5,
            "jun" | "june" => 6,
            "jul" | "july" => 7,
            "aug" | "august" => 8,
            "sep" | "september" => 9,
            "oct" | "october" => 10,
            "nov" | "november" => 11,
            "dec" | "december" => 12,
            _ => bail!(ErrorKind::Expression(format!(
                "'{}' is not a valid month name.",
                name
            ))),
        };
        Ok(ordinal)
    }
    fn ordinals(&self) -> &OrdinalSet {
        &self.0
    }
}
