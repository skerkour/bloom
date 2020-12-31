//! This module and its contents are not subject to stability guarantees and
//! should not be relied upon.
//!
//! These methods either exist to reduce duplication in code elsewhere or are
//! public only for usage in macros. The reasoning for a method's existence is
//! generally documented alongside the method.
//!
//! Failure to ensure that parameters to the contained functions are in range
//! will likely result in invalid behavior.

#![doc(hidden)]
#![allow(missing_debug_implementations, missing_copy_implementations)]

use crate::{days_in_year, is_leap_year, Weekday};
use const_fn::const_fn;

pub struct Time;

impl Time {
    /// Create a `Time` from its components.
    pub const fn from_hms_nanos_unchecked(
        hour: u8,
        minute: u8,
        second: u8,
        nanosecond: u32,
    ) -> crate::Time {
        crate::Time {
            hour,
            minute,
            second,
            nanosecond,
        }
    }
}

pub struct Date;

impl Date {
    // macros
    pub const fn from_yo_unchecked(year: i32, ordinal: u16) -> crate::Date {
        crate::Date {
            value: (year << 9) | ordinal as i32,
        }
    }

    // reduce duplication
    pub(crate) const fn from_ymd_unchecked(year: i32, month: u8, day: u8) -> crate::Date {
        /// Cumulative days through the beginning of a month in both common and
        /// leap years.
        const DAYS_CUMULATIVE_COMMON_LEAP: [[u16; 12]; 2] = [
            [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334],
            [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335],
        ];

        Date::from_yo_unchecked(
            year,
            DAYS_CUMULATIVE_COMMON_LEAP[is_leap_year(year) as usize][month as usize - 1]
                + day as u16,
        )
    }

    // reduce duplication
    #[const_fn("1.46")]
    pub(crate) const fn from_iso_ywd_unchecked(
        year: i32,
        week: u8,
        weekday: Weekday,
    ) -> crate::Date {
        let (ordinal, overflow) = (week as u16 * 7 + weekday.iso_weekday_number() as u16)
            .overflowing_sub(jan_weekday(year, 4) as u16 + 4);

        if overflow || ordinal == 0 {
            return Self::from_yo_unchecked(year - 1, ordinal.wrapping_add(days_in_year(year - 1)));
        }

        let days_in_cur_year = days_in_year(year);
        if ordinal > days_in_cur_year {
            Self::from_yo_unchecked(year + 1, ordinal - days_in_cur_year)
        } else {
            Self::from_yo_unchecked(year, ordinal)
        }
    }
}

/// Obtain the ISO weekday number of a day in January.
#[const_fn("1.46")]
pub(crate) const fn jan_weekday(year: i32, ordinal: i32) -> u8 {
    let adj_year = year - 1;
    let rem = (ordinal + adj_year + adj_year / 4 - adj_year / 100 + adj_year / 400 + 6) % 7;
    if rem < 0 {
        (rem + 7) as u8
    } else {
        rem as u8
    }
}
