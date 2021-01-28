use crate::{
    error,
    format::parse::{parse, ParsedItems},
    internals,
    util::{days_in_year, days_in_year_month, is_leap_year, weeks_in_year},
    DeferredFormat, Duration, ParseResult, PrimitiveDateTime, Time, Weekday,
};
#[cfg(not(feature = "std"))]
use alloc::{
    borrow::ToOwned,
    string::{String, ToString},
};
use const_fn::const_fn;
use core::{
    cmp::{Ord, Ordering, PartialOrd},
    fmt::{self, Display},
    ops::{Add, AddAssign, Sub, SubAssign},
    time::Duration as StdDuration,
};
#[cfg(feature = "serde")]
use standback::convert::TryInto;
#[allow(unused_imports)]
use standback::prelude::*;

/// The minimum valid year.
pub(crate) const MIN_YEAR: i32 = -100_000;
/// The maximum valid year.
pub(crate) const MAX_YEAR: i32 = 100_000;

/// Floored division for integers. This differs from the default behavior, which
/// is truncation.
#[const_fn("1.46")]
pub(crate) const fn div_floor(a: i64, b: i64) -> i64 {
    let (quotient, remainder) = (a / b, a % b);

    if (remainder > 0 && b < 0) || (remainder < 0 && b > 0) {
        quotient - 1
    } else {
        quotient
    }
}

/// Calendar date.
///
/// Years between `-100_000` and `+100_000` inclusive are guaranteed to be
/// representable. Any values outside this range may have incidental support
/// that can change at any time without notice. If you need support outside this
/// range, please [file an issue](https://github.com/time-rs/time/issues/new)
/// with your use case.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(into = "crate::serde::Date"))]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Date {
    /// Bitpacked field containing both the year and ordinal.
    // |     xx     | xxxxxxxxxxxxxxxxxxxxx | xxxxxxxxx |
    // |   2 bits   |        21 bits        |  9 bits   |
    // | unassigned |         year          |  ordinal  |
    pub(crate) value: i32,
}

impl fmt::Debug for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.debug_struct("Date")
            .field("year", &self.year())
            .field("ordinal", &self.ordinal())
            .finish()
    }
}

#[cfg(feature = "serde")]
impl<'a> serde::Deserialize<'a> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        crate::serde::Date::deserialize(deserializer)?
            .try_into()
            .map_err(serde::de::Error::custom)
    }
}

