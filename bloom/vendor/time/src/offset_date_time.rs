#[cfg(feature = "std")]
use crate::error;
use crate::{
    format::parse::{parse, ParsedItems},
    internals, Date, DeferredFormat, Duration, Format, ParseResult, PrimitiveDateTime, Time,
    UtcOffset, Weekday,
};
#[cfg(not(feature = "std"))]
use alloc::{
    borrow::ToOwned,
    string::{String, ToString},
};
#[cfg(feature = "std")]
use core::convert::From;
use core::{
    cmp::Ordering,
    fmt::{self, Display},
    hash::{Hash, Hasher},
    ops::{Add, AddAssign, Sub, SubAssign},
    time::Duration as StdDuration,
};
#[cfg(feature = "std")]
use standback::convert::TryFrom;
#[cfg(feature = "serde")]
use standback::convert::TryInto;
#[cfg(feature = "std")]
use std::time::SystemTime;

/// A [`PrimitiveDateTime`] with a [`UtcOffset`].
///
/// All comparisons are performed using the UTC time.
// Internally, an `OffsetDateTime` is a thin wrapper around a
// [`PrimitiveDateTime`] coupled with a [`UtcOffset`]. This offset is added to
// the date, time, or datetime as necessary for presentation or returning from a
// function.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(into = "crate::serde::PrimitiveDateTime"))]
#[derive(Debug, Clone, Copy, Eq)]
pub struct OffsetDateTime {
    /// The `PrimitiveDateTime`, which is _always_ UTC.
    utc_datetime: PrimitiveDateTime,
    /// The `UtcOffset`, which will be added to the `PrimitiveDateTime` as necessary.
    offset: UtcOffset,
}

#[cfg(feature = "serde")]
impl<'a> serde::Deserialize<'a> for OffsetDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        crate::serde::PrimitiveDateTime::deserialize(deserializer)?
            .try_into()
            .map_err(serde::de::Error::custom)
    }
}

impl OffsetDateTime {
    /// Create a new `OffsetDateTime` from the provided `PrimitiveDateTime` and
    /// `UtcOffset`. The `PrimitiveDateTime` is assumed to be in the provided
    /// offset.
    // TODO Should this be made public?
    pub(crate) fn new_assuming_offset(utc_datetime: PrimitiveDateTime, offset: UtcOffset) -> Self {
        Self {
            utc_datetime: utc_datetime - offset.as_duration(),
            offset,
        }
    }

    /// Create a new `OffsetDateTime` from the provided `PrimitiveDateTime` and
    /// `UtcOffset`. The `PrimitiveDateTime` is assumed to be in UTC.
    // TODO Should this be made public?
    pub(crate) const fn new_assuming_utc(utc_datetime: PrimitiveDateTime) -> Self {
        Self {
            utc_datetime,
            offset: UtcOffset::UTC,
        }
    }

