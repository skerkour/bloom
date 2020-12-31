// Because time (indirectly) depends on this crate, we can't depend on time
// here. As such, we need to copy the functions over. If/when proc macros can be
// declared in the same crate, this will no longer be necessary.

mod date;

pub(crate) use date::{days_in_year, days_in_year_month, weeks_in_year, Date};

#[derive(PartialEq)]
pub(crate) enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Weekday {
    pub(crate) const fn iso_weekday_number(self) -> u8 {
        self as u8 + 1
    }
}