impl Date {
    /// Create a `Date` from the year, month, and day.
    ///
    /// ```rust
    /// # #![allow(deprecated)]
    /// # use time::{Date, date};
    /// assert_eq!(Date::from_ymd(2019, 1, 1), date!(2019-001));
    /// assert_eq!(Date::from_ymd(2019, 12, 31), date!(2019-365));
    /// ```
    ///
    /// Panics if the date is not valid.
    ///
    /// ```rust,should_panic
    /// # #![allow(deprecated)]
    /// # use time::Date;
    /// Date::from_ymd(2019, 2, 29); // 2019 isn't a leap year.
    /// ```
    #[cfg(feature = "panicking-api")]
    #[cfg_attr(__time_02_docs, doc(cfg(feature = "panicking-api")))]
    #[deprecated(
        since = "0.2.3",
        note = "For dates knowable at compile-time, use the `date!` macro. For situations where a \
                value isn't known, use `Date::try_from_ymd`."
    )]
    pub fn from_ymd(year: i32, month: u8, day: u8) -> Self {
        assert_value_in_range!(year in MIN_YEAR => MAX_YEAR);
        assert_value_in_range!(month in 1 => 12);
        assert_value_in_range!(day in 1 => days_in_year_month(year, month), given year, month);

        internals::Date::from_ymd_unchecked(year, month, day)
    }

    /// Attempt to create a `Date` from the year, month, and day.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert!(Date::try_from_ymd(2019, 1, 1).is_ok());
    /// assert!(Date::try_from_ymd(2019, 12, 31).is_ok());
    /// ```
    ///
    /// Returns `None` if the date is not valid.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert!(Date::try_from_ymd(2019, 2, 29).is_err()); // 2019 isn't a leap year.
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn try_from_ymd(
        year: i32,
        month: u8,
        day: u8,
    ) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(year in MIN_YEAR => MAX_YEAR);
        ensure_value_in_range!(month in 1 => 12);
        ensure_value_in_range!(day conditionally in 1 => days_in_year_month(year, month));

        Ok(internals::Date::from_ymd_unchecked(year, month, day))
    }

    /// Create a `Date` from the year and ordinal day number.
    ///
    /// ```rust
    /// # #![allow(deprecated)]
    /// # use time::{Date, date};
    /// assert_eq!(Date::from_yo(2019, 1), date!(2019-01-01));
    /// assert_eq!(Date::from_yo(2019, 365), date!(2019-12-31));
    /// ```
    ///
    /// Panics if the date is not valid.
    ///
    /// ```rust,should_panic
    /// # #![allow(deprecated)]
    /// # use time::Date;
    /// Date::from_yo(2019, 366); // 2019 isn't a leap year.
    /// ```
    #[cfg(feature = "panicking-api")]
    #[cfg_attr(__time_02_docs, doc(cfg(feature = "panicking-api")))]
    #[deprecated(
        since = "0.2.3",
        note = "For dates knowable at compile-time, use the `date!` macro. For situations where a \
                value isn't known, use `Date::try_from_yo`."
    )]
    pub fn from_yo(year: i32, ordinal: u16) -> Self {
        assert_value_in_range!(year in MIN_YEAR => MAX_YEAR);
        assert_value_in_range!(ordinal in 1 => days_in_year(year), given year);
        internals::Date::from_yo_unchecked(year, ordinal)
    }

    /// Attempt to create a `Date` from the year and ordinal day number.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert!(Date::try_from_yo(2019, 1).is_ok());
    /// assert!(Date::try_from_yo(2019, 365).is_ok());
    /// ```
    ///
    /// Returns `None` if the date is not valid.
    ///
    /// ```rust
    /// # use time::Date;
    /// assert!(Date::try_from_yo(2019, 366).is_err()); // 2019 isn't a leap year.
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn try_from_yo(year: i32, ordinal: u16) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(year in MIN_YEAR => MAX_YEAR);
        ensure_value_in_range!(ordinal conditionally in 1 => days_in_year(year));
        Ok(internals::Date::from_yo_unchecked(year, ordinal))
    }

    /// Create a `Date` from the ISO year, week, and weekday.
    ///
    /// ```rust
    /// # #![allow(deprecated)]
    /// # use time::{Date, Weekday::*, date};
    /// assert_eq!(
    ///     Date::from_iso_ywd(2019, 1, Monday),
    ///     date!(2018-12-31)
    /// );
    /// assert_eq!(
    ///     Date::from_iso_ywd(2019, 1, Tuesday),
    ///     date!(2019-01-01)
    /// );
    /// assert_eq!(
    ///     Date::from_iso_ywd(2020, 53, Friday),
    ///     date!(2021-01-01)
    /// );
    /// ```
    ///
    /// Panics if the week is not valid.
    ///
    /// ```rust,should_panic
    /// # #![allow(deprecated)]
    /// # use time::{Date, Weekday::*};
    /// Date::from_iso_ywd(2019, 53, Monday); // 2019 doesn't have 53 weeks.
    /// ```
    #[cfg(feature = "panicking-api")]
    #[cfg_attr(__time_02_docs, doc(cfg(feature = "panicking-api")))]
    #[deprecated(
        since = "0.2.3",
        note = "For dates knowable at compile-time, use the `date!` macro. For situations where a \
                value isn't known, use `Date::try_from_iso_ywd`."
    )]
    pub fn from_iso_ywd(year: i32, week: u8, weekday: Weekday) -> Self {
        assert_value_in_range!(year in MIN_YEAR => MAX_YEAR);
        assert_value_in_range!(week in 1 => weeks_in_year(year), given year);
        internals::Date::from_iso_ywd_unchecked(year, week, weekday)
    }

    /// Attempt to create a `Date` from the ISO year, week, and weekday.
    ///
    /// ```rust
    /// # use time::{Date, Weekday::*};
    /// assert!(Date::try_from_iso_ywd(2019, 1, Monday).is_ok());
    /// assert!(Date::try_from_iso_ywd(2019, 1, Tuesday).is_ok());
    /// assert!(Date::try_from_iso_ywd(2020, 53, Friday).is_ok());
    /// ```
    ///
    /// Returns `None` if the week is not valid.
    ///
    /// ```rust
    /// # use time::{Date, Weekday::*};
    /// assert!(Date::try_from_iso_ywd(2019, 53, Monday).is_err()); // 2019 doesn't have 53 weeks.
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn try_from_iso_ywd(
        year: i32,
        week: u8,
        weekday: Weekday,
    ) -> Result<Self, error::ComponentRange> {
        ensure_value_in_range!(year in MIN_YEAR => MAX_YEAR);
        ensure_value_in_range!(week conditionally in 1 => weeks_in_year(year));
        Ok(internals::Date::from_iso_ywd_unchecked(year, week, weekday))
    }

    /// Create a `Date` representing the current date.
    ///
    /// ```rust
    /// # #![allow(deprecated)]
    /// # use time::Date;
    /// assert!(Date::today().year() >= 2019);
    /// ```
    #[cfg(feature = "std")]
    #[cfg_attr(__time_02_docs, doc(cfg(feature = "std")))]
    #[deprecated(
        since = "0.2.7",
        note = "This method returns a value that assumes an offset of UTC."
    )]
    #[allow(deprecated)]
    pub fn today() -> Self {
        PrimitiveDateTime::now().date()
    }

    /// Get the year of the date.
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(date!(2019-01-01).year(), 2019);
    /// assert_eq!(date!(2019-12-31).year(), 2019);
    /// assert_eq!(date!(2020-01-01).year(), 2020);
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[allow(clippy::missing_const_for_fn)]
    #[const_fn("1.46")]
    pub const fn year(self) -> i32 {
        self.value >> 9
    }

    /// Get the month. If fetching both the month and day, it is more efficient
    /// to use [`Date::month_day`].
    ///
    /// The returned value will always be in the range `1..=12`.
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(date!(2019-01-01).month(), 1);
    /// assert_eq!(date!(2019-12-31).month(), 12);
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn month(self) -> u8 {
        self.month_day().0
    }

    /// Get the day of the month. If fetching both the month and day, it is more
    /// efficient to use [`Date::month_day`].
    ///
    /// The returned value will always be in the range `1..=31`.
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(date!(2019-01-01).day(), 1);
    /// assert_eq!(date!(2019-12-31).day(), 31);
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn day(self) -> u8 {
        self.month_day().1
    }

    /// Get the month and day. This is more efficient than fetching the
    /// components individually.
    ///
    /// The month component will always be in the range `1..=12`;
    /// the day component in `1..=31`.
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(date!(2019-01-01).month_day(), (1, 1));
    /// assert_eq!(date!(2019-12-31).month_day(), (12, 31));
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    // For whatever reason, rustc has difficulty optimizing this function. It's
    // significantly faster to write the statements out by hand.
    #[const_fn("1.46")]
    pub const fn month_day(self) -> (u8, u8) {
        /// The number of days up to and including the given month. Common years
        /// are first, followed by leap years.
        #[allow(clippy::items_after_statements)]
        const CUMULATIVE_DAYS_IN_MONTH_COMMON_LEAP: [[u16; 11]; 2] = [
            [31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334],
            [31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335],
        ];

        let days = CUMULATIVE_DAYS_IN_MONTH_COMMON_LEAP[is_leap_year(self.year()) as usize];
        let ordinal = self.ordinal();

        if ordinal > days[10] {
            (12, (ordinal - days[10]) as u8)
        } else if ordinal > days[9] {
            (11, (ordinal - days[9]) as u8)
        } else if ordinal > days[8] {
            (10, (ordinal - days[8]) as u8)
        } else if ordinal > days[7] {
            (9, (ordinal - days[7]) as u8)
        } else if ordinal > days[6] {
            (8, (ordinal - days[6]) as u8)
        } else if ordinal > days[5] {
            (7, (ordinal - days[5]) as u8)
        } else if ordinal > days[4] {
            (6, (ordinal - days[4]) as u8)
        } else if ordinal > days[3] {
            (5, (ordinal - days[3]) as u8)
        } else if ordinal > days[2] {
            (4, (ordinal - days[2]) as u8)
        } else if ordinal > days[1] {
            (3, (ordinal - days[1]) as u8)
        } else if ordinal > days[0] {
            (2, (ordinal - days[0]) as u8)
        } else {
            (1, ordinal as u8)
        }
    }

    /// Get the day of the year.
    ///
    /// The returned value will always be in the range `1..=366` (`1..=365` for
    /// common years).
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(date!(2019-01-01).ordinal(), 1);
    /// assert_eq!(date!(2019-12-31).ordinal(), 365);
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[allow(clippy::missing_const_for_fn)]
    #[const_fn("1.46")]
    pub const fn ordinal(self) -> u16 {
        (self.value & 0x1FF) as u16
    }

    /// Get the ISO 8601 year and week number.
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(date!(2019-01-01).iso_year_week(), (2019, 1));
    /// assert_eq!(date!(2019-10-04).iso_year_week(), (2019, 40));
    /// assert_eq!(date!(2020-01-01).iso_year_week(), (2020, 1));
    /// assert_eq!(date!(2020-12-31).iso_year_week(), (2020, 53));
    /// assert_eq!(date!(2021-01-01).iso_year_week(), (2020, 53));
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn iso_year_week(self) -> (i32, u8) {
        let (year, ordinal) = self.as_yo();

        match ((ordinal + 10 - self.iso_weekday_number() as u16) / 7) as u8 {
            0 => (year - 1, weeks_in_year(year - 1)),
            53 if weeks_in_year(year) == 52 => (year + 1, 1),
            _ => (
                year,
                ((ordinal + 10 - self.iso_weekday_number() as u16) / 7) as u8,
            ),
        }
    }

    /// Get the ISO week number.
    ///
    /// The returned value will always be in the range `1..=53`.
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(date!(2019-01-01).week(), 1);
    /// assert_eq!(date!(2019-10-04).week(), 40);
    /// assert_eq!(date!(2020-01-01).week(), 1);
    /// assert_eq!(date!(2020-12-31).week(), 53);
    /// assert_eq!(date!(2021-01-01).week(), 53);
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn week(self) -> u8 {
        self.iso_year_week().1
    }

    /// Get the week number where week 1 begins on the first Sunday.
    ///
    /// The returned value will always be in the range `0..=53`.
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(date!(2019-01-01).sunday_based_week(), 0);
    /// assert_eq!(date!(2020-01-01).sunday_based_week(), 0);
    /// assert_eq!(date!(2020-12-31).sunday_based_week(), 52);
    /// assert_eq!(date!(2021-01-01).sunday_based_week(), 0);
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn sunday_based_week(self) -> u8 {
        ((self.ordinal() as i16 - self.number_days_from_sunday() as i16 + 6) / 7) as u8
    }

    /// Get the week number where week 1 begins on the first Monday.
    ///
    /// The returned value will always be in the range `0..=53`.
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(date!(2019-01-01).monday_based_week(), 0);
    /// assert_eq!(date!(2020-01-01).monday_based_week(), 0);
    /// assert_eq!(date!(2020-12-31).monday_based_week(), 52);
    /// assert_eq!(date!(2021-01-01).monday_based_week(), 0);
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn monday_based_week(self) -> u8 {
        ((self.ordinal() as i16 - self.number_days_from_monday() as i16 + 6) / 7) as u8
    }

    /// Get the year, month, and day.
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(date!(2019-01-01).as_ymd(), (2019, 1, 1));
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn as_ymd(self) -> (i32, u8, u8) {
        let (month, day) = self.month_day();
        (self.year(), month, day)
    }

    /// Get the year and ordinal day number.
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(date!(2019-01-01).as_yo(), (2019, 1));
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[allow(clippy::missing_const_for_fn)]
    #[const_fn("1.46")]
    pub const fn as_yo(self) -> (i32, u16) {
        (self.year(), self.ordinal())
    }

    /// Get the ISO weekday number.
    ///
    /// This is equivalent to calling `.weekday().iso_weekday_number()`, but is
    /// usable in `const` contexts.
    #[const_fn("1.46")]
    pub(crate) const fn iso_weekday_number(self) -> u8 {
        self.number_days_from_monday() + 1
    }

    /// Get the number of days from Sunday.
    ///
    /// This is equivalent to calling `.weekday().number_days_from_sunday()`,
    /// but is usable in `const` contexts.
    #[const_fn("1.46")]
    pub(crate) const fn number_days_from_sunday(self) -> u8 {
        self.iso_weekday_number() % 7
    }

    /// Get the number of days from Monday.
    ///
    /// This is equivalent to calling `.weekday().number_days_from_monday()`,
    /// but is usable in `const` contexts.
    #[const_fn("1.46")]
    pub(crate) const fn number_days_from_monday(self) -> u8 {
        let (year, month, day) = self.as_ymd();

        let (month, adjusted_year) = if month < 3 {
            (month + 12, year - 1)
        } else {
            (month, year)
        };

        let raw_weekday =
            ((day as i32 + (13 * (month as i32 + 1)) / 5 + adjusted_year + adjusted_year / 4
                - adjusted_year / 100
                + adjusted_year / 400)
                - 2)
                % 7;

        if raw_weekday < 0 {
            (raw_weekday + 7) as u8
        } else {
            raw_weekday as u8
        }
    }

    /// Get the weekday.
    ///
    /// This current uses [Zeller's congruence](https://en.wikipedia.org/wiki/Zeller%27s_congruence)
    /// internally.
    ///
    /// ```rust
    /// # use time::{date, Weekday::*};
    /// assert_eq!(date!(2019-01-01).weekday(), Tuesday);
    /// assert_eq!(date!(2019-02-01).weekday(), Friday);
    /// assert_eq!(date!(2019-03-01).weekday(), Friday);
    /// assert_eq!(date!(2019-04-01).weekday(), Monday);
    /// assert_eq!(date!(2019-05-01).weekday(), Wednesday);
    /// assert_eq!(date!(2019-06-01).weekday(), Saturday);
    /// assert_eq!(date!(2019-07-01).weekday(), Monday);
    /// assert_eq!(date!(2019-08-01).weekday(), Thursday);
    /// assert_eq!(date!(2019-09-01).weekday(), Sunday);
    /// assert_eq!(date!(2019-10-01).weekday(), Tuesday);
    /// assert_eq!(date!(2019-11-01).weekday(), Friday);
    /// assert_eq!(date!(2019-12-01).weekday(), Sunday);
    /// ```
    pub fn weekday(self) -> Weekday {
        match self.number_days_from_monday() {
            0 => Weekday::Monday,
            1 => Weekday::Tuesday,
            2 => Weekday::Wednesday,
            3 => Weekday::Thursday,
            4 => Weekday::Friday,
            5 => Weekday::Saturday,
            6 => Weekday::Sunday,
            // FIXME The compiler isn't able to optimize this away. See
            // rust-lang/rust#66993.
            n => unreachable!("A value mod 7 is always in the range 0..7 (was {})", n),
        }
    }

    /// Get the next calendar date.
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(date!(2019-01-01).next_day(), date!(2019-01-02));
    /// assert_eq!(date!(2019-01-31).next_day(), date!(2019-02-01));
    /// assert_eq!(date!(2019-12-31).next_day(), date!(2020-01-01));
    /// ```
    pub fn next_day(self) -> Self {
        let (mut year, mut ordinal) = self.as_yo();

        ordinal += 1;

        if ordinal > days_in_year(year) {
            year += 1;
            ordinal = 1;
        }

        if year > MAX_YEAR {
            panic!("overflow when fetching next day");
        }

        internals::Date::from_yo_unchecked(year, ordinal)
    }

    /// Get the previous calendar date.
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(date!(2019-01-02).previous_day(), date!(2019-01-01));
    /// assert_eq!(date!(2019-02-01).previous_day(), date!(2019-01-31));
    /// assert_eq!(date!(2020-01-01).previous_day(), date!(2019-12-31));
    /// ```
    pub fn previous_day(self) -> Self {
        let (mut year, mut ordinal) = self.as_yo();

        ordinal -= 1;

        if ordinal == 0 {
            year -= 1;
            ordinal = days_in_year(year);
        }

        if year < MIN_YEAR {
            panic!("overflow when fetching previous day");
        }

        internals::Date::from_yo_unchecked(year, ordinal)
    }

    /// Get the Julian day for the date.
    ///
    /// The algorithm to perform this conversion is derived from one provided by
    /// Peter Baum; it is freely available
    /// [here](https://www.researchgate.net/publication/316558298_Date_Algorithms).
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(date!(-4713-11-24).julian_day(), 0);
    /// assert_eq!(date!(2000-01-01).julian_day(), 2_451_545);
    /// assert_eq!(date!(2019-01-01).julian_day(), 2_458_485);
    /// assert_eq!(date!(2019-12-31).julian_day(), 2_458_849);
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn julian_day(self) -> i64 {
        let (mut year, mut month, day) = self.as_ymd();

        if month < 3 {
            year -= 1;
            month += 12;
        }

        let year = year as i64;
        let month = month as i64;
        let day = day as i64;

        day + (153 * month - 457) / 5 + 365 * year + div_floor(year, 4) - div_floor(year, 100)
            + div_floor(year, 400)
            + 1_721_119
    }

    /// Create a `Date` from the Julian day.
    ///
    /// The algorithm to perform this conversion is derived from one provided by
    /// Peter Baum; it is freely available
    /// [here](https://www.researchgate.net/publication/316558298_Date_Algorithms).
    ///
    /// ```rust
    /// # use time::{Date, date};
    /// assert_eq!(
    ///     Date::from_julian_day(0),
    ///     date!(-4713-11-24)
    /// );
    /// assert_eq!(Date::from_julian_day(2_451_545), date!(2000-01-01));
    /// assert_eq!(Date::from_julian_day(2_458_485), date!(2019-01-01));
    /// assert_eq!(Date::from_julian_day(2_458_849), date!(2019-12-31));
    /// ```
    // TODO Return a `Result<Self, error::ComponentRange>` in 0.3
    pub fn from_julian_day(julian_day: i64) -> Self {
        #![allow(clippy::many_single_char_names)]
        let z = julian_day - 1_721_119;
        let h = 100 * z - 25;
        let a = div_floor(h, 3_652_425);
        let b = a - div_floor(a, 4);
        let mut year = div_floor(100 * b + h, 36_525);
        let c = b + z - 365 * year - div_floor(year, 4);
        let mut month = (5 * c + 456) / 153;
        let day = c - (153 * month - 457) / 5;

        if month > 12 {
            year += 1;
            month -= 12;
        }

        match Date::try_from_ymd(year as i32, month as u8, day as u8) {
            Ok(date) => date,
            Err(err) => panic!("{}", err),
        }
    }
}