    /// Create a new `OffsetDateTime` with the current date and time in UTC.
    ///
    /// ```rust
    /// # #![allow(deprecated)]
    /// # use time::{OffsetDateTime, offset};
    /// assert!(OffsetDateTime::now().year() >= 2019);
    /// assert_eq!(OffsetDateTime::now().offset(), offset!(UTC));
    /// ```
    #[deprecated(
        since = "0.2.11",
        note = "This function returns a value with an offset of UTC, which is not apparent from \
                its name alone. You should use `OffsetDateTime::now_utc()` instead."
    )]
    #[cfg(feature = "std")]
    #[cfg_attr(__time_02_docs, doc(cfg(feature = "std")))]
    pub fn now() -> Self {
        SystemTime::now().into()
    }

    /// Create a new `OffsetDateTime` with the current date and time in UTC.
    ///
    /// ```rust
    /// # use time::{OffsetDateTime, offset};
    /// assert!(OffsetDateTime::now_utc().year() >= 2019);
    /// assert_eq!(OffsetDateTime::now_utc().offset(), offset!(UTC));
    /// ```
    #[cfg(feature = "std")]
    #[cfg_attr(__time_02_docs, doc(cfg(feature = "std")))]
    pub fn now_utc() -> Self {
        SystemTime::now().into()
    }

    /// Create a new `OffsetDateTime` with the current date and time in the
    /// local offset.
    ///
    /// ```rust
    /// # #![allow(deprecated)]
    /// # use time::OffsetDateTime;
    /// assert!(OffsetDateTime::now_local().year() >= 2019);
    /// ```
    #[cfg(feature = "std")]
    #[cfg_attr(__time_02_docs, doc(cfg(feature = "std")))]
    #[deprecated(
        since = "0.2.23",
        note = "UTC is returned if the local offset cannot be determined"
    )]
    #[allow(deprecated)]
    pub fn now_local() -> Self {
        let t = Self::now_utc();
        t.to_offset(UtcOffset::local_offset_at(t))
    }

    /// Attempt to create a new `OffsetDateTime` with the current date and time
    /// in the local offset. If the offset cannot be determined, an error is
    /// returned.
    ///
    /// ```rust
    /// # use time::OffsetDateTime;
    /// let now = OffsetDateTime::try_now_local();
    /// # if false {
    /// assert!(now.is_ok());
    /// # }
    /// ```
    #[cfg(feature = "std")]
    #[cfg_attr(__time_02_docs, doc(cfg(feature = "std")))]
    pub fn try_now_local() -> Result<Self, error::IndeterminateOffset> {
        let t = Self::now_utc();
        Ok(t.to_offset(UtcOffset::try_local_offset_at(t)?))
    }

    /// Convert the `OffsetDateTime` from the current `UtcOffset` to the
    /// provided `UtcOffset`.
    ///
    /// ```rust
    /// # use time::{date, offset};
    /// assert_eq!(
    ///     date!(2000-01-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .to_offset(offset!(-1))
    ///         .year(),
    ///     1999,
    /// );
    ///
    /// // Let's see what time Sydney's new year's celebration is in New York
    /// // and Los Angeles.
    ///
    /// // Construct midnight on new year's in Sydney. This is equivalent to
    /// // 13:00 UTC.
    /// let sydney = date!(2000-01-01).midnight().assume_offset(offset!(+11));
    /// let new_york = sydney.to_offset(offset!(-5));
    /// let los_angeles = sydney.to_offset(offset!(-8));
    /// assert_eq!(sydney.hour(), 0);
    /// assert_eq!(new_york.hour(), 8);
    /// assert_eq!(los_angeles.hour(), 5);
    /// ```
    pub const fn to_offset(self, offset: UtcOffset) -> Self {
        Self {
            utc_datetime: self.utc_datetime,
            offset,
        }
    }

    /// Midnight, 1 January, 1970 (UTC).
    ///
    /// ```rust
    /// # use time::{date, OffsetDateTime};
    /// assert_eq!(
    ///     OffsetDateTime::unix_epoch(),
    ///     date!(1970-01-01)
    ///         .midnight()
    ///         .assume_utc(),
    /// );
    /// ```
    pub const fn unix_epoch() -> Self {
        internals::Date::from_yo_unchecked(1970, 1)
            .midnight()
            .assume_utc()
    }

    /// Create an `OffsetDateTime` from the provided [Unix timestamp](https://en.wikipedia.org/wiki/Unix_time).
    ///
    /// ```rust
    /// # use time::{date, OffsetDateTime};
    /// assert_eq!(
    ///     OffsetDateTime::from_unix_timestamp(0),
    ///     OffsetDateTime::unix_epoch(),
    /// );
    /// assert_eq!(
    ///     OffsetDateTime::from_unix_timestamp(1_546_300_800),
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .assume_utc(),
    /// );
    /// ```
    ///
    /// If you have a timestamp-nanosecond pair, you can use something along the
    /// lines of the following:
    ///
    /// ```rust
    /// # use time::{Duration, OffsetDateTime, ext::NumericalDuration};
    /// let (timestamp, nanos) = (1, 500_000_000);
    /// assert_eq!(
    ///     OffsetDateTime::from_unix_timestamp(timestamp) + Duration::nanoseconds(nanos),
    ///     OffsetDateTime::unix_epoch() + 1.5.seconds()
    /// );
    /// ```
    pub fn from_unix_timestamp(timestamp: i64) -> Self {
        OffsetDateTime::unix_epoch() + Duration::seconds(timestamp)
    }

    /// Construct an `OffsetDateTime` from the provided Unix timestamp (in
    /// nanoseconds).
    ///
    /// ```rust
    /// # use time::{date, OffsetDateTime};
    /// assert_eq!(
    ///     OffsetDateTime::from_unix_timestamp_nanos(0),
    ///     OffsetDateTime::unix_epoch(),
    /// );
    /// assert_eq!(
    ///     OffsetDateTime::from_unix_timestamp_nanos(1_546_300_800_000_000_000),
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .assume_utc(),
    /// );
    /// ```
    ///
    /// Note that the range of timestamps possible here is far larger than the
    /// valid range of dates storable in this crate. It is the _user's
    /// responsibility_ to ensure the timestamp provided as a parameter is
    /// valid. No behavior is guaranteed if this parameter would not result in a
    /// valid value.
    pub fn from_unix_timestamp_nanos(timestamp: i128) -> Self {
        OffsetDateTime::unix_epoch() + Duration::nanoseconds_i128(timestamp)
    }

    /// Get the `UtcOffset`.
    ///
    /// ```rust
    /// # use time::{date, offset};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .offset(),
    ///     offset!(UTC),
    /// );
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .assume_offset(offset!(+1))
    ///         .offset(),
    ///     offset!(+1),
    /// );
    /// ```
    pub const fn offset(self) -> UtcOffset {
        self.offset
    }

    /// Get the [Unix timestamp](https://en.wikipedia.org/wiki/Unix_time).
    ///
    /// ```rust
    /// # use time::{date, offset};
    /// assert_eq!(
    ///     date!(1970-01-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .unix_timestamp(),
    ///     0,
    /// );
    /// assert_eq!(
    ///     date!(1970-01-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .to_offset(offset!(-1))
    ///         .unix_timestamp(),
    ///     0,
    /// );
    /// ```
    #[allow(clippy::missing_const_for_fn)]
    pub fn unix_timestamp(self) -> i64 {
        let days = (self.utc_datetime.date.julian_day()
            - internals::Date::from_yo_unchecked(1970, 1).julian_day())
            * 86_400;
        let hours = self.utc_datetime.hour() as i64 * 3_600;
        let minutes = self.utc_datetime.minute() as i64 * 60;
        let seconds = self.utc_datetime.second() as i64;
        days + hours + minutes + seconds
    }

    /// Get the [Unix timestamp](https://en.wikipedia.org/wiki/Unix_time).
    ///
    /// ```rust
    /// # #![allow(deprecated)]
    /// # use time::{date, offset};
    /// assert_eq!(
    ///     date!(1970-01-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .timestamp(),
    ///     0,
    /// );
    /// assert_eq!(
    ///     date!(1970-01-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .to_offset(offset!(-1))
    ///         .timestamp(),
    ///     0,
    /// );
    /// ```
    #[deprecated(
        since = "0.2.23",
        note = "Use `OffsetDateTime::unix_timestamp` instead"
    )]
    pub fn timestamp(self) -> i64 {
        self.unix_timestamp()
    }

    /// Get the Unix timestamp in nanoseconds.
    ///
    /// ```rust
    /// use time::{date, offset, time};
    /// assert_eq!(
    ///     date!(1970-01-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .unix_timestamp_nanos(),
    ///     0,
    /// );
    /// assert_eq!(
    ///     date!(1970-01-01)
    ///         .with_time(time!(1:00))
    ///         .assume_utc()
    ///         .to_offset(offset!(-1))
    ///         .unix_timestamp_nanos(),
    ///     3_600_000_000_000,
    /// );
    /// ```
    pub fn unix_timestamp_nanos(self) -> i128 {
        (self - Self::unix_epoch()).whole_nanoseconds()
    }

    /// Get the Unix timestamp in nanoseconds.
    ///
    /// ```rust
    /// # #![allow(deprecated)]
    /// use time::{date, offset, time};
    /// assert_eq!(
    ///     date!(1970-01-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .unix_timestamp_nanos(),
    ///     0,
    /// );
    /// assert_eq!(
    ///     date!(1970-01-01)
    ///         .with_time(time!(1:00))
    ///         .assume_utc()
    ///         .to_offset(offset!(-1))
    ///         .unix_timestamp_nanos(),
    ///     3_600_000_000_000,
    /// );
    /// ```
    #[deprecated(
        since = "0.2.23",
        note = "Use `OffsetDateTime::unix_timestamp_nanos` instead"
    )]
    pub fn timestamp_nanos(self) -> i128 {
        self.unix_timestamp_nanos()
    }

    /// Get the `Date` in the stored offset.
    ///
    /// ```rust
    /// # use time::{date, offset};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .date(),
    ///     date!(2019-01-01),
    /// );
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .to_offset(offset!(-1))
    ///         .date(),
    ///     date!(2018-12-31),
    /// );
    /// ```
    pub fn date(self) -> Date {
        (self.utc_datetime + self.offset.as_duration()).date()
    }

    /// Get the `Time` in the stored offset.
    ///
    /// ```rust
    /// # use time::{date, offset, time};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .time(),
    ///     time!(0:00)
    /// );
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .to_offset(offset!(-1))
    ///         .time(),
    ///     time!(23:00)
    /// );
    /// ```
    pub fn time(self) -> Time {
        (self.utc_datetime + self.offset.as_duration()).time()
    }

    /// Get the year of the date in the stored offset.
    ///
    /// ```rust
    /// # use time::{date, offset, time};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .year(),
    ///     2019,
    /// );
    /// assert_eq!(
    ///     date!(2019-12-31)
    ///         .with_time(time!(23:00))
    ///         .assume_utc()
    ///         .to_offset(offset!(+1))
    ///         .year(),
    ///     2020,
    /// );
    /// assert_eq!(
    ///     date!(2020-01-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .year(),
    ///     2020,
    /// );
    /// ```
    pub fn year(self) -> i32 {
        self.date().year()
    }

    /// Get the month of the date in the stored offset. If fetching both the
    /// month and day, it is more efficient to use
    /// [`OffsetDateTime::month_day`].
    ///
    /// The returned value will always be in the range `1..=12`.
    ///
    /// ```rust
    /// # use time::{date, offset, time};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .month(),
    ///     1,
    /// );
    /// assert_eq!(
    ///     date!(2019-12-31)
    ///         .with_time(time!(23:00))
    ///         .assume_utc()
    ///         .to_offset(offset!(+1))
    ///         .month(),
    ///     1,
    /// );
    /// ```
    pub fn month(self) -> u8 {
        self.date().month()
    }

    /// Get the day of the date in the stored offset. If fetching both the month
    /// and day, it is more efficient to use [`OffsetDateTime::month_day`].
    ///
    /// The returned value will always be in the range `1..=31`.
    ///
    /// ```rust
    /// # use time::{date, offset, time};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .day(),
    ///     1,
    /// );
    /// assert_eq!(
    ///     date!(2019-12-31)
    ///         .with_time(time!(23:00))
    ///         .assume_utc()
    ///         .to_offset(offset!(+1))
    ///         .day(),
    ///     1,
    /// );
    /// ```
    pub fn day(self) -> u8 {
        self.date().day()
    }

    /// Get the month and day of the date in the stored offset.
    ///
    /// The month component will always be in the range `1..=12`;
    /// the day component in `1..=31`.
    ///
    /// ```rust
    /// # use time::{date, offset, time};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .month_day(),
    ///     (1, 1),
    /// );
    /// assert_eq!(
    ///     date!(2019-12-31)
    ///         .with_time(time!(23:00))
    ///         .assume_utc()
    ///         .to_offset(offset!(+1))
    ///         .month_day(),
    ///     (1, 1),
    /// );
    /// ```
    pub fn month_day(self) -> (u8, u8) {
        self.date().month_day()
    }

    /// Get the day of the year of the date in the stored offset.
    ///
    /// The returned value will always be in the range `1..=366`.
    ///
    /// ```rust
    /// # use time::{date, offset, time};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .ordinal(),
    ///     1,
    /// );
    /// assert_eq!(
    ///     date!(2019-12-31)
    ///         .with_time(time!(23:00))
    ///         .assume_utc()
    ///         .to_offset(offset!(+1))
    ///         .ordinal(),
    ///     1,
    /// );
    /// ```
    pub fn ordinal(self) -> u16 {
        self.date().ordinal()
    }

    /// Get the ISO 8601 year and week number in the stored offset.
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .iso_year_week(),
    ///     (2019, 1),
    /// );
    /// assert_eq!(
    ///     date!(2019-10-04)
    ///         .midnight()
    ///         .assume_utc()
    ///         .iso_year_week(),
    ///     (2019, 40),
    /// );
    /// assert_eq!(
    ///     date!(2020-01-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .iso_year_week(),
    ///     (2020, 1),
    /// );
    /// assert_eq!(
    ///     date!(2020-12-31)
    ///         .midnight()
    ///         .assume_utc()
    ///         .iso_year_week(),
    ///     (2020, 53),
    /// );
    /// assert_eq!(
    ///     date!(2021-01-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .iso_year_week(),
    ///     (2020, 53),
    /// );
    /// ```
    pub fn iso_year_week(self) -> (i32, u8) {
        self.date().iso_year_week()
    }

    /// Get the ISO week number of the date in the stored offset.
    ///
    /// The returned value will always be in the range `1..=53`.
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .week(),
    ///     1,
    /// );
    /// assert_eq!(
    ///     date!(2020-01-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .week(),
    ///     1,
    /// );
    /// assert_eq!(
    ///     date!(2020-12-31)
    ///         .midnight()
    ///         .assume_utc()
    ///         .week(),
    ///     53,
    /// );
    /// assert_eq!(
    ///     date!(2021-01-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .week(),
    ///     53,
    /// );
    /// ```
    pub fn week(self) -> u8 {
        self.date().week()
    }

    /// Get the weekday of the date in the stored offset.
    ///
    /// This current uses [Zeller's congruence](https://en.wikipedia.org/wiki/Zeller%27s_congruence)
    /// internally.
    ///
    /// ```rust
    /// # use time::{date, Weekday::*};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .weekday(),
    ///     Tuesday,
    /// );
    /// assert_eq!(
    ///     date!(2019-02-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .weekday(),
    ///     Friday,
    /// );
    /// assert_eq!(
    ///     date!(2019-03-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .weekday(),
    ///     Friday,
    /// );
    /// ```
    pub fn weekday(self) -> Weekday {
        self.date().weekday()
    }

    /// Get the clock hour in the stored offset.
    ///
    /// The returned value will always be in the range `0..24`.
    ///
    /// ```rust
    /// # use time::{date, time, offset};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .hour(),
    ///     0,
    /// );
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .with_time(time!(23:59:59))
    ///         .assume_utc()
    ///         .to_offset(offset!(-2))
    ///         .hour(),
    ///     21,
    /// );
    /// ```
    pub fn hour(self) -> u8 {
        self.time().hour()
    }

    /// Get the minute within the hour in the stored offset.
    ///
    /// The returned value will always be in the range `0..60`.
    ///
    /// ```rust
    /// # use time::{date, offset, time};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .minute(),
    ///     0,
    /// );
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .with_time(time!(23:59:59))
    ///         .assume_utc()
    ///         .to_offset(offset!(+0:30))
    ///         .minute(),
    ///     29,
    /// );
    /// ```
    pub fn minute(self) -> u8 {
        self.time().minute()
    }

    /// Get the second within the minute in the stored offset.
    ///
    /// The returned value will always be in the range `0..60`.
    ///
    /// ```rust
    /// # use time::{date, offset, time};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .second(),
    ///     0,
    /// );
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .with_time(time!(23:59:59))
    ///         .assume_utc()
    ///         .to_offset(offset!(+0:00:30))
    ///         .second(),
    ///     29,
    /// );
    /// ```
    pub fn second(self) -> u8 {
        self.time().second()
    }

    /// Get the milliseconds within the second in the stored offset.
    ///
    /// The returned value will always be in the range `0..1_000`.
    ///
    /// ```rust
    /// # use time::{date, time};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .millisecond(),
    ///     0,
    /// );
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .with_time(time!(23:59:59.999))
    ///         .assume_utc()
    ///         .millisecond(),
    ///     999,
    /// );
    /// ```
    pub fn millisecond(self) -> u16 {
        self.time().millisecond()
    }

    /// Get the microseconds within the second in the stored offset.
    ///
    /// The returned value will always be in the range `0..1_000_000`.
    ///
    /// ```rust
    /// # use time::{date, time};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .microsecond(),
    ///     0,
    /// );
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .with_time(time!(23:59:59.999_999))
    ///         .assume_utc()
    ///         .microsecond(),
    ///     999_999,
    /// );
    /// ```
    pub fn microsecond(self) -> u32 {
        self.time().microsecond()
    }

    /// Get the nanoseconds within the second in the stored offset.
    ///
    /// The returned value will always be in the range `0..1_000_000_000`.
    ///
    /// ```rust
    /// # use time::{date, time};
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .midnight()
    ///         .assume_utc()
    ///         .nanosecond(),
    ///     0,
    /// );
    /// assert_eq!(
    ///     date!(2019-01-01)
    ///         .with_time(time!(23:59:59.999_999_999))
    ///         .assume_utc()
    ///         .nanosecond(),
    ///     999_999_999,
    /// );
    /// ```
    pub fn nanosecond(self) -> u32 {
        self.time().nanosecond()
    }
}

