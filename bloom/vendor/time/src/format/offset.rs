//! Formatting helpers for a `UtcOffset`.

#![allow(non_snake_case)]

use crate::{
    error,
    format::{
        parse::{try_consume_exact_digits, try_consume_first_match},
        Padding, ParsedItems,
    },
    ParseResult, UtcOffset,
};
use core::fmt::{self, Formatter};

/// UTC offset
pub(crate) fn fmt_z(f: &mut Formatter<'_>, offset: UtcOffset) -> fmt::Result {
    let offset = offset.as_duration();

    write!(
        f,
        "{}{:02}{:02}",
        if offset.is_negative() { '-' } else { '+' },
        offset.whole_hours().abs(),
        (offset.whole_minutes() - 60 * offset.whole_hours()).abs()
    )
}

/// UTC offset
pub(crate) fn parse_z(items: &mut ParsedItems, s: &mut &str) -> ParseResult<()> {
    let sign = try_consume_first_match(s, [("+", 1), ("-", -1)].iter().cloned())
        .ok_or(error::Parse::InvalidOffset)?;

    let hours: i16 =
        try_consume_exact_digits(s, 2, Padding::Zero).ok_or(error::Parse::InvalidOffset)?;

    let minutes: i16 =
        try_consume_exact_digits(s, 2, Padding::Zero).ok_or(error::Parse::InvalidOffset)?;

    items.offset = UtcOffset::minutes(sign * (hours * 60 + minutes)).into();
    Ok(())
}
