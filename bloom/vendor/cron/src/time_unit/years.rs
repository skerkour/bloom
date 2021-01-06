use schedule::{Ordinal, OrdinalSet};
use std::borrow::Cow;
use time_unit::TimeUnitField;

#[derive(Clone, Debug)]
pub struct Years(OrdinalSet);

impl TimeUnitField for Years {
    fn from_ordinal_set(ordinal_set: OrdinalSet) -> Self {
        Years(ordinal_set)
    }
    fn name() -> Cow<'static, str> {
        Cow::from("Years")
    }

    // TODO: Using the default impl, this will make a set w/100+ items each time "*" is used.
    // This is obviously suboptimal.
    fn inclusive_min() -> Ordinal {
        1970
    }
    fn inclusive_max() -> Ordinal {
        2100
    }
    fn ordinals(&self) -> &OrdinalSet {
        &self.0
    }
}
