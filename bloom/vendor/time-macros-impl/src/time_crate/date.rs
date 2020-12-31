use super::Weekday::{self, Friday, Monday, Saturday, Sunday, Thursday, Tuesday, Wednesday};
#[allow(unused_imports)]
use standback::prelude::*;

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0))
}

pub(crate) fn days_in_year(year: i32) -> u16 {
    365 + is_leap_year(year) as u16
}

/// The number of days in a month in both common and leap years.
const DAYS_IN_MONTH_COMMON_LEAP: [[u16; 12]; 2] = [
    [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
    [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
];

#[allow(clippy::cast_possible_truncation)]
pub(crate) fn days_in_year_month(year: i32, month: u8) -> u8 {
    DAYS_IN_MONTH_COMMON_LEAP[is_leap_year(year) as usize][month as usize - 1] as u8
}

pub(crate) fn weeks_in_year(year: i32) -> u8 {
    let weekday = Date { year, ordinal: 1 }.weekday();

    if (weekday == Thursday) || (weekday == Wednesday && is_leap_year(year)) {
        53
    } else {
        52
    }
}

pub(crate) struct Date {
    year: i32,
    pub(crate) ordinal: u16,
}

impl Date {
    pub(crate) const fn as_yo(&self) -> (i32, u16) {
        (self.year, self.ordinal)
    }

    pub(crate) fn month_day(&self) -> (u8, u8) {
        const CUMULATIVE_DAYS_IN_MONTH_COMMON_LEAP: [[u16; 11]; 2] = [
            [31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334],
            [31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335],
        ];

        let days = CUMULATIVE_DAYS_IN_MONTH_COMMON_LEAP[is_leap_year(self.year) as usize];
        let ordinal = self.ordinal;

        #[allow(clippy::cast_possible_truncation)]
        {
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
    }

    #[allow(unstable_name_collisions)]
    pub(crate) fn weekday(&self) -> Weekday {
        let (month, day) = self.month_day();

        let (month, adjusted_year) = if month < 3 {
            (month + 12, self.year - 1)
        } else {
            (month, self.year)
        };

        match (day as i32 + (13 * (month as i32 + 1)) / 5 + adjusted_year + adjusted_year / 4
            - adjusted_year / 100
            + adjusted_year / 400)
            .rem_euclid(7)
        {
            0 => Saturday,
            1 => Sunday,
            2 => Monday,
            3 => Tuesday,
            4 => Wednesday,
            5 => Thursday,
            6 => Friday,
            _ => unreachable!("A value mod 7 is always in the range 0..7"),
        }
    }

    pub(crate) fn from_iso_ywd_unchecked(year: i32, week: u8, iso_weekday_number: u8) -> Date {
        let ordinal = week as u16 * 7 + iso_weekday_number as u16
            - (Self::from_yo_unchecked(year, 4)
                .weekday()
                .iso_weekday_number() as u16
                + 3);

        if ordinal < 1 {
            return Self::from_yo_unchecked(year - 1, ordinal + days_in_year(year - 1));
        }

        let days_in_cur_year = days_in_year(year);
        if ordinal > days_in_cur_year {
            Self::from_yo_unchecked(year + 1, ordinal - days_in_cur_year)
        } else {
            Self::from_yo_unchecked(year, ordinal)
        }
    }

    pub(crate) fn from_ymd_unchecked(year: i32, month: u8, day: u8) -> Date {
        /// Cumulative days through the beginning of a month in both common and
        /// leap years.
        const DAYS_CUMULATIVE_COMMON_LEAP: [[u16; 12]; 2] = [
            [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334],
            [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335],
        ];

        let ordinal = DAYS_CUMULATIVE_COMMON_LEAP[is_leap_year(year) as usize][month as usize - 1];

        Date {
            year,
            ordinal: ordinal + day as u16,
        }
    }

    pub(crate) const fn from_yo_unchecked(year: i32, ordinal: u16) -> Date {
        Date { year, ordinal }
    }
}
