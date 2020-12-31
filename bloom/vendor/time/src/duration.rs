use crate::error;
#[cfg(feature = "std")]
use crate::Instant;
use const_fn::const_fn;
use core::{
    cmp::Ordering,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    time::Duration as StdDuration,
};
use standback::convert::{TryFrom, TryInto};
#[allow(unused_imports)]
use standback::prelude::*;

/// A span of time with nanosecond precision.
///
/// Each `Duration` is composed of a whole number of seconds and a fractional
/// part represented in nanoseconds.
///
/// `Duration` implements many traits, including [`Add`], [`Sub`], [`Mul`], and
/// [`Div`], among others.
///
/// This implementation allows for negative durations, unlike
/// [`core::time::Duration`].
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(from = "crate::serde::Duration", into = "crate::serde::Duration")
)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Duration {
    /// Number of whole seconds.
    seconds: i64,
    /// Number of nanoseconds within the second. The sign always matches the
    /// `seconds` field.
    nanoseconds: i32, // always -10^9 < nanoseconds < 10^9
}

/// The number of seconds in one minute.
const SECONDS_PER_MINUTE: i64 = 60;

/// The number of seconds in one hour.
const SECONDS_PER_HOUR: i64 = 60 * SECONDS_PER_MINUTE;

/// The number of seconds in one day.
const SECONDS_PER_DAY: i64 = 24 * SECONDS_PER_HOUR;

/// The number of seconds in one week.
const SECONDS_PER_WEEK: i64 = 7 * SECONDS_PER_DAY;

impl Duration {
    /// Equivalent to `0.seconds()`.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::zero(), 0.seconds());
    /// ```
    pub const fn zero() -> Self {
        Self::seconds(0)
    }

    /// Equivalent to `1.nanoseconds()`.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::nanosecond(), 1.nanoseconds());
    /// ```
    pub const fn nanosecond() -> Self {
        Self::nanoseconds(1)
    }

    /// Equivalent to `1.microseconds()`.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::microsecond(), 1.microseconds());
    /// ```
    pub const fn microsecond() -> Self {
        Self::microseconds(1)
    }

    /// Equivalent to `1.milliseconds()`.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::millisecond(), 1.milliseconds());
    /// ```
    pub const fn millisecond() -> Self {
        Self::milliseconds(1)
    }

    /// Equivalent to `1.seconds()`.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::second(), 1.seconds());
    /// ```
    pub const fn second() -> Self {
        Self::seconds(1)
    }

    /// Equivalent to `1.minutes()`.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::minute(), 1.minutes());
    /// ```
    pub const fn minute() -> Self {
        Self::minutes(1)
    }

    /// Equivalent to `1.hours()`.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::hour(), 1.hours());
    /// ```
    pub const fn hour() -> Self {
        Self::hours(1)
    }

    /// Equivalent to `1.days()`.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::day(), 1.days());
    /// ```
    pub const fn day() -> Self {
        Self::days(1)
    }

    /// Equivalent to `1.weeks()`.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::week(), 1.weeks());
    /// ```
    pub const fn week() -> Self {
        Self::weeks(1)
    }

    /// The maximum possible duration. Adding any positive duration to this will
    /// cause an overflow.
    ///
    /// The value returned by this method may change at any time.
    pub const fn max_value() -> Self {
        Self::new(i64::max_value(), 999_999_999)
    }

    /// The minimum possible duration. Adding any negative duration to this will
    /// cause an overflow.
    ///
    /// The value returned by this method may change at any time.
    pub const fn min_value() -> Self {
        Self::new(i64::min_value(), -999_999_999)
    }

    /// Check if a duration is exactly zero.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert!(0.seconds().is_zero());
    /// assert!(!1.nanoseconds().is_zero());
    /// ```
    pub const fn is_zero(self) -> bool {
        (self.seconds == 0) & (self.nanoseconds == 0)
    }

    /// Check if a duration is negative.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert!((-1).seconds().is_negative());
    /// assert!(!0.seconds().is_negative());
    /// assert!(!1.seconds().is_negative());
    /// ```
    pub const fn is_negative(self) -> bool {
        (self.seconds < 0) | (self.nanoseconds < 0)
    }

