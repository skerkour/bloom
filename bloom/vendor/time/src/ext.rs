#![allow(trivial_numeric_casts)]

use crate::Duration;
use core::time::Duration as StdDuration;

/// Create `Duration`s from primitive and core numeric types.
///
/// This trait can be imported with `use time::prelude::*`.
///
/// Due to limitations in rustc, these methods are currently _not_ `const fn`.
/// See [RFC 2632](https://github.com/rust-lang/rfcs/pull/2632) for details.
///
/// # Examples
///
/// Basic construction of `Duration`s.
///
/// ```rust
/// # use time::{Duration, ext::NumericalDuration};
/// assert_eq!(5.nanoseconds(), Duration::nanoseconds(5));
/// assert_eq!(5.microseconds(), Duration::microseconds(5));
/// assert_eq!(5.milliseconds(), Duration::milliseconds(5));
/// assert_eq!(5.seconds(), Duration::seconds(5));
/// assert_eq!(5.minutes(), Duration::minutes(5));
/// assert_eq!(5.hours(), Duration::hours(5));
/// assert_eq!(5.days(), Duration::days(5));
/// assert_eq!(5.weeks(), Duration::weeks(5));
/// ```
///
/// Signed integers work as well!
///
/// ```rust
/// # use time::{Duration, ext::NumericalDuration};
/// assert_eq!((-5).nanoseconds(), Duration::nanoseconds(-5));
/// assert_eq!((-5).microseconds(), Duration::microseconds(-5));
/// assert_eq!((-5).milliseconds(), Duration::milliseconds(-5));
/// assert_eq!((-5).seconds(), Duration::seconds(-5));
/// assert_eq!((-5).minutes(), Duration::minutes(-5));
/// assert_eq!((-5).hours(), Duration::hours(-5));
/// assert_eq!((-5).days(), Duration::days(-5));
/// assert_eq!((-5).weeks(), Duration::weeks(-5));
/// ```
///
/// Just like any other `Duration`, they can be added, subtracted, etc.
///
/// ```rust
/// # use time::ext::NumericalDuration;
/// assert_eq!(2.seconds() + 500.milliseconds(), 2_500.milliseconds());
/// assert_eq!(2.seconds() - 500.milliseconds(), 1_500.milliseconds());
/// ```
///
/// When called on floating point values, any remainder of the floating point
/// value will be truncated. Keep in mind that floating point numbers are
/// inherently imprecise and have limited capacity.
pub trait NumericalDuration {
    /// Create a `Duration` from the number of nanoseconds.
    fn nanoseconds(self) -> Duration;
    /// Create a `Duration` from the number of microseconds.
    fn microseconds(self) -> Duration;
    /// Create a `Duration` from the number of milliseconds.
    fn milliseconds(self) -> Duration;
    /// Create a `Duration` from the number of seconds.
    fn seconds(self) -> Duration;
    /// Create a `Duration` from the number of minutes.
    fn minutes(self) -> Duration;
    /// Create a `Duration` from the number of hours.
    fn hours(self) -> Duration;
    /// Create a `Duration` from the number of days.
    fn days(self) -> Duration;
    /// Create a `Duration` from the number of weeks.
    fn weeks(self) -> Duration;
}

macro_rules! impl_numerical_duration {
    ($($type:ty),* $(,)?) => {
        $(
            impl NumericalDuration for $type {
                fn nanoseconds(self) -> Duration {
                    Duration::nanoseconds(self as i64)
                }

                fn microseconds(self) -> Duration {
                    Duration::microseconds(self as i64)
                }

                fn milliseconds(self) -> Duration {
                    Duration::milliseconds(self as i64)
                }

                fn seconds(self) -> Duration {
                    Duration::seconds(self as i64)
                }

                fn minutes(self) -> Duration {
                    Duration::minutes(self as i64)
                }

                fn hours(self) -> Duration {
                    Duration::hours(self as i64)
                }

                fn days(self) -> Duration {
                    Duration::days(self as i64)
                }

                fn weeks(self) -> Duration {
                    Duration::weeks(self as i64)
                }
            }
        )*
    };
}