/// Methods that allow formatting the `OffsetDateTime`.
impl OffsetDateTime {
    /// Format the `OffsetDateTime` using the provided string.
    ///
    /// ```rust
    /// # use time::{date};
    /// assert_eq!(
    ///     date!(2019-01-02)
    ///         .midnight()
    ///         .assume_utc()
    ///         .format("%F %r %z"),
    ///     "2019-01-02 12:00:00 am +0000",
    /// );
    /// ```
    pub fn format(self, format: impl Into<Format>) -> String {
        self.lazy_format(format).to_string()
    }

    /// Format the `OffsetDateTime` using the provided string.
    ///
    /// ```rust
    /// # use time::date;
    /// assert_eq!(
    ///     date!(2019-01-02)
    ///         .midnight()
    ///         .assume_utc()
    ///         .lazy_format("%F %r %z")
    ///         .to_string(),
    ///     "2019-01-02 12:00:00 am +0000",
    /// );
    /// ```
    pub fn lazy_format(self, format: impl Into<Format>) -> impl Display {
        DeferredFormat::new(format)
            .with_date(self.date())
            .with_time(self.time())
            .with_offset(self.offset())
            .to_owned()
    }

    /// Attempt to parse an `OffsetDateTime` using the provided string.
    ///
    /// ```rust
    /// # use time::{date, OffsetDateTime, time};
    /// assert_eq!(
    ///     OffsetDateTime::parse("2019-01-02 00:00:00 +0000", "%F %T %z"),
    ///     Ok(date!(2019-01-02).midnight().assume_utc()),
    /// );
    /// assert_eq!(
    ///     OffsetDateTime::parse("2019-002 23:59:59 +0000", "%Y-%j %T %z"),
    ///     Ok(date!(2019-002).with_time(time!(23:59:59)).assume_utc()),
    /// );
    /// assert_eq!(
    ///     OffsetDateTime::parse("2019-W01-3 12:00:00 pm +0000", "%G-W%V-%u %r %z"),
    ///     Ok(date!(2019-W01-3).with_time(time!(12:00)).assume_utc()),
    /// );
    /// ```
    pub fn parse(s: impl AsRef<str>, format: impl Into<Format>) -> ParseResult<Self> {
        Self::try_from_parsed_items(parse(s.as_ref(), &format.into())?)
    }