    /// Check if a duration is positive.
    ///
    /// ```rust
    /// # use time::{prelude::*};
    /// assert!(1.seconds().is_positive());
    /// assert!(!0.seconds().is_positive());
    /// assert!(!(-1).seconds().is_positive());
    /// ```
    pub const fn is_positive(self) -> bool {
        (self.seconds > 0) | (self.nanoseconds > 0)
    }

    /// Get the sign of the duration.
    ///
    /// ```rust
    /// # #![allow(deprecated)]
    /// # use time::{Sign, prelude::*};
    /// assert_eq!(1.seconds().sign(), Sign::Positive);
    /// assert_eq!((-1).seconds().sign(), Sign::Negative);
    /// assert_eq!(0.seconds().sign(), Sign::Zero);
    /// ```
    #[deprecated(
        since = "0.2.7",
        note = "To obtain the sign of a `Duration`, you should use the `is_positive`, \
                `is_negative`, and `is_zero` methods."
    )]
    #[allow(deprecated, clippy::missing_const_for_fn)]
    pub fn sign(self) -> crate::Sign {
        use crate::Sign::*;

        if self.nanoseconds > 0 {
            Positive
        } else if self.nanoseconds < 0 {
            Negative
        } else if self.seconds > 0 {
            Positive
        } else if self.seconds < 0 {
            Negative
        } else {
            Zero
        }
    }

    /// Get the absolute value of the duration.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert_eq!(1.seconds().abs(), 1.seconds());
    /// assert_eq!(0.seconds().abs(), 0.seconds());
    /// assert_eq!((-1).seconds().abs(), 1.seconds());
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.39.
    #[const_fn("1.39")]
    pub const fn abs(self) -> Self {
        Self {
            seconds: self.seconds.abs(),
            nanoseconds: self.nanoseconds.abs(),
        }
    }

    /// Convert the existing `Duration` to a `std::time::Duration` and its sign.
    // This doesn't actually require the standard library, but is currently only
    // used when it's enabled.
    #[allow(clippy::missing_const_for_fn)] // false positive
    #[cfg(feature = "std")]
    pub(crate) fn abs_std(self) -> StdDuration {
        StdDuration::new(self.seconds.abs() as u64, self.nanoseconds.abs() as u32)
    }

    /// Create a new `Duration` with the provided seconds and nanoseconds. If
    /// nanoseconds is at least ±10<sup>9</sup>, it will wrap to the number of
    /// seconds.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::new(1, 0), 1.seconds());
    /// assert_eq!(Duration::new(-1, 0), (-1).seconds());
    /// assert_eq!(Duration::new(1, 2_000_000_000), 3.seconds());
    /// ```
    // FIXME This code is stupidly complex for the sole reason of maintaining
    // back-compatibility with Rust 1.32.0. The equivalent code is commented out
    // immediately below this function implementation. Thankfully, the compiler
    // is able to do quite well at deduplicating the operations.
    pub const fn new(seconds: i64, nanoseconds: i32) -> Self {
        Self {
            seconds: (seconds + nanoseconds as i64 / 1_000_000_000)
                + (((((seconds + nanoseconds as i64 / 1_000_000_000) > 0) as i8
                    - ((seconds + nanoseconds as i64 / 1_000_000_000) < 0) as i8)
                    == -1)
                    & ((((nanoseconds % 1_000_000_000) > 0) as i8
                        - ((nanoseconds % 1_000_000_000) < 0) as i8)
                        == 1)) as i64
                - (((((seconds + nanoseconds as i64 / 1_000_000_000) > 0) as i8
                    - ((seconds + nanoseconds as i64 / 1_000_000_000) < 0) as i8)
                    == 1)
                    & ((((nanoseconds % 1_000_000_000) > 0) as i8
                        - ((nanoseconds % 1_000_000_000) < 0) as i8)
                        == -1)) as i64,
            nanoseconds: (nanoseconds % 1_000_000_000)
                + 1_000_000_000
                    * ((((((seconds + nanoseconds as i64 / 1_000_000_000) > 0) as i8
                        - ((seconds + nanoseconds as i64 / 1_000_000_000) < 0) as i8)
                        == 1)
                        & ((((nanoseconds % 1_000_000_000) > 0) as i8
                            - ((nanoseconds % 1_000_000_000) < 0) as i8)
                            == -1)) as i32
                        - (((((seconds + nanoseconds as i64 / 1_000_000_000) > 0) as i8
                            - ((seconds + nanoseconds as i64 / 1_000_000_000) < 0) as i8)
                            == -1)
                            & ((((nanoseconds % 1_000_000_000) > 0) as i8
                                - ((nanoseconds % 1_000_000_000) < 0) as i8)
                                == 1)) as i32),
        }
    }

    // pub const fn new(mut seconds: i64, mut nanoseconds: i32) -> Self {
    //     seconds += nanoseconds as i64 / 1_000_000_000;
    //     nanoseconds %= 1_000_000_000;
    //
    //     // Alternatively, we can use `(nano)seconds.signum()` once it is
    //     // stabilized within `const fn`. The behavior is identical.
    //     let seconds_sign = match seconds {
    //         n if n > 0 => 1,
    //         0 => 0,
    //         _ => -1,
    //     };
    //     let nanoseconds_sign = match nanoseconds {
    //         n if n > 0 => 1,
    //         0 => 0,
    //         _ => -1,
    //     };
    //
    //     if seconds_sign == 1 && nanoseconds_sign == -1 {
    //         seconds -= 1;
    //         nanoseconds += 1_000_000_000;
    //     } else if seconds_sign == -1 && nanoseconds_sign == 1 {
    //         seconds += 1;
    //         nanoseconds -= 1_000_000_000;
    //     }
    //
    //     Self {
    //         seconds,
    //         nanoseconds,
    //     }
    // }

    /// Create a new `Duration` with the given number of weeks. Equivalent to
    /// `Duration::seconds(weeks * 604_800)`.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::weeks(1), 604_800.seconds());
    /// ```
    pub const fn weeks(weeks: i64) -> Self {
        Self::seconds(weeks * SECONDS_PER_WEEK)
    }

    /// Get the number of whole weeks in the duration.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert_eq!(1.weeks().whole_weeks(), 1);
    /// assert_eq!((-1).weeks().whole_weeks(), -1);
    /// assert_eq!(6.days().whole_weeks(), 0);
    /// assert_eq!((-6).days().whole_weeks(), 0);
    /// ```
    pub const fn whole_weeks(self) -> i64 {
        self.whole_seconds() / SECONDS_PER_WEEK
    }

    /// Create a new `Duration` with the given number of days. Equivalent to
    /// `Duration::seconds(days * 86_400)`.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::days(1), 86_400.seconds());
    /// ```
    pub const fn days(days: i64) -> Self {
        Self::seconds(days * SECONDS_PER_DAY)
    }

    /// Get the number of whole days in the duration.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert_eq!(1.days().whole_days(), 1);
    /// assert_eq!((-1).days().whole_days(), -1);
    /// assert_eq!(23.hours().whole_days(), 0);
    /// assert_eq!((-23).hours().whole_days(), 0);
    /// ```
    pub const fn whole_days(self) -> i64 {
        self.whole_seconds() / SECONDS_PER_DAY
    }

    /// Create a new `Duration` with the given number of hours. Equivalent to
    /// `Duration::seconds(hours * 3_600)`.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::hours(1), 3_600.seconds());
    /// ```
    pub const fn hours(hours: i64) -> Self {
        Self::seconds(hours * SECONDS_PER_HOUR)
    }

    /// Get the number of whole hours in the duration.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert_eq!(1.hours().whole_hours(), 1);
    /// assert_eq!((-1).hours().whole_hours(), -1);
    /// assert_eq!(59.minutes().whole_hours(), 0);
    /// assert_eq!((-59).minutes().whole_hours(), 0);
    /// ```
    pub const fn whole_hours(self) -> i64 {
        self.whole_seconds() / SECONDS_PER_HOUR
    }

    /// Create a new `Duration` with the given number of minutes. Equivalent to
    /// `Duration::seconds(minutes * 60)`.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::minutes(1), 60.seconds());
    /// ```
    pub const fn minutes(minutes: i64) -> Self {
        Self::seconds(minutes * SECONDS_PER_MINUTE)
    }

    /// Get the number of whole minutes in the duration.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert_eq!(1.minutes().whole_minutes(), 1);
    /// assert_eq!((-1).minutes().whole_minutes(), -1);
    /// assert_eq!(59.seconds().whole_minutes(), 0);
    /// assert_eq!((-59).seconds().whole_minutes(), 0);
    /// ```
    pub const fn whole_minutes(self) -> i64 {
        self.whole_seconds() / SECONDS_PER_MINUTE
    }

    /// Create a new `Duration` with the given number of seconds.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::seconds(1), 1_000.milliseconds());
    /// ```
    pub const fn seconds(seconds: i64) -> Self {
        Self {
            seconds,
            nanoseconds: 0,
        }
    }

    /// Get the number of whole seconds in the duration.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert_eq!(1.seconds().whole_seconds(), 1);
    /// assert_eq!((-1).seconds().whole_seconds(), -1);
    /// assert_eq!(1.minutes().whole_seconds(), 60);
    /// assert_eq!((-1).minutes().whole_seconds(), -60);
    /// ```
    pub const fn whole_seconds(self) -> i64 {
        self.seconds
    }

    /// Creates a new `Duration` from the specified number of seconds
    /// represented as `f64`.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::seconds_f64(0.5), 0.5.seconds());
    /// assert_eq!(Duration::seconds_f64(-0.5), -0.5.seconds());
    /// ```
    pub fn seconds_f64(seconds: f64) -> Self {
        Self {
            seconds: seconds as i64,
            nanoseconds: ((seconds % 1.) * 1_000_000_000.) as i32,
        }
    }

    /// Get the number of fractional seconds in the duration.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert_eq!(1.5.seconds().as_seconds_f64(), 1.5);
    /// assert_eq!((-1.5).seconds().as_seconds_f64(), -1.5);
    /// ```
    pub fn as_seconds_f64(self) -> f64 {
        self.seconds as f64 + self.nanoseconds as f64 / 1_000_000_000.
    }

    /// Creates a new `Duration` from the specified number of seconds
    /// represented as `f32`.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::seconds_f32(0.5), 0.5.seconds());
    /// assert_eq!(Duration::seconds_f32(-0.5), (-0.5).seconds());
    /// ```
    pub fn seconds_f32(seconds: f32) -> Self {
        Self {
            seconds: seconds as i64,
            nanoseconds: ((seconds % 1.) * 1_000_000_000.) as i32,
        }
    }

    /// Get the number of fractional seconds in the duration.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert_eq!(1.5.seconds().as_seconds_f32(), 1.5);
    /// assert_eq!((-1.5).seconds().as_seconds_f32(), -1.5);
    /// ```
    pub fn as_seconds_f32(self) -> f32 {
        self.seconds as f32 + self.nanoseconds as f32 / 1_000_000_000.
    }

    /// Create a new `Duration` with the given number of milliseconds.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::milliseconds(1), 1_000.microseconds());
    /// assert_eq!(Duration::milliseconds(-1), (-1_000).microseconds());
    /// ```
    pub const fn milliseconds(milliseconds: i64) -> Self {
        Self {
            seconds: milliseconds / 1_000,
            nanoseconds: ((milliseconds % 1_000) * 1_000_000) as i32,
        }
    }

    /// Get the number of whole milliseconds in the duration.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert_eq!(1.seconds().whole_milliseconds(), 1_000);
    /// assert_eq!((-1).seconds().whole_milliseconds(), -1_000);
    /// assert_eq!(1.milliseconds().whole_milliseconds(), 1);
    /// assert_eq!((-1).milliseconds().whole_milliseconds(), -1);
    /// ```
    pub const fn whole_milliseconds(self) -> i128 {
        self.seconds as i128 * 1_000 + self.nanoseconds as i128 / 1_000_000
    }

    /// Get the number of milliseconds past the number of whole seconds.
    ///
    /// Always in the range `-1_000..1_000`.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert_eq!(1.4.seconds().subsec_milliseconds(), 400);
    /// assert_eq!((-1.4).seconds().subsec_milliseconds(), -400);
    /// ```
    // Allow the lint, as the value is guaranteed to be less than 1000.
    pub const fn subsec_milliseconds(self) -> i16 {
        (self.nanoseconds / 1_000_000) as i16
    }

    /// Create a new `Duration` with the given number of microseconds.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::microseconds(1), 1_000.nanoseconds());
    /// assert_eq!(Duration::microseconds(-1), (-1_000).nanoseconds());
    /// ```
    pub const fn microseconds(microseconds: i64) -> Self {
        Self {
            seconds: microseconds / 1_000_000,
            nanoseconds: ((microseconds % 1_000_000) * 1_000) as i32,
        }
    }

    /// Get the number of whole microseconds in the duration.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert_eq!(1.milliseconds().whole_microseconds(), 1_000);
    /// assert_eq!((-1).milliseconds().whole_microseconds(), -1_000);
    /// assert_eq!(1.microseconds().whole_microseconds(), 1);
    /// assert_eq!((-1).microseconds().whole_microseconds(), -1);
    /// ```
    pub const fn whole_microseconds(self) -> i128 {
        self.seconds as i128 * 1_000_000 + self.nanoseconds as i128 / 1_000
    }

    /// Get the number of microseconds past the number of whole seconds.
    ///
    /// Always in the range `-1_000_000..1_000_000`.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert_eq!(1.0004.seconds().subsec_microseconds(), 400);
    /// assert_eq!((-1.0004).seconds().subsec_microseconds(), -400);
    /// ```
    pub const fn subsec_microseconds(self) -> i32 {
        self.nanoseconds / 1_000
    }

    /// Create a new `Duration` with the given number of nanoseconds.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(Duration::nanoseconds(1), 1.microseconds() / 1_000);
    /// assert_eq!(Duration::nanoseconds(-1), (-1).microseconds() / 1_000);
    /// ```
    pub const fn nanoseconds(nanoseconds: i64) -> Self {
        Self {
            seconds: nanoseconds / 1_000_000_000,
            nanoseconds: (nanoseconds % 1_000_000_000) as i32,
        }
    }

    /// Create a new `Duration` with the given number of nanoseconds.
    ///
    /// As the input range cannot be fully mapped to the output, this should
    /// only be used where it's known to result in a valid value.
    pub(crate) const fn nanoseconds_i128(nanoseconds: i128) -> Self {
        Self {
            seconds: (nanoseconds / 1_000_000_000) as i64,
            nanoseconds: (nanoseconds % 1_000_000_000) as i32,
        }
    }

    /// Get the number of nanoseconds in the duration.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert_eq!(1.microseconds().whole_nanoseconds(), 1_000);
    /// assert_eq!((-1).microseconds().whole_nanoseconds(), -1_000);
    /// assert_eq!(1.nanoseconds().whole_nanoseconds(), 1);
    /// assert_eq!((-1).nanoseconds().whole_nanoseconds(), -1);
    /// ```
    pub const fn whole_nanoseconds(self) -> i128 {
        self.seconds as i128 * 1_000_000_000 + self.nanoseconds as i128
    }

    /// Get the number of nanoseconds past the number of whole seconds.
    ///
    /// The returned value will always be in the range
    /// `-1_000_000_000..1_000_000_000`.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert_eq!(1.000_000_400.seconds().subsec_nanoseconds(), 400);
    /// assert_eq!((-1.000_000_400).seconds().subsec_nanoseconds(), -400);
    /// ```
    pub const fn subsec_nanoseconds(self) -> i32 {
        self.nanoseconds
    }

    /// Computes `self + rhs`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(5.seconds().checked_add(5.seconds()), Some(10.seconds()));
    /// assert_eq!(Duration::max_value().checked_add(1.nanoseconds()), None);
    /// assert_eq!((-5).seconds().checked_add(5.seconds()), Some(0.seconds()));
    /// ```
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        let mut seconds = self.seconds.checked_add(rhs.seconds)?;
        let mut nanoseconds = self.nanoseconds + rhs.nanoseconds;

        if nanoseconds >= 1_000_000_000 || seconds < 0 && nanoseconds > 0 {
            nanoseconds -= 1_000_000_000;
            seconds = seconds.checked_add(1)?;
        } else if nanoseconds <= -1_000_000_000 || seconds > 0 && nanoseconds < 0 {
            nanoseconds += 1_000_000_000;
            seconds = seconds.checked_sub(1)?;
        }

        // Ensure that the signs match _unless_ one of them is zero.
        debug_assert_ne!(seconds.signum() * nanoseconds.signum() as i64, -1);
        debug_assert!((-999_999_999..1_000_000_000).contains(&nanoseconds));

        Some(Self {
            seconds,
            nanoseconds,
        })
    }

    /// Computes `self - rhs`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(5.seconds().checked_sub(5.seconds()), Some(Duration::zero()));
    /// assert_eq!(Duration::min_value().checked_sub(1.nanoseconds()), None);
    /// assert_eq!(5.seconds().checked_sub(10.seconds()), Some((-5).seconds()));
    /// ```
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.checked_add(-rhs)
    }

    /// Computes `self * rhs`, returning `None` if an overflow occurred.
    ///
    /// ```rust
    /// # use time::{Duration, prelude::*};
    /// assert_eq!(5.seconds().checked_mul(2), Some(10.seconds()));
    /// assert_eq!(5.seconds().checked_mul(-2), Some((-10).seconds()));
    /// assert_eq!(5.seconds().checked_mul(0), Some(0.seconds()));
    /// assert_eq!(Duration::max_value().checked_mul(2), None);
    /// assert_eq!(Duration::min_value().checked_mul(2), None);
    /// ```
    pub fn checked_mul(self, rhs: i32) -> Option<Self> {
        // Multiply nanoseconds as i64, because it cannot overflow that way.
        let total_nanos = self.nanoseconds as i64 * rhs as i64;
        let extra_secs = total_nanos / 1_000_000_000;
        let nanoseconds = (total_nanos % 1_000_000_000) as i32;
        let seconds = self
            .seconds
            .checked_mul(rhs as i64)?
            .checked_add(extra_secs)?;

        Some(Self {
            seconds,
            nanoseconds,
        })
    }

    /// Computes `self / rhs`, returning `None` if `rhs == 0`.
    ///
    /// ```rust
    /// # use time::prelude::*;
    /// assert_eq!(10.seconds().checked_div(2), Some(5.seconds()));
    /// assert_eq!(10.seconds().checked_div(-2), Some((-5).seconds()));
    /// assert_eq!(1.seconds().checked_div(0), None);
    /// ```
    ///
    /// This function is `const fn` when using rustc >= 1.46.
    #[const_fn("1.46")]
    pub const fn checked_div(self, rhs: i32) -> Option<Self> {
        if rhs == 0 {
            return None;
        }

        let seconds = self.seconds / (rhs as i64);
        let carry = self.seconds - seconds * (rhs as i64);
        let extra_nanos = carry * 1_000_000_000 / (rhs as i64);
        let nanoseconds = self.nanoseconds / rhs + (extra_nanos as i32);

        Some(Self {
            seconds,
            nanoseconds,
        })
    }

    /// Runs a closure, returning the duration of time it took to run. The
    /// return value of the closure is provided in the second part of the tuple.
    #[cfg(feature = "std")]
    #[cfg_attr(__time_02_docs, doc(cfg(feature = "std")))]
    pub fn time_fn<T>(f: impl FnOnce() -> T) -> (Self, T) {
        let start = Instant::now();
        let return_value = f();
        let end = Instant::now();

        (end - start, return_value)
    }
}