macro_rules! impl_numerical_duration_nonzero {
    ($($type:ty),* $(,)?) => {
        $(
            impl NumericalDuration for $type {
                fn nanoseconds(self) -> Duration {
                    Duration::nanoseconds(self.get() as i64)
                }

                fn microseconds(self) -> Duration {
                    Duration::microseconds(self.get() as i64)
                }

                fn milliseconds(self) -> Duration {
                    Duration::milliseconds(self.get() as i64)
                }

                fn seconds(self) -> Duration {
                    Duration::seconds(self.get() as i64)
                }

                fn minutes(self) -> Duration {
                    Duration::minutes(self.get() as i64)
                }

                fn hours(self) -> Duration {
                    Duration::hours(self.get() as i64)
                }

                fn days(self) -> Duration {
                    Duration::days(self.get() as i64)
                }

                fn weeks(self) -> Duration {
                    Duration::weeks(self.get() as i64)
                }
            }
        )*
    };
}

macro_rules! impl_numerical_duration_float {
    ($($type:ty),* $(,)?) => {
        $(
            impl NumericalDuration for $type {
                fn nanoseconds(self) -> Duration {
                    Duration::nanoseconds(self as i64)
                }

                fn microseconds(self) -> Duration {
                    Duration::nanoseconds((self * 1_000.) as i64)
                }

                fn milliseconds(self) -> Duration {
                    Duration::nanoseconds((self * 1_000_000.) as i64)
                }

                fn seconds(self) -> Duration {
                    Duration::nanoseconds((self * 1_000_000_000.) as i64)
                }

                fn minutes(self) -> Duration {
                    Duration::nanoseconds((self * 60_000_000_000.) as i64)
                }

                fn hours(self) -> Duration {
                    Duration::nanoseconds((self * 3_600_000_000_000.) as i64)
                }

                fn days(self) -> Duration {
                    Duration::nanoseconds((self * 86_400_000_000_000.) as i64)
                }

                fn weeks(self) -> Duration {
                    Duration::nanoseconds((self * 604_800_000_000_000.) as i64)
                }
            }
        )*
    };
}

impl_numerical_duration![u8, u16, u32, i8, i16, i32, i64];
impl_numerical_duration_nonzero![
    core::num::NonZeroU8,
    core::num::NonZeroU16,
    core::num::NonZeroU32,
];
#[cfg(__time_02_nonzero_signed)]
impl_numerical_duration_nonzero![
    core::num::NonZeroI8,
    core::num::NonZeroI16,
    core::num::NonZeroI32,
    core::num::NonZeroI64,
];
impl_numerical_duration_float![f32, f64];