    /// Given the items already parsed, attempt to create an `OffsetDateTime`.
    pub(crate) fn try_from_parsed_items(items: ParsedItems) -> ParseResult<Self> {
        let offset = UtcOffset::try_from_parsed_items(items)?;
        Ok(PrimitiveDateTime::try_from_parsed_items(items)?.assume_offset(offset))
    }
}

impl Display for OffsetDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.date(), self.time(), self.offset())
    }
}

impl PartialEq for OffsetDateTime {
    fn eq(&self, rhs: &Self) -> bool {
        self.utc_datetime.eq(&rhs.utc_datetime)
    }
}

impl PartialOrd for OffsetDateTime {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for OffsetDateTime {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.utc_datetime.cmp(&rhs.utc_datetime)
    }
}

impl Hash for OffsetDateTime {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        // We need to distinguish this from a `PrimitiveDateTime`, which would
        // otherwise conflict.
        hasher.write(b"OffsetDateTime");
        self.utc_datetime.hash(hasher);
    }
}

impl Add<Duration> for OffsetDateTime {
    type Output = Self;

    fn add(self, duration: Duration) -> Self::Output {
        Self {
            utc_datetime: self.utc_datetime + duration,
            offset: self.offset,
        }
    }
}

impl Add<StdDuration> for OffsetDateTime {
    type Output = Self;