/// Functions that have been renamed or had signatures changed since v0.1. As
/// such, they are deprecated.
#[cfg(feature = "deprecated")]
#[allow(clippy::missing_docs_in_private_items, clippy::missing_const_for_fn)]
impl Duration {
    #[deprecated(since = "0.2.0", note = "Use the `whole_weeks` function")]
    pub fn num_weeks(&self) -> i64 {
        self.whole_weeks()
    }

    #[deprecated(since = "0.2.0", note = "Use the `whole_days` function")]
    pub fn num_days(&self) -> i64 {
        self.whole_days()
    }

    #[deprecated(since = "0.2.0", note = "Use the `whole_hours` function")]
    pub fn num_hours(&self) -> i64 {
        self.whole_hours()
    }

    #[deprecated(since = "0.2.0", note = "Use the `whole_minutes` function")]
    pub fn num_minutes(&self) -> i64 {
        self.whole_minutes()
    }

    #[allow(clippy::missing_const_for_fn)]
    #[deprecated(since = "0.2.0", note = "Use the `whole_seconds` function")]
    pub fn num_seconds(&self) -> i64 {
        self.whole_seconds()
    }

    /// [`Duration::whole_milliseconds`] returns an `i128`, rather than
    /// panicking on overflow. To avoid panicking, this method currently limits
    /// the value to the range `i64::min_value()..=i64::max_value()`.
    #[deprecated(
        since = "0.2.0",
        note = "Use the `whole_milliseconds` function. The value is clamped between \
                `i64::min_value()` and `i64::max_value()`."
    )]
    pub fn num_milliseconds(&self) -> i64 {
        let millis = self.whole_milliseconds();

        if millis > i64::max_value() as i128 {
            return i64::max_value();
        }

        if millis < i64::min_value() as i128 {
            return i64::min_value();
        }

        millis as i64
    }

    /// [`Duration::whole_microseconds`] returns an `i128` rather than returning
    /// `None` on `i64` overflow.
    #[deprecated(since = "0.2.0", note = "Use the `whole_microseconds` function")]
    pub fn num_microseconds(&self) -> Option<i64> {
        let micros = self.whole_microseconds();

        if micros.abs() > i64::max_value() as i128 {
            None
        } else {
            Some(micros as i64)
        }
    }

    /// [`Duration::whole_nanoseconds`] returns an `i128` rather than returning
    /// `None` on `i64` overflow.
    #[deprecated(since = "0.2.0", note = "Use the `whole_nanoseconds` function")]
    pub fn num_nanoseconds(&self) -> Option<i64> {
        let nanos = self.whole_nanoseconds();

        if nanos.abs() > i64::max_value() as i128 {
            None
        } else {
            Some(nanos as i64)
        }
    }

    #[cfg(feature = "std")]
    #[deprecated(since = "0.2.0", note = "Use the `time_fn` function")]
    pub fn span<F: FnOnce()>(f: F) -> Self {
        Self::time_fn(f).0
    }

    #[allow(deprecated)]
    #[deprecated(
        since = "0.2.0",
        note = "Use `Duration::try_from(value)` or `value.try_into()`"
    )]
    pub fn from_std(std: StdDuration) -> Result<Self, error::ConversionRange> {
        std.try_into()
    }

    #[allow(deprecated)]
    #[deprecated(
        since = "0.2.0",
        note = "Use `std::time::Duration::try_from(value)` or `value.try_into()`"
    )]
    pub fn to_std(&self) -> Result<StdDuration, error::ConversionRange> {
        (*self).try_into()
    }
}