/// Methods to add a `Time` component, resulting in a `PrimitiveDateTime`.
impl Date {
    /// Create a `PrimitiveDateTime` using the existing date. The `Time` component will
    /// be set to midnight.
    ///
    /// ```rust
    /// # use time::{date, time};
    /// assert_eq!(
    ///     date!(1970-01-01).midnight(),
    ///     date!(1970-01-01).with_time(time!(0:00))
    /// );
    /// ```
    pub const fn midnight(self) -> PrimitiveDateTime {
        PrimitiveDateTime::new(self, Time::midnight())
    }

    /// Create a `PrimitiveDateTime` using the existing date and the provided `Time`.
    ///
    /// ```rust
    /// # use time::{date, time};
    /// assert_eq!(
    ///     date!(1970-01-01).with_time(time!(0:00)),
    ///     date!(1970-01-01).midnight(),
    /// );
    /// ```
    pub const fn with_time(self, time: Time) -> PrimitiveDateTime {
        PrimitiveDateTime::new(self, time)
    }

    /// Create a `PrimitiveDateTime` using the existing date and the provided time.
    ///
    /// ```rust
    /// # #![allow(deprecated)]
    /// # use time::{date, time};
    /// assert_eq!(
    ///     date!(1970-01-01).with_hms(0, 0, 0),
    ///     date!(1970-01-01).with_time(time!(0:00)),
    /// );
    /// ```
    #[cfg(feature = "panicking-api")]
    #[cfg_attr(__time_02_docs, doc(cfg(feature = "panicking-api")))]
    #[allow(deprecated)]
    #[deprecated(
        since = "0.2.3",
        note = "For times knowable at compile-time, use the `time!` macro and `Date::with_time`. \
                For situations where a value isn't known, use `Date::try_with_hms`."
    )]
    pub fn with_hms(self, hour: u8, minute: u8, second: u8) -> PrimitiveDateTime {
        PrimitiveDateTime::new(self, Time::from_hms(hour, minute, second))
    }

    /// Attempt to create a `PrimitiveDateTime` using the existing date and the
    /// provided time.
    ///
    /// ```rust
    /// # use time::date;
    /// assert!(date!(1970-01-01).try_with_hms(0, 0, 0).is_ok());
    /// assert!(date!(1970-01-01).try_with_hms(24, 0, 0).is_err());
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn try_with_hms(
        self,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<PrimitiveDateTime, error::ComponentRange> {
        Ok(PrimitiveDateTime::new(
            self,
            const_try!(Time::try_from_hms(hour, minute, second)),
        ))
    }

    /// Create a `PrimitiveDateTime` using the existing date and the provided
    /// time.
    ///
    /// ```rust
    /// # #![allow(deprecated)]
    /// # use time::{date, time};
    /// assert_eq!(
    ///     date!(1970-01-01).with_hms_milli(0, 0, 0, 0),
    ///     date!(1970-01-01).with_time(time!(0:00)),
    /// );
    /// ```
    #[cfg(feature = "panicking-api")]
    #[cfg_attr(__time_02_docs, doc(cfg(feature = "panicking-api")))]
    #[allow(deprecated)]
    #[deprecated(
        since = "0.2.3",
        note = "For times knowable at compile-time, use the `time!` macro and `Date::with_time`. \
                For situations where a value isn't known, use `Date::try_with_hms_milli`."
    )]
    pub fn with_hms_milli(
        self,
        hour: u8,
        minute: u8,
        second: u8,
        millisecond: u16,
    ) -> PrimitiveDateTime {
        PrimitiveDateTime::new(
            self,
            Time::from_hms_milli(hour, minute, second, millisecond),
        )
    }

    /// Attempt to create a `PrimitiveDateTime` using the existing date and the provided time.
    ///
    /// ```rust
    /// # use time::date;
    /// assert!(date!(1970-01-01).try_with_hms_milli(0, 0, 0, 0).is_ok());
    /// assert!(date!(1970-01-01).try_with_hms_milli(24, 0, 0, 0).is_err());
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn try_with_hms_milli(
        self,
        hour: u8,
        minute: u8,
        second: u8,
        millisecond: u16,
    ) -> Result<PrimitiveDateTime, error::ComponentRange> {
        Ok(PrimitiveDateTime::new(
            self,
            const_try!(Time::try_from_hms_milli(hour, minute, second, millisecond)),
        ))
    }

    /// Create a `PrimitiveDateTime` using the existing date and the provided time.
    ///
    /// ```rust
    /// # #![allow(deprecated)]
    /// # use time::{date, time};
    /// assert_eq!(
    ///     date!(1970-01-01).with_hms_micro(0, 0, 0, 0),
    ///     date!(1970-01-01).with_time(time!(0:00)),
    /// );
    /// ```
    #[cfg(feature = "panicking-api")]
    #[cfg_attr(__time_02_docs, doc(cfg(feature = "panicking-api")))]
    #[allow(deprecated)]
    #[deprecated(
        since = "0.2.3",
        note = "For times knowable at compile-time, use the `time!` macro and `Date::with_time`. \
                For situations where a value isn't known, use `Date::try_with_hms_micro`."
    )]
    pub fn with_hms_micro(
        self,
        hour: u8,
        minute: u8,
        second: u8,
        microsecond: u32,
    ) -> PrimitiveDateTime {
        PrimitiveDateTime::new(
            self,
            Time::from_hms_micro(hour, minute, second, microsecond),
        )
    }

    /// Attempt to create a `PrimitiveDateTime` using the existing date and the
    /// provided time.
    ///
    /// ```rust
    /// # use time::date;
    /// assert!(date!(1970-01-01)
    ///     .try_with_hms_micro(0, 0, 0, 0)
    ///     .is_ok());
    /// assert!(date!(1970-01-01)
    ///     .try_with_hms_micro(24, 0, 0, 0)
    ///     .is_err());
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn try_with_hms_micro(
        self,
        hour: u8,
        minute: u8,
        second: u8,
        microsecond: u32,
    ) -> Result<PrimitiveDateTime, error::ComponentRange> {
        Ok(PrimitiveDateTime::new(
            self,
            const_try!(Time::try_from_hms_micro(hour, minute, second, microsecond)),
        ))
    }

    /// Create a `PrimitiveDateTime` using the existing date and the provided time.
    ///
    /// ```rust
    /// # #![allow(deprecated)]
    /// # use time::{date, time};
    /// assert_eq!(
    ///     date!(1970-01-01).with_hms_nano(0, 0, 0, 0),
    ///     date!(1970-01-01).with_time(time!(0:00)),
    /// );
    /// ```
    #[cfg(feature = "panicking-api")]
    #[cfg_attr(__time_02_docs, doc(cfg(feature = "panicking-api")))]
    #[allow(deprecated)]
    #[deprecated(
        since = "0.2.3",
        note = "For times knowable at compile-time, use the `time!` macro and `Date::with_time`. \
                For situations where a value isn't known, use `Date::try_with_hms_nano`."
    )]
    pub fn with_hms_nano(
        self,
        hour: u8,
        minute: u8,
        second: u8,
        nanosecond: u32,
    ) -> PrimitiveDateTime {
        PrimitiveDateTime::new(self, Time::from_hms_nano(hour, minute, second, nanosecond))
    }

    /// Attempt to create a `PrimitiveDateTime` using the existing date and the provided time.
    ///
    /// ```rust
    /// # use time::date;
    /// assert!(date!(1970-01-01).try_with_hms_nano(0, 0, 0, 0).is_ok());
    /// assert!(date!(1970-01-01).try_with_hms_nano(24, 0, 0, 0).is_err());
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn try_with_hms_nano(
        self,
        hour: u8,
        minute: u8,
        second: u8,
        nanosecond: u32,
    ) -> Result<PrimitiveDateTime, error::ComponentRange> {
        Ok(PrimitiveDateTime::new(
            self,
            const_try!(Time::try_from_hms_nano(hour, minute, second, nanosecond)),
        ))
    }
}

