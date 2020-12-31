//! Formatting helpers for a `Date`.

#![allow(non_snake_case)]

use crate::{
    error,
    format::{
        parse::{
            consume_padding, try_consume_digits, try_consume_exact_digits, try_consume_first_match,
        },
        Padding, ParseResult, ParsedItems,
    },
    Date, Weekday,
};
#[cfg(not(feature = "std"))]
use alloc::string::ToString;
use core::{
    fmt::{self, Formatter},
    num::{NonZeroU16, NonZeroU8},
};
#[allow(unused_imports)]
use standback::prelude::*;

/// Array of weekdays that corresponds to the localized values. This can be
/// zipped via an iterator to perform parsing easily.
const WEEKDAYS: [Weekday; 7] = [
    Weekday::Monday,
    Weekday::Tuesday,
    Weekday::Wednesday,
    Weekday::Thursday,
    Weekday::Friday,
    Weekday::Saturday,
    Weekday::Sunday,
];

/// Full weekday names
const WEEKDAYS_FULL: [&str; 7] = [
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
];

/// Abbreviated weekday names
const WEEKDAYS_ABBR: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

/// Full month names
const MONTHS_FULL: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

/// Abbreviated month names
const MONTHS_ABBR: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

/// Short day of the week
pub(crate) fn fmt_a(f: &mut Formatter<'_>, date: Date) -> fmt::Result {
    f.write_str(WEEKDAYS_ABBR[date.weekday().number_days_from_monday() as usize])
}

/// Short day of the week
pub(crate) fn parse_a(items: &mut ParsedItems, s: &mut &str) -> ParseResult<()> {
    items.weekday = Some(
        try_consume_first_match(s, WEEKDAYS_ABBR.iter().zip(WEEKDAYS.iter().cloned()))
            .ok_or(error::Parse::InvalidDayOfWeek)?,
    );

    Ok(())
}

/// Day of the week
pub(crate) fn fmt_A(f: &mut Formatter<'_>, date: Date) -> fmt::Result {
    f.write_str(WEEKDAYS_FULL[date.weekday().number_days_from_monday() as usize])
}

/// Day of the week
pub(crate) fn parse_A(items: &mut ParsedItems, s: &mut &str) -> ParseResult<()> {
    items.weekday = Some(
        try_consume_first_match(s, WEEKDAYS_FULL.iter().zip(WEEKDAYS.iter().cloned()))
            .ok_or(error::Parse::InvalidDayOfWeek)?,
    );

    Ok(())
}

/// Short month name
pub(crate) fn fmt_b(f: &mut Formatter<'_>, date: Date) -> fmt::Result {
    f.write_str(MONTHS_ABBR[date.month() as usize - 1])
}

/// Short month name
pub(crate) fn parse_b(items: &mut ParsedItems, s: &mut &str) -> ParseResult<()> {
    items.month = Some(
        try_consume_first_match(s, MONTHS_ABBR.iter().cloned().zip(1..))
            .and_then(NonZeroU8::new)
            .ok_or(error::Parse::InvalidMonth)?,
    );

    Ok(())
}

/// Month name
pub(crate) fn fmt_B(f: &mut Formatter<'_>, date: Date) -> fmt::Result {
    f.write_str(MONTHS_FULL[date.month() as usize - 1])
}

/// Month name
pub(crate) fn parse_B(items: &mut ParsedItems, s: &mut &str) -> ParseResult<()> {
    items.month = Some(
        try_consume_first_match(s, MONTHS_FULL.iter().cloned().zip(1..))
            .and_then(NonZeroU8::new)
            .ok_or(error::Parse::InvalidMonth)?,
    );

    Ok(())
}

/// Year divided by 100 and truncated to integer (`00`-`999`)
pub(crate) fn fmt_C(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    pad!(f, padding, 2, date.year() / 100)
}