impl TryFrom<StdDuration> for Duration {
    type Error = error::ConversionRange;

    fn try_from(original: StdDuration) -> Result<Self, error::ConversionRange> {
        Ok(Self::new(
            original
                .as_secs()
                .try_into()
                .map_err(|_| error::ConversionRange)?,
            original
                .subsec_nanos()
                .try_into()
                .map_err(|_| error::ConversionRange)?,
        ))
    }
}

impl TryFrom<Duration> for StdDuration {
    type Error = error::ConversionRange;

    fn try_from(duration: Duration) -> Result<Self, error::ConversionRange> {
        Ok(Self::new(
            duration
                .seconds
                .try_into()
                .map_err(|_| error::ConversionRange)?,
            duration
                .nanoseconds
                .try_into()
                .map_err(|_| error::ConversionRange)?,
        ))
    }
}

impl Add for Duration {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs)
            .expect("overflow when adding durations")
    }
}

impl Add<StdDuration> for Duration {
    type Output = Self;

    fn add(self, std_duration: StdDuration) -> Self::Output {
        self + Self::try_from(std_duration)
            .expect("overflow converting `std::time::Duration` to `time::Duration`")
    }
}

impl Add<Duration> for StdDuration {
    type Output = Duration;

    fn add(self, rhs: Duration) -> Self::Output {
        rhs + self
    }
}