/// Methods that allow formatting the `Date`.
impl Date {
    /// Format the `Date` using the provided string.
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(date!(2019-01-02).format("%Y-%m-%d"), "2019-01-02");
    /// ```
    pub fn format(self, format: impl AsRef<str>) -> String {
        self.lazy_format(format).to_string()
    }

    /// Format the `Date` using the provided string.
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(date!(2019-01-02).lazy_format("%Y-%m-%d").to_string(), "2019-01-02");
    /// ```
    pub fn lazy_format(self, format: impl AsRef<str>) -> impl Display {
        DeferredFormat::new(format.as_ref())
            .with_date(self)
            .to_owned()
    }

    /// Attempt to parse a `Date` using the provided string.
    ///
    /// ```rust
    /// # use time::{Date, date};
    /// assert_eq!(
    ///     Date::parse("2019-01-02", "%F"),
    ///     Ok(date!(2019-01-02))
    /// );
    /// assert_eq!(
    ///     Date::parse("2019-002", "%Y-%j"),
    ///     Ok(date!(2019-002))
    /// );
    /// assert_eq!(
    ///     Date::parse("2019-W01-3", "%G-W%V-%u"),
    ///     Ok(date!(2019-W01-3))
    /// );
    /// ```
    pub fn parse(s: impl AsRef<str>, format: impl AsRef<str>) -> ParseResult<Self> {
        Self::try_from_parsed_items(parse(s.as_ref(), &format.into())?)
    }