/// Create `std::time::Duration`s from primitive and core numeric types.
///
/// This trait can be imported (alongside others) with `use time::prelude::*`.
///
/// Due to limitations in rustc, these methods are currently _not_ `const fn`.
/// See [RFC 2632](https://github.com/rust-lang/rfcs/pull/2632) for details.
///
/// # Examples
///
/// Basic construction of `std::time::Duration`s.
///
/// ```rust
/// # use time::ext::NumericalStdDuration;
/// # use core::time::Duration;
/// assert_eq!(5.std_nanoseconds(), Duration::from_nanos(5));
/// assert_eq!(5.std_microseconds(), Duration::from_micros(5));
/// assert_eq!(5.std_milliseconds(), Duration::from_millis(5));
/// assert_eq!(5.std_seconds(), Duration::from_secs(5));
/// assert_eq!(5.std_minutes(), Duration::from_secs(5 * 60));
/// assert_eq!(5.std_hours(), Duration::from_secs(5 * 3_600));
/// assert_eq!(5.std_days(), Duration::from_secs(5 * 86_400));
/// assert_eq!(5.std_weeks(), Duration::from_secs(5 * 604_800));
/// ```
///
/// Just like any other `std::time::Duration`, they can be added, subtracted,
/// etc.
///
/// ```rust
/// # use time::ext::NumericalStdDuration;
/// assert_eq!(
///     2.std_seconds() + 500.std_milliseconds(),
///     2_500.std_milliseconds()
/// );
/// assert_eq!(
///     2.std_seconds() - 500.std_milliseconds(),
///     1_500.std_milliseconds()
/// );
/// ```
///
/// When called on floating point values, any remainder of the floating point
/// value will be truncated. Keep in mind that floating point numbers are
/// inherently imprecise and have limited capacity.
pub trait NumericalStdDuration {
    /// Create a `std::time::Duration` from the number of nanoseconds.
    fn std_nanoseconds(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of microseconds.
    fn std_microseconds(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of milliseconds.
    fn std_milliseconds(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of seconds.
    fn std_seconds(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of minutes.
    fn std_minutes(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of hours.
    fn std_hours(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of days.
    fn std_days(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of weeks.
    fn std_weeks(self) -> StdDuration;
}

macro_rules! impl_numerical_std_duration {
    ($($type:ty),* $(,)?) => {
        $(
            impl NumericalStdDuration for $type {
                fn std_nanoseconds(self) -> StdDuration {
                    StdDuration::from_nanos(self as u64)
                }

                fn std_microseconds(self) -> StdDuration {
                    StdDuration::from_micros(self as u64)
                }

                fn std_milliseconds(self) -> StdDuration {
                    StdDuration::from_millis(self as u64)
                }

                fn std_seconds(self) -> StdDuration {
                    StdDuration::from_secs(self as u64)
                }

                fn std_minutes(self) -> StdDuration {
                    StdDuration::from_secs(self as u64 * 60)
                }

                fn std_hours(self) -> StdDuration {
                    StdDuration::from_secs(self as u64 * 3_600)
                }

                fn std_days(self) -> StdDuration {
                    StdDuration::from_secs(self as u64 * 86_400)
                }

                fn std_weeks(self) -> StdDuration {
                    StdDuration::from_secs(self as u64 * 604_800)
                }
            }
        )*
    };
}

macro_rules! impl_numerical_std_duration_nonzero {
    ($($type:ty),* $(,)?) => {
        $(
            impl NumericalStdDuration for $type {
                fn std_nanoseconds(self) -> StdDuration {
                    StdDuration::from_nanos(self.get() as u64)
                }

                fn std_microseconds(self) -> StdDuration {
                    StdDuration::from_micros(self.get() as u64)
                }

                fn std_milliseconds(self) -> StdDuration {
                    StdDuration::from_millis(self.get() as u64)
                }

                fn std_seconds(self) -> StdDuration {
                    StdDuration::from_secs(self.get() as u64)
                }

                fn std_minutes(self) -> StdDuration {
                    StdDuration::from_secs(self.get() as u64 * 60)
                }

                fn std_hours(self) -> StdDuration {
                    StdDuration::from_secs(self.get() as u64 * 3_600)
                }

                fn std_days(self) -> StdDuration {
                    StdDuration::from_secs(self.get() as u64 * 86_400)
                }

                fn std_weeks(self) -> StdDuration {
                    StdDuration::from_secs(self.get() as u64 * 604_800)
                }
            }
        )*
    };
}

impl_numerical_std_duration![u8, u16, u32, u64];
impl_numerical_std_duration_nonzero![
    core::num::NonZeroU8,
    core::num::NonZeroU16,
    core::num::NonZeroU32,
    core::num::NonZeroU64,
];

/// Implement on `i32` because that's the default type for integers. This
/// performs a runtime check and panics if the value is negative.
impl NumericalStdDuration for i32 {
    fn std_nanoseconds(self) -> StdDuration {
        assert!(self >= 0);
        StdDuration::from_nanos(self as u64)
    }

    fn std_microseconds(self) -> StdDuration {
        assert!(self >= 0);
        StdDuration::from_micros(self as u64)
    }

    fn std_milliseconds(self) -> StdDuration {
        assert!(self >= 0);
        StdDuration::from_millis(self as u64)
    }

    fn std_seconds(self) -> StdDuration {
        assert!(self >= 0);
        StdDuration::from_secs(self as u64)
    }

    fn std_minutes(self) -> StdDuration {
        assert!(self >= 0);
        StdDuration::from_secs(self as u64 * 60)
    }

    fn std_hours(self) -> StdDuration {
        assert!(self >= 0);
        StdDuration::from_secs(self as u64 * 3_600)
    }

    fn std_days(self) -> StdDuration {
        assert!(self >= 0);
        StdDuration::from_secs(self as u64 * 86_400)
    }

    fn std_weeks(self) -> StdDuration {
        assert!(self >= 0);
        StdDuration::from_secs(self as u64 * 604_800)
    }
}

/// Implement on `f64` because that's the default type for floats. This performs
/// a runtime check and panics if the value is negative.
impl NumericalStdDuration for f64 {
    fn std_nanoseconds(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos(self as u64)
    }

    fn std_microseconds(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 1_000.) as u64)
    }

    fn std_milliseconds(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 1_000_000.) as u64)
    }

    fn std_seconds(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 1_000_000_000.) as u64)
    }

    fn std_minutes(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 60_000_000_000.) as u64)
    }

    fn std_hours(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 3_600_000_000_000.) as u64)
    }

    fn std_days(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 86_400_000_000_000.) as u64)
    }

    fn std_weeks(self) -> StdDuration {
        assert!(self >= 0.);
        StdDuration::from_nanos((self * 604_800_000_000_000.) as u64)
    }
}