impl AddAssign for Duration {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<StdDuration> for Duration {
    fn add_assign(&mut self, rhs: StdDuration) {
        *self = *self + rhs;
    }
}

impl Neg for Duration {
    type Output = Self;

    fn neg(self) -> Self::Output {
        -1 * self
    }
}

impl Sub for Duration {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs)
            .expect("overflow when subtracting durations")
    }
}

impl Sub<StdDuration> for Duration {
    type Output = Self;

    fn sub(self, rhs: StdDuration) -> Self::Output {
        self - Self::try_from(rhs)
            .expect("overflow converting `std::time::Duration` to `time::Duration`")
    }
}

impl Sub<Duration> for StdDuration {
    type Output = Duration;

    fn sub(self, rhs: Duration) -> Self::Output {
        Duration::try_from(self)
            .expect("overflow converting `std::time::Duration` to `time::Duration`")
            - rhs
    }
}

impl SubAssign for Duration {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl SubAssign<StdDuration> for Duration {
    fn sub_assign(&mut self, rhs: StdDuration) {
        *self = *self - rhs;
    }
}

impl SubAssign<Duration> for StdDuration {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = (*self - rhs).try_into().expect(
            "Cannot represent a resulting duration in std. Try `let x = x - rhs;`, which will \
             change the type.",
        );
    }
}