    /// Given the items already parsed, attempt to create a `Date`.
    pub(crate) fn try_from_parsed_items(items: ParsedItems) -> ParseResult<Self> {
        macro_rules! items {
            ($($item:ident),* $(,)?) => {
                ParsedItems { $($item: Some($item)),*, .. }
            };
        }

        /// Get the value needed to adjust the ordinal day for Sunday and
        /// Monday-based week numbering.
        fn adjustment(year: i32) -> i16 {
            match internals::Date::from_yo_unchecked(year, 1).weekday() {
                Weekday::Monday => 7,
                Weekday::Tuesday => 1,
                Weekday::Wednesday => 2,
                Weekday::Thursday => 3,
                Weekday::Friday => 4,
                Weekday::Saturday => 5,
                Weekday::Sunday => 6,
            }
        }

        match items {
            items!(year, month, day) => {
                Date::try_from_ymd(year, month.get(), day.get()).map_err(Into::into)
            }
            items!(year, ordinal_day) => {
                Date::try_from_yo(year, ordinal_day.get()).map_err(Into::into)
            }
            items!(week_based_year, iso_week, weekday) => {
                Date::try_from_iso_ywd(week_based_year, iso_week.get(), weekday).map_err(Into::into)
            }
            items!(year, sunday_week, weekday) => Date::try_from_yo(
                year,
                (sunday_week as i16 * 7 + weekday.number_days_from_sunday() as i16
                    - adjustment(year)
                    + 1) as u16,
            )
            .map_err(Into::into),
            items!(year, monday_week, weekday) => Date::try_from_yo(
                year,
                (monday_week as i16 * 7 + weekday.number_days_from_monday() as i16
                    - adjustment(year)
                    + 1) as u16,
            )
            .map_err(Into::into),
            _ => Err(error::Parse::InsufficientInformation),
        }
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::format::{date, Padding};

        date::fmt_Y(f, *self, Padding::Zero)?;
        f.write_str("-")?;
        date::fmt_m(f, *self, Padding::Zero)?;
        f.write_str("-")?;
        date::fmt_d(f, *self, Padding::Zero)?;

        Ok(())
    }
}

