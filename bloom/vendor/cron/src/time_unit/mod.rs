mod days_of_month;
mod days_of_week;
mod hours;
mod minutes;
mod months;
mod seconds;
mod years;

pub use self::days_of_month::DaysOfMonth;
pub use self::days_of_week::DaysOfWeek;
pub use self::hours::Hours;
pub use self::minutes::Minutes;
pub use self::months::Months;
pub use self::seconds::Seconds;
pub use self::years::Years;

use error::*;
use schedule::{Ordinal, OrdinalSet, Specifier};
use std::borrow::Cow;
use std::collections::btree_set;
use std::iter;
use std::ops::RangeBounds;

pub struct OrdinalIter<'a> {
    set_iter: btree_set::Iter<'a, Ordinal>,
}

impl<'a> Iterator for OrdinalIter<'a> {
    type Item = Ordinal;
    fn next(&mut self) -> Option<Ordinal> {
        self.set_iter.next().map(|ordinal| ordinal.clone()) // No real expense; Ordinal is u32: Copy
    }
}

pub struct OrdinalRangeIter<'a> {
    range_iter: btree_set::Range<'a, Ordinal>,
}

impl<'a> Iterator for OrdinalRangeIter<'a> {
    type Item = Ordinal;
    fn next(&mut self) -> Option<Ordinal> {
        self.range_iter.next().map(|ordinal| ordinal.clone()) // No real expense; Ordinal is u32: Copy
    }
}

/// Methods exposing a schedule's configured ordinals for each individual unit of time.
/// # Example
/// ```
/// use cron::{Schedule,TimeUnitSpec};
/// use std::collections::Bound::{Included,Excluded};
/// use std::str::FromStr;
///
/// let expression = "* * * * * * 2015-2044";
/// let schedule = Schedule::from_str(expression).expect("Failed to parse expression.");
///
/// // Membership
/// assert_eq!(true, schedule.years().includes(2031));
/// assert_eq!(false, schedule.years().includes(1969));
///
/// // Number of years specified
/// assert_eq!(30, schedule.years().count());
///
/// // Iterator
/// let mut years_iter = schedule.years().iter();
/// assert_eq!(Some(2015), years_iter.next());
/// assert_eq!(Some(2016), years_iter.next());
/// // ...
///
/// // Range Iterator
/// let mut five_year_plan = schedule.years().range((Included(2017), Excluded(2017 + 5)));
/// assert_eq!(Some(2017), five_year_plan.next());
/// assert_eq!(Some(2018), five_year_plan.next());
/// assert_eq!(Some(2019), five_year_plan.next());
/// assert_eq!(Some(2020), five_year_plan.next());
/// assert_eq!(Some(2021), five_year_plan.next());
/// assert_eq!(None, five_year_plan.next());
/// ```
pub trait TimeUnitSpec {
    /// Returns true if the provided ordinal was included in the schedule spec for the unit of time
    /// being described.
    /// # Example
    /// ```
    /// use cron::{Schedule,TimeUnitSpec};
    /// use std::str::FromStr;
    ///
    /// let expression = "* * * * * * 2015-2044";
    /// let schedule = Schedule::from_str(expression).expect("Failed to parse expression.");
    ///
    /// // Membership
    /// assert_eq!(true, schedule.years().includes(2031));
    /// assert_eq!(false, schedule.years().includes(2004));
    /// ```
    fn includes(&self, ordinal: Ordinal) -> bool;

    /// Provides an iterator which will return each included ordinal for this schedule in order from
    /// lowest to highest.
    /// # Example
    /// ```
    /// use cron::{Schedule,TimeUnitSpec};
    /// use std::str::FromStr;
    ///
    /// let expression = "* * * * 5-8 * *";
    /// let schedule = Schedule::from_str(expression).expect("Failed to parse expression.");
    ///
    /// // Iterator
    /// let mut summer = schedule.months().iter();
    /// assert_eq!(Some(5), summer.next());
    /// assert_eq!(Some(6), summer.next());
    /// assert_eq!(Some(7), summer.next());
    /// assert_eq!(Some(8), summer.next());
    /// assert_eq!(None, summer.next());
    /// ```
    fn iter<'a>(&'a self) -> OrdinalIter<'a>;

    /// Provides an iterator which will return each included ordinal within the specified range.
    /// # Example
    /// ```
    /// use cron::{Schedule,TimeUnitSpec};
    /// use std::collections::Bound::{Included,Excluded};
    /// use std::str::FromStr;
    ///
    /// let expression = "* * * 1,15 * * *";
    /// let schedule = Schedule::from_str(expression).expect("Failed to parse expression.");
    ///
    /// // Range Iterator
    /// let mut mid_month_paydays = schedule.days_of_month().range((Included(10), Included(20)));
    /// assert_eq!(Some(15), mid_month_paydays.next());
    /// assert_eq!(None, mid_month_paydays.next());
    /// ```
    fn range<'a, R>(&'a self, range: R) -> OrdinalRangeIter<'a>
    where
        R: RangeBounds<Ordinal>;

    /// Returns the number of ordinals included in the associated schedule
    /// # Example
    /// ```
    /// use cron::{Schedule,TimeUnitSpec};
    /// use std::str::FromStr;
    ///
    /// let expression = "* * * 1,15 * * *";
    /// let schedule = Schedule::from_str(expression).expect("Failed to parse expression.");
    ///
    /// assert_eq!(2, schedule.days_of_month().count());
    /// ```
    fn count(&self) -> u32;
}