macro_rules! duration_mul_div_int {
    ($($type:ty),+) => {
        $(
            impl Mul<$type> for Duration {
                type Output = Self;

                fn mul(self, rhs: $type) -> Self::Output {
                    Self::nanoseconds_i128(
                        self.whole_nanoseconds()
                            .checked_mul(rhs as i128)
                            .expect("overflow when multiplying duration")
                    )
                }
            }

            impl MulAssign<$type> for Duration {
                fn mul_assign(&mut self, rhs: $type) {
                    *self = *self * rhs;
                }
            }

            impl Mul<Duration> for $type {
                type Output = Duration;

                fn mul(self, rhs: Duration) -> Self::Output {
                    rhs * self
                }
            }

            impl Div<$type> for Duration {
                type Output = Self;

                fn div(self, rhs: $type) -> Self::Output {
                    Self::nanoseconds_i128(self.whole_nanoseconds() / rhs as i128)
                }
            }

            impl DivAssign<$type> for Duration {
                fn div_assign(&mut self, rhs: $type) {
                    *self = *self / rhs;
                }
            }
        )+
    };
}
duration_mul_div_int![i8, i16, i32, u8, u16, u32];

impl Mul<f32> for Duration {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::seconds_f32(self.as_seconds_f32() * rhs)
    }
}