impl Add<Duration> for Date {
    type Output = Self;

    fn add(self, duration: Duration) -> Self::Output {
        Self::from_julian_day(self.julian_day() + duration.whole_days())
    }
}

impl Add<StdDuration> for Date {
    type Output = Self;

    fn add(self, duration: StdDuration) -> Self::Output {
        Self::from_julian_day(self.julian_day() + (duration.as_secs() / 86_400) as i64)
    }
}

impl AddAssign<Duration> for Date {
    fn add_assign(&mut self, duration: Duration) {
        *self = *self + duration;
    }
}

impl AddAssign<StdDuration> for Date {
    fn add_assign(&mut self, duration: StdDuration) {
        *self = *self + duration;
    }
}

impl Sub<Duration> for Date {
    type Output = Self;

    fn sub(self, duration: Duration) -> Self::Output {
        self + -duration
    }
}

impl Sub<StdDuration> for Date {
    type Output = Self;

    fn sub(self, duration: StdDuration) -> Self::Output {
        Self::from_julian_day(self.julian_day() - (duration.as_secs() / 86_400) as i64)
    }
}

impl SubAssign<Duration> for Date {
    fn sub_assign(&mut self, duration: Duration) {
        *self = *self - duration;
    }
}

impl SubAssign<StdDuration> for Date {
    fn sub_assign(&mut self, duration: StdDuration) {
        *self = *self - duration;
    }
}