/// Create `std::time::Duration`s from primitive and core numeric types. Unless
/// you are always expecting a `std::time::Duration`, you should prefer to use
/// [`NumericalStdDuration`] for clarity.
///
/// Due to limitations in rustc, these methods are currently _not_ `const fn`.
/// See [this RFC](https://github.com/rust-lang/rfcs/pull/2632) for details.
///
/// # Examples
///
/// Basic construction of `std::time::Duration`s.
///
/// ```rust
/// # use time::ext::NumericalStdDurationShort;
/// # use core::time::Duration;
/// assert_eq!(5.nanoseconds(), Duration::from_nanos(5));
/// assert_eq!(5.microseconds(), Duration::from_micros(5));
/// assert_eq!(5.milliseconds(), Duration::from_millis(5));
/// assert_eq!(5.seconds(), Duration::from_secs(5));
/// assert_eq!(5.minutes(), Duration::from_secs(5 * 60));
/// assert_eq!(5.hours(), Duration::from_secs(5 * 3_600));
/// assert_eq!(5.days(), Duration::from_secs(5 * 86_400));
/// assert_eq!(5.weeks(), Duration::from_secs(5 * 604_800));
/// ```
///
/// Just like any other `std::time::Duration`, they can be added, subtracted,
/// etc.
///
/// ```rust
/// # use time::ext::NumericalStdDurationShort;
/// assert_eq!(2.seconds() + 500.milliseconds(), 2_500.milliseconds());
/// assert_eq!(2.seconds() - 500.milliseconds(), 1_500.milliseconds());
/// ```
///
/// When called on floating point values, any remainder of the floating point
/// value will be truncated. Keep in mind that floating point numbers are
/// inherently imprecise and have limited capacity.
pub trait NumericalStdDurationShort {
    /// Create a `std::time::Duration` from the number of nanoseconds.
    fn nanoseconds(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of microseconds.
    fn microseconds(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of milliseconds.
    fn milliseconds(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of seconds.
    fn seconds(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of minutes.
    fn minutes(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of hours.
    fn hours(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of days.
    fn days(self) -> StdDuration;
    /// Create a `std::time::Duration` from the number of weeks.
    fn weeks(self) -> StdDuration;
}

impl<T: NumericalStdDuration> NumericalStdDurationShort for T {
    fn nanoseconds(self) -> StdDuration {
        <Self as NumericalStdDuration>::std_nanoseconds(self)
    }

    fn microseconds(self) -> StdDuration {
        <Self as NumericalStdDuration>::std_microseconds(self)
    }

    fn milliseconds(self) -> StdDuration {
        <Self as NumericalStdDuration>::std_milliseconds(self)
    }

    fn seconds(self) -> StdDuration {
        <Self as NumericalStdDuration>::std_seconds(self)
    }

    fn minutes(self) -> StdDuration {
        <Self as NumericalStdDuration>::std_minutes(self)
    }

    fn hours(self) -> StdDuration {
        <Self as NumericalStdDuration>::std_hours(self)
    }

    fn days(self) -> StdDuration {
        <Self as NumericalStdDuration>::std_days(self)
    }

    fn weeks(self) -> StdDuration {
        <Self as NumericalStdDuration>::std_weeks(self)
    }
}
