use schedule::{Ordinal, OrdinalSet};
use std::borrow::Cow;
use time_unit::TimeUnitField;

#[derive(Clone, Debug)]
pub struct Seconds(OrdinalSet);

impl TimeUnitField for Seconds {
    fn from_ordinal_set(ordinal_set: OrdinalSet) -> Self {
        Seconds(ordinal_set)
    }
    fn name() -> Cow<'static, str> {
        Cow::from("Seconds")
    }
    fn inclusive_min() -> Ordinal {
        0
    }
    fn inclusive_max() -> Ordinal {
        59
    }
    fn ordinals(&self) -> &OrdinalSet {
        &self.0
    }
}