impl Sub<Date> for Date {
    type Output = Duration;

    fn sub(self, other: Self) -> Self::Output {
        Duration::days(self.julian_day() - other.julian_day())
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> Ordering {
        self.year()
            .cmp(&other.year())
            .then_with(|| self.ordinal().cmp(&other.ordinal()))
    }
}

#[cfg(test)]
#[rustfmt::skip::macros(date)]
mod test {
    use super::*;

    #[test]
    fn test_days_in_year_month() {
        // Common year
        assert_eq!(days_in_year_month(2019, 1), 31);
        assert_eq!(days_in_year_month(2019, 2), 28);
        assert_eq!(days_in_year_month(2019, 3), 31);
        assert_eq!(days_in_year_month(2019, 4), 30);
        assert_eq!(days_in_year_month(2019, 5), 31);
        assert_eq!(days_in_year_month(2019, 6), 30);
        assert_eq!(days_in_year_month(2019, 7), 31);
        assert_eq!(days_in_year_month(2019, 8), 31);
        assert_eq!(days_in_year_month(2019, 9), 30);
        assert_eq!(days_in_year_month(2019, 10), 31);
        assert_eq!(days_in_year_month(2019, 11), 30);
        assert_eq!(days_in_year_month(2019, 12), 31);

        // Leap year
        assert_eq!(days_in_year_month(2020, 1), 31);
        assert_eq!(days_in_year_month(2020, 2), 29);
        assert_eq!(days_in_year_month(2020, 3), 31);
        assert_eq!(days_in_year_month(2020, 4), 30);
        assert_eq!(days_in_year_month(2020, 5), 31);
        assert_eq!(days_in_year_month(2020, 6), 30);
        assert_eq!(days_in_year_month(2020, 7), 31);
        assert_eq!(days_in_year_month(2020, 8), 31);
        assert_eq!(days_in_year_month(2020, 9), 30);
        assert_eq!(days_in_year_month(2020, 10), 31);
        assert_eq!(days_in_year_month(2020, 11), 30);
        assert_eq!(days_in_year_month(2020, 12), 31);
    }
}