    fn add(self, duration: StdDuration) -> Self::Output {
        Self {
            utc_datetime: self.utc_datetime + duration,
            offset: self.offset,
        }
    }
}

impl AddAssign<Duration> for OffsetDateTime {
    fn add_assign(&mut self, duration: Duration) {
        *self = *self + duration;
    }
}

impl AddAssign<StdDuration> for OffsetDateTime {
    fn add_assign(&mut self, duration: StdDuration) {
        *self = *self + duration;
    }
}

impl Sub<Duration> for OffsetDateTime {
    type Output = Self;

    fn sub(self, duration: Duration) -> Self::Output {
        Self {
            utc_datetime: self.utc_datetime - duration,
            offset: self.offset,
        }
    }
}

impl Sub<StdDuration> for OffsetDateTime {
    type Output = Self;

    fn sub(self, duration: StdDuration) -> Self::Output {
        Self {
            utc_datetime: self.utc_datetime - duration,
            offset: self.offset,
        }
    }
}

impl SubAssign<Duration> for OffsetDateTime {
    fn sub_assign(&mut self, duration: Duration) {
        *self = *self - duration;
    }
}

impl SubAssign<StdDuration> for OffsetDateTime {
    fn sub_assign(&mut self, duration: StdDuration) {
        *self = *self - duration;
    }
}