/// Year divided by 100 and truncated to integer (`00`-`999`)
pub(crate) fn parse_C(items: &mut ParsedItems, s: &mut &str, padding: Padding) -> ParseResult<()> {
    let padding_length = consume_padding(s, padding, 1);
    items.year = Some(
        try_consume_digits::<i32>(s, 2 - padding_length, 3 - padding_length)
            .ok_or(error::Parse::InvalidYear)?
            * 100
            + items.year.unwrap_or(0).rem_euclid(100),
    );

    Ok(())
}

/// Day of the month, zero-padded (`01`-`31`)
pub(crate) fn fmt_d(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    pad!(f, padding, 2, date.day())
}

/// Day of the month, zero-padded (`01`-`31`)
pub(crate) fn parse_d(items: &mut ParsedItems, s: &mut &str, padding: Padding) -> ParseResult<()> {
    items.day = Some(
        try_consume_exact_digits(s, 2, padding)
            .and_then(NonZeroU8::new)
            .ok_or(error::Parse::InvalidDayOfMonth)?,
    );

    Ok(())
}

/// Week-based year, last two digits (`00`-`99`)
pub(crate) fn fmt_g(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    pad!(f, padding, 2, date.iso_year_week().0.rem_euclid(100))
}

/// Week-based year, last two digits (`00`-`99`)
pub(crate) fn parse_g(items: &mut ParsedItems, s: &mut &str, padding: Padding) -> ParseResult<()> {
    items.week_based_year = Some(
        items.week_based_year.unwrap_or(0) / 100 * 100
            + try_consume_exact_digits::<i32>(s, 2, padding).ok_or(error::Parse::InvalidYear)?,
    );

    Ok(())
}

/// Week-based year
pub(crate) fn fmt_G(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    let year = date.iso_year_week().0;

    if year >= 10_000 {
        f.write_str("+")?;
    }

    pad!(f, padding, 4, year)
}

/// Week-based year
pub(crate) fn parse_G(items: &mut ParsedItems, s: &mut &str, padding: Padding) -> ParseResult<()> {
    let sign = try_consume_first_match(s, [("+", 1), ("-", -1)].iter().cloned()).unwrap_or(1);

    consume_padding(s, padding, 4);

    items.week_based_year = Some(
        try_consume_digits(s, 1, 6)
            .map(|v: i32| sign * v)
            .ok_or(error::Parse::InvalidYear)?,
    );

    Ok(())
}

/// Day of the year, zero-padded to width 3 (`001`-`366`)
pub(crate) fn fmt_j(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    pad!(f, padding, 3, date.ordinal())
}

/// Day of the year, zero-padded to width 3 (`001`-`366`)
pub(crate) fn parse_j(items: &mut ParsedItems, s: &mut &str, padding: Padding) -> ParseResult<()> {
    items.ordinal_day = Some(
        try_consume_exact_digits(s, 3, padding)
            .and_then(NonZeroU16::new)
            .ok_or(error::Parse::InvalidDayOfYear)?,
    );

    Ok(())
}

/// Month of the year, zero-padded (`01`-`12`)
pub(crate) fn fmt_m(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    pad!(f, padding, 2, date.month())
}

/// Month of the year, zero-padded (`01`-`12`)
pub(crate) fn parse_m(items: &mut ParsedItems, s: &mut &str, padding: Padding) -> ParseResult<()> {
    items.month = Some(
        try_consume_exact_digits(s, 2, padding)
            .and_then(NonZeroU8::new)
            .ok_or(error::Parse::InvalidMonth)?,
    );

    Ok(())
}

/// ISO weekday (Monday = `1`, Sunday = `7`)
pub(crate) fn fmt_u(f: &mut Formatter<'_>, date: Date) -> fmt::Result {
    write!(f, "{}", date.weekday().iso_weekday_number())
}