impl MulAssign<f32> for Duration {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Mul<Duration> for f32 {
    type Output = Duration;

    fn mul(self, rhs: Duration) -> Self::Output {
        rhs * self
    }
}

impl Mul<f64> for Duration {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::seconds_f64(self.as_seconds_f64() * rhs)
    }
}

impl MulAssign<f64> for Duration {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Mul<Duration> for f64 {
    type Output = Duration;

    fn mul(self, rhs: Duration) -> Self::Output {
        rhs * self
    }
}

impl Div<f32> for Duration {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::seconds_f32(self.as_seconds_f32() / rhs)
    }
}

impl DivAssign<f32> for Duration {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl Div<f64> for Duration {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::seconds_f64(self.as_seconds_f64() / rhs)
    }
}

impl DivAssign<f64> for Duration {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl Div<Duration> for Duration {
    type Output = f64;

    fn div(self, rhs: Self) -> Self::Output {
        self.as_seconds_f64() / rhs.as_seconds_f64()
    }
}

impl Div<StdDuration> for Duration {
    type Output = f64;

    fn div(self, rhs: StdDuration) -> Self::Output {
        self.as_seconds_f64() / rhs.as_secs_f64()
    }
}

impl Div<Duration> for StdDuration {
    type Output = f64;

    fn div(self, rhs: Duration) -> Self::Output {
        self.as_secs_f64() / rhs.as_seconds_f64()
    }
}

impl PartialEq<StdDuration> for Duration {
    fn eq(&self, rhs: &StdDuration) -> bool {
        Ok(*self) == Self::try_from(*rhs)
    }
}

impl PartialEq<Duration> for StdDuration {
    fn eq(&self, rhs: &Duration) -> bool {
        rhs == self
    }
}

impl PartialOrd for Duration {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl PartialOrd<StdDuration> for Duration {
    fn partial_cmp(&self, rhs: &StdDuration) -> Option<Ordering> {
        if rhs.as_secs() > i64::max_value() as u64 {
            return Some(Ordering::Less);
        }

        Some(
            self.seconds
                .cmp(&(rhs.as_secs() as i64))
                .then_with(|| self.nanoseconds.cmp(&(rhs.subsec_nanos() as i32))),
        )
    }
}

impl PartialOrd<Duration> for StdDuration {
    fn partial_cmp(&self, rhs: &Duration) -> Option<Ordering> {
        rhs.partial_cmp(self).map(Ordering::reverse)
    }
}

impl Ord for Duration {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.seconds
            .cmp(&rhs.seconds)
            .then_with(|| self.nanoseconds.cmp(&rhs.nanoseconds))
    }
}