impl Sub<OffsetDateTime> for OffsetDateTime {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        self.utc_datetime - rhs.utc_datetime
    }
}

#[cfg(feature = "std")]
impl Add<Duration> for SystemTime {
    type Output = Self;

    fn add(self, duration: Duration) -> Self::Output {
        if duration.is_zero() {
            self
        } else if duration.is_positive() {
            self + duration.abs_std()
        } else {
            // duration.is_negative()
            self - duration.abs_std()
        }
    }
}

#[cfg(feature = "std")]
impl AddAssign<Duration> for SystemTime {
    fn add_assign(&mut self, duration: Duration) {
        *self = *self + duration;
    }
}

#[cfg(feature = "std")]
impl Sub<Duration> for SystemTime {
    type Output = Self;

    fn sub(self, duration: Duration) -> Self::Output {
        (OffsetDateTime::from(self) - duration).into()
    }
}

#[cfg(feature = "std")]
impl SubAssign<Duration> for SystemTime {
    fn sub_assign(&mut self, duration: Duration) {
        *self = *self - duration;
    }
}

#[cfg(feature = "std")]
impl Sub<SystemTime> for OffsetDateTime {
    type Output = Duration;

    fn sub(self, rhs: SystemTime) -> Self::Output {
        self - Self::from(rhs)
    }
}

