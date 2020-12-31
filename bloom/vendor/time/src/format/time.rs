//! Formatting helpers for a `Time`.

#![allow(non_snake_case)]

use crate::{
    error,
    format::{
        parse::{
            try_consume_exact_digits, try_consume_first_match,
            AmPm::{AM, PM},
        },
        Padding, ParseResult, ParsedItems,
    },
    Time,
};
use core::{
    fmt::{self, Formatter},
    num::NonZeroU8,
};
#[allow(unused_imports)]
use standback::prelude::*;

/// Hour in 24h format (`00`-`23`)
pub(crate) fn fmt_H(f: &mut Formatter<'_>, time: Time, padding: Padding) -> fmt::Result {
    pad!(f, padding, 2, time.hour())
}

/// Hour in 24h format (`00`-`23`)
pub(crate) fn parse_H(items: &mut ParsedItems, s: &mut &str, padding: Padding) -> ParseResult<()> {
    items.hour_24 = Some(try_consume_exact_digits(s, 2, padding).ok_or(error::Parse::InvalidHour)?);
    Ok(())
}

/// Hour in 12h format (`01`-`12`)
pub(crate) fn fmt_I(f: &mut Formatter<'_>, time: Time, padding: Padding) -> fmt::Result {
    pad!(f, padding, 2, (time.hour() as i8 - 1).rem_euclid(12) + 1)
}

/// Hour in 12h format (`01`-`12`)
pub(crate) fn parse_I(items: &mut ParsedItems, s: &mut &str, padding: Padding) -> ParseResult<()> {
    items.hour_12 = Some(
        try_consume_exact_digits(s, 2, padding)
            .and_then(NonZeroU8::new)
            .ok_or(error::Parse::InvalidHour)?,
    );
    Ok(())
}

/// Minutes, zero-padded (`00`-`59`)
pub(crate) fn fmt_M(f: &mut Formatter<'_>, time: Time, padding: Padding) -> fmt::Result {
    pad!(f, padding, 2, time.minute())
}

/// Minutes, zero-added (`00`-`59`)
pub(crate) fn parse_M(items: &mut ParsedItems, s: &mut &str, padding: Padding) -> ParseResult<()> {
    items.minute =
        Some(try_consume_exact_digits(s, 2, padding).ok_or(error::Parse::InvalidMinute)?);
    Ok(())
}

/// Subsecond nanoseconds. Always 9 digits
pub(crate) fn fmt_N(f: &mut Formatter<'_>, time: Time) -> fmt::Result {
    write!(f, "{:09}", time.nanosecond)
}

/// Subsecond nanoseconds. Always 9 digits
pub(crate) fn parse_N(items: &mut ParsedItems, s: &mut &str) -> ParseResult<()> {
    items.nanosecond =
        Some(try_consume_exact_digits(s, 9, Padding::None).ok_or(error::Parse::InvalidNanosecond)?);
    Ok(())
}

/// am/pm
pub(crate) fn fmt_p(f: &mut Formatter<'_>, time: Time) -> fmt::Result {
    if time.hour() < 12 {
        f.write_str("am")
    } else {
        f.write_str("pm")
    }
}

/// am/pm
pub(crate) fn parse_p(items: &mut ParsedItems, s: &mut &str) -> ParseResult<()> {
    items.am_pm = Some(
        try_consume_first_match(s, [("am", AM), ("pm", PM)].iter().cloned())
            .ok_or(error::Parse::InvalidAmPm)?,
    );
    Ok(())
}

/// AM/PM
pub(crate) fn fmt_P(f: &mut Formatter<'_>, time: Time) -> fmt::Result {
    if time.hour() < 12 {
        f.write_str("AM")
    } else {
        f.write_str("PM")
    }
}

/// AM/PM
pub(crate) fn parse_P(items: &mut ParsedItems, s: &mut &str) -> ParseResult<()> {
    items.am_pm = Some(
        try_consume_first_match(s, [("AM", AM), ("PM", PM)].iter().cloned())
            .ok_or(error::Parse::InvalidAmPm)?,
    );
    Ok(())
}

/// Seconds, zero-padded (`00`-`59`)
pub(crate) fn fmt_S(f: &mut Formatter<'_>, time: Time, padding: Padding) -> fmt::Result {
    pad!(f, padding, 2, time.second())
}

/// Seconds, zero-added (`00`-`59`)
pub(crate) fn parse_S(items: &mut ParsedItems, s: &mut &str, padding: Padding) -> ParseResult<()> {
    items.second =
        Some(try_consume_exact_digits(s, 2, padding).ok_or(error::Parse::InvalidSecond)?);
    Ok(())
}