impl<T> TimeUnitSpec for T
where
    T: TimeUnitField,
{
    fn includes(&self, ordinal: Ordinal) -> bool {
        self.ordinals().contains(&ordinal)
    }
    fn iter<'a>(&'a self) -> OrdinalIter<'a> {
        OrdinalIter {
            set_iter: TimeUnitField::ordinals(self).iter(),
        }
    }
    fn range<'a, R>(&'a self, range: R) -> OrdinalRangeIter<'a>
    where
        R: RangeBounds<Ordinal>,
    {
        OrdinalRangeIter {
            range_iter: TimeUnitField::ordinals(self).range(range),
        }
    }
    fn count(&self) -> u32 {
        self.ordinals().len() as u32
    }
}

pub trait TimeUnitField
where
    Self: Sized,
{
    fn from_ordinal_set(ordinal_set: OrdinalSet) -> Self;
    fn name() -> Cow<'static, str>;
    fn inclusive_min() -> Ordinal;
    fn inclusive_max() -> Ordinal;
    fn ordinals(&self) -> &OrdinalSet;
    fn from_ordinal(ordinal: Ordinal) -> Self {
        Self::from_ordinal_set(iter::once(ordinal).collect())
    }
    fn supported_ordinals() -> OrdinalSet {
        (Self::inclusive_min()..Self::inclusive_max() + 1).collect()
    }
    fn all() -> Self {
        Self::from_ordinal_set(Self::supported_ordinals())
    }
    fn ordinal_from_name(name: &str) -> Result<Ordinal> {
        bail!(ErrorKind::Expression(format!(
            "The '{}' field does not support using names. '{}' \
             specified.",
            Self::name(),
            name
        )))
    }
    fn validate_ordinal(ordinal: Ordinal) -> Result<Ordinal> {
        //println!("validate_ordinal for {} => {}", Self::name(), ordinal);
        match ordinal {
            i if i < Self::inclusive_min() => bail!(ErrorKind::Expression(format!(
                "{} must be greater than or equal to {}. ('{}' \
                 specified.)",
                Self::name(),
                Self::inclusive_min(),
                i
            ))),
            i if i > Self::inclusive_max() => bail!(ErrorKind::Expression(format!(
                "{} must be less than {}. ('{}' specified.)",
                Self::name(),
                Self::inclusive_max(),
                i
            ))),
            i => Ok(i),
        }
    }

    fn ordinals_from_specifier(specifier: &Specifier) -> Result<OrdinalSet> {
        use self::Specifier::*;
        //println!("ordinals_from_specifier for {} => {:?}", Self::name(), specifier);
        match *specifier {
            All => Ok(Self::supported_ordinals()),
            Point(ordinal) => Ok((&[ordinal]).iter().cloned().collect()),
            NamedPoint(ref name) => Ok((&[Self::ordinal_from_name(name)?])
                .iter()
                .cloned()
                .collect()),
            Period(start, step) => {
                let start = Self::validate_ordinal(start)?;
                Ok((start..Self::inclusive_max() + 1)
                    .step_by(step as usize)
                    .collect())
            }
            Range(start, end) => {
                match (Self::validate_ordinal(start), Self::validate_ordinal(end)) {
                    (Ok(start), Ok(end)) if start <= end => Ok((start..end + 1).collect()),
                    _ => bail!(ErrorKind::Expression(format!(
                        "Invalid range for {}: {}-{}",
                        Self::name(),
                        start,
                        end
                    ))),
                }
            }
            NamedRange(ref start_name, ref end_name) => {
                let start = Self::ordinal_from_name(start_name)?;
                let end = Self::ordinal_from_name(end_name)?;
                match (Self::validate_ordinal(start), Self::validate_ordinal(end)) {
                    (Ok(start), Ok(end)) if start <= end => Ok((start..end + 1).collect()),
                    _ => bail!(ErrorKind::Expression(format!(
                        "Invalid named range for {}: {}-{}",
                        Self::name(),
                        start_name,
                        end_name
                    ))),
                }
            }
        }
    }
}