#[cfg(feature = "std")]
impl Sub<OffsetDateTime> for SystemTime {
    type Output = Duration;

    fn sub(self, rhs: OffsetDateTime) -> Self::Output {
        OffsetDateTime::from(self) - rhs
    }
}

#[cfg(feature = "std")]
impl PartialEq<SystemTime> for OffsetDateTime {
    fn eq(&self, rhs: &SystemTime) -> bool {
        self == &Self::from(*rhs)
    }
}

#[cfg(feature = "std")]
impl PartialEq<OffsetDateTime> for SystemTime {
    fn eq(&self, rhs: &OffsetDateTime) -> bool {
        &OffsetDateTime::from(*self) == rhs
    }
}

#[cfg(feature = "std")]
impl PartialOrd<SystemTime> for OffsetDateTime {
    fn partial_cmp(&self, other: &SystemTime) -> Option<Ordering> {
        self.partial_cmp(&Self::from(*other))
    }
}

#[cfg(feature = "std")]
impl PartialOrd<OffsetDateTime> for SystemTime {
    fn partial_cmp(&self, other: &OffsetDateTime) -> Option<Ordering> {
        OffsetDateTime::from(*self).partial_cmp(other)
    }
}

#[cfg(feature = "std")]
impl From<SystemTime> for OffsetDateTime {
    // There is definitely some way to have this conversion be infallible, but
    // it won't be an issue for over 500 years.
    fn from(system_time: SystemTime) -> Self {
        let duration = match system_time.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => Duration::try_from(duration)
                .expect("overflow converting `std::time::Duration` to `time::Duration`"),
            Err(err) => -Duration::try_from(err.duration())
                .expect("overflow converting `std::time::Duration` to `time::Duration`"),
        };

        Self::unix_epoch() + duration
    }
}

#[cfg(feature = "std")]
impl From<OffsetDateTime> for SystemTime {
    fn from(datetime: OffsetDateTime) -> Self {
        let duration = datetime - OffsetDateTime::unix_epoch();

        if duration.is_zero() {
            Self::UNIX_EPOCH
        } else if duration.is_positive() {
            Self::UNIX_EPOCH + duration.abs_std()
        } else {
            // duration.is_negative()
            Self::UNIX_EPOCH - duration.abs_std()
        }
    }
}