/// ISO weekday (Monday = `1`, Sunday = `7`)
pub(crate) fn parse_u(items: &mut ParsedItems, s: &mut &str) -> ParseResult<()> {
    items.weekday = Some(
        try_consume_first_match(
            s,
            (1..).map(|d| d.to_string()).zip(WEEKDAYS.iter().cloned()),
        )
        .ok_or(error::Parse::InvalidDayOfWeek)?,
    );

    Ok(())
}

/// Sunday-based week number (`00`-`53`)
pub(crate) fn fmt_U(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    pad!(f, padding, 2, date.sunday_based_week())
}

/// Sunday-based week number (`00`-`53`)
pub(crate) fn parse_U(items: &mut ParsedItems, s: &mut &str, padding: Padding) -> ParseResult<()> {
    items.sunday_week =
        Some(try_consume_exact_digits(s, 2, padding).ok_or(error::Parse::InvalidWeek)?);

    Ok(())
}

/// ISO week number, zero-padded (`01`-`53`)
pub(crate) fn fmt_V(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    pad!(f, padding, 2, date.week())
}

/// ISO week number, zero-padded (`01`-`53`)
pub(crate) fn parse_V(items: &mut ParsedItems, s: &mut &str, padding: Padding) -> ParseResult<()> {
    items.iso_week = Some(
        try_consume_exact_digits(s, 2, padding)
            .and_then(NonZeroU8::new)
            .ok_or(error::Parse::InvalidWeek)?,
    );

    Ok(())
}

/// Weekday number (Sunday = `0`, Saturday = `6`)
pub(crate) fn fmt_w(f: &mut Formatter<'_>, date: Date) -> fmt::Result {
    write!(f, "{}", date.weekday().number_days_from_sunday())
}

/// Weekday number (Sunday = `0`, Saturday = `6`)
pub(crate) fn parse_w(items: &mut ParsedItems, s: &mut &str) -> ParseResult<()> {
    let mut weekdays = WEEKDAYS;
    weekdays.rotate_left(1);

    items.weekday = Some(
        try_consume_first_match(
            s,
            (0..)
                .map(|d: u8| d.to_string())
                .zip(weekdays.iter().cloned()),
        )
        .ok_or(error::Parse::InvalidDayOfWeek)?,
    );

    Ok(())
}

/// Monday-based week number (`00`-`53`)
pub(crate) fn fmt_W(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    pad!(f, padding, 2, date.monday_based_week())
}

/// Monday-based week number (`00`-`53`)
pub(crate) fn parse_W(items: &mut ParsedItems, s: &mut &str, padding: Padding) -> ParseResult<()> {
    items.monday_week =
        Some(try_consume_exact_digits(s, 2, padding).ok_or(error::Parse::InvalidWeek)?);

    Ok(())
}

/// Last two digits of year (`00`-`99`)
pub(crate) fn fmt_y(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    pad!(f, padding, 2, date.year().rem_euclid(100))
}

/// Last two digits of year (`00`-`99`)
pub(crate) fn parse_y(items: &mut ParsedItems, s: &mut &str, padding: Padding) -> ParseResult<()> {
    items.year = Some(
        items.year.unwrap_or(0) / 100 * 100
            + try_consume_exact_digits::<i32>(s, 2, padding).ok_or(error::Parse::InvalidYear)?,
    );

    Ok(())
}

/// Full year
pub(crate) fn fmt_Y(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    let year = date.year();

    if year >= 10_000 {
        f.write_str("+")?;
    }

    pad!(f, padding, 4, year)
}

/// Full year
pub(crate) fn parse_Y(items: &mut ParsedItems, s: &mut &str, padding: Padding) -> ParseResult<()> {
    let (sign, max_digits) =
        try_consume_first_match(s, [("+", (1, 6)), ("-", (-1, 6))].iter().cloned())
            .unwrap_or((1, 4));

    consume_padding(s, padding, 3);

    items.year = Some(
        try_consume_digits(s, 1, max_digits)
            .map(|v: i32| sign * v)
            .ok_or(error::Parse::InvalidYear)?,
    );

    Ok(())
}
