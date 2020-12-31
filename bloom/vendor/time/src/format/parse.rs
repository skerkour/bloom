//! Parsing for various types.

use crate::{
    error,
    format::{parse_fmt_string, well_known, FormatItem, Padding, Specifier},
    Format, UtcOffset, Weekday,
};
#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
use core::{
    fmt::{self, Display, Formatter},
    num::{NonZeroU16, NonZeroU8},
    str::FromStr,
};

/// Helper type to avoid repeating the error type.
pub(crate) type ParseResult<T> = Result<T, Error>;

/// An error occurred while parsing.
#[cfg_attr(__time_02_supports_non_exhaustive, non_exhaustive)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Error {
    /// The nanosecond present was not valid.
    InvalidNanosecond,
    /// The second present was not valid.
    InvalidSecond,
    /// The minute present was not valid.
    InvalidMinute,
    /// The hour present was not valid.
    InvalidHour,
    /// The AM/PM was not valid.
    InvalidAmPm,
    /// The month present was not valid.
    InvalidMonth,
    /// The year present was not valid.
    InvalidYear,
    /// The week present was not valid.
    InvalidWeek,
    /// The day of week present was not valid.
    InvalidDayOfWeek,
    /// The day of month present was not valid.
    InvalidDayOfMonth,
    /// The day of year present was not valid.
    InvalidDayOfYear,
    /// The UTC offset present was not valid.
    InvalidOffset,
    /// There was no character following a `%`.
    MissingFormatSpecifier,
    /// The character following `%` is not valid.
    InvalidFormatSpecifier(char),
    /// A character literal was expected to be present but was not.
    UnexpectedCharacter {
        /// The character that was expected to be present.
        expected: char,
        /// The character that was present in the string.
        actual: char,
    },
    /// The string ended, but there should be more content.
    UnexpectedEndOfString,
    /// There was not enough information provided to create the requested type.
    InsufficientInformation,
    /// A component was out of range.
    ComponentOutOfRange(Box<error::ComponentRange>),
    #[cfg(not(__time_02_supports_non_exhaustive))]
    #[doc(hidden)]
    __NonExhaustive,
}

impl From<error::ComponentRange> for Error {
    fn from(error: error::ComponentRange) -> Self {
        Error::ComponentOutOfRange(Box::new(error))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            InvalidNanosecond => f.write_str("invalid nanosecond"),
            InvalidSecond => f.write_str("invalid second"),
            InvalidMinute => f.write_str("invalid minute"),
            InvalidHour => f.write_str("invalid hour"),
            InvalidAmPm => f.write_str("invalid am/pm"),
            InvalidMonth => f.write_str("invalid month"),
            InvalidYear => f.write_str("invalid year"),
            InvalidWeek => f.write_str("invalid week"),
            InvalidDayOfWeek => f.write_str("invalid day of week"),
            InvalidDayOfMonth => f.write_str("invalid day of month"),
            InvalidDayOfYear => f.write_str("invalid day of year"),
            InvalidOffset => f.write_str("invalid offset"),
            MissingFormatSpecifier => f.write_str("missing format specifier after `%`"),
            InvalidFormatSpecifier(c) => write!(f, "invalid format specifier `{}` after `%`", c),
            UnexpectedCharacter { expected, actual } => {
                write!(f, "expected character `{}`, found `{}`", expected, actual)
            }
            UnexpectedEndOfString => f.write_str("unexpected end of string"),
            InsufficientInformation => {
                f.write_str("insufficient information provided to create the requested type")
            }
            ComponentOutOfRange(e) => write!(f, "{}", e),
            #[cfg(not(__time_02_supports_non_exhaustive))]
            __NonExhaustive => unreachable!(),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ComponentOutOfRange(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}

/// A value representing a time that is either "AM" or "PM".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AmPm {
    /// A time before noon.
    AM,
    /// A time at or after noon.
    PM,
}

/// All information gathered from parsing a provided string.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ParsedItems {
    /// Year the ISO week belongs to.
    pub(crate) week_based_year: Option<i32>,
    /// The year the month, day, and ordinal day belong to.
    pub(crate) year: Option<i32>,
    /// One-indexed month number.
    pub(crate) month: Option<NonZeroU8>,
    /// Day of the month.
    pub(crate) day: Option<NonZeroU8>,
    /// Day of the week.
    pub(crate) weekday: Option<Weekday>,
    /// Day of the year.
    pub(crate) ordinal_day: Option<NonZeroU16>,
    /// ISO week within the year. Week 1 contains the year's first Thursday.
    pub(crate) iso_week: Option<NonZeroU8>,
    /// Week number, counted from the first Sunday. May be zero.
    pub(crate) sunday_week: Option<u8>,
    /// Week number, counted from the first Monday. May be zero.
    pub(crate) monday_week: Option<u8>,
    /// Hour in the 12-hour clock.
    pub(crate) hour_12: Option<NonZeroU8>,
    /// Hour in the 24-hour clock.
    pub(crate) hour_24: Option<u8>,
    /// Minute within the hour.
    pub(crate) minute: Option<u8>,
    /// Second within the minute.
    pub(crate) second: Option<u8>,
    /// Nanosecond within the second.
    pub(crate) nanosecond: Option<u32>,
    /// The UTC offset of the datetime.
    pub(crate) offset: Option<UtcOffset>,
    /// Whether the hour indicated is AM or PM.
    pub(crate) am_pm: Option<AmPm>,
}

impl ParsedItems {
    /// Create a new `ParsedItems` with nothing known.
    pub(crate) const fn new() -> Self {
        Self {
            week_based_year: None,
            year: None,
            month: None,
            day: None,
            weekday: None,
            ordinal_day: None,
            iso_week: None,
            sunday_week: None,
            monday_week: None,
            hour_12: None,
            hour_24: None,
            minute: None,
            second: None,
            nanosecond: None,
            offset: None,
            am_pm: None,
        }
    }
}

/// Attempt to consume the provided character.
pub(crate) fn try_consume_char(s: &mut &str, expected: char) -> ParseResult<()> {
    match s.char_indices().next() {
        Some((index, actual_char)) if actual_char == expected => {
            *s = &s[(index + actual_char.len_utf8())..];
            Ok(())
        }
        Some((_, actual)) => Err(Error::UnexpectedCharacter { expected, actual }),
        None => Err(Error::UnexpectedEndOfString),
    }
}

/// Attempt to consume the provided character, ignoring case.
pub(crate) fn try_consume_char_case_insensitive(s: &mut &str, expected: char) -> ParseResult<()> {
    match s.char_indices().next() {
        Some((index, actual_char)) if actual_char.eq_ignore_ascii_case(&expected) => {
            *s = &s[(index + actual_char.len_utf8())..];
            Ok(())
        }
        Some((_, actual)) => Err(Error::UnexpectedCharacter { expected, actual }),
        None => Err(Error::UnexpectedEndOfString),
    }
}

/// Attempt to consume the provided string.
pub(crate) fn try_consume_str(s: &mut &str, expected: &str) -> ParseResult<()> {
    if s.starts_with(expected) {
        *s = &s[expected.len()..];
        Ok(())
    } else {
        // Iterate through the characters, returning the error where differing.
        for c in expected.chars() {
            try_consume_char(s, c)?;
        }
        // TODO Find a way to allow the compiler to prove the following is not
        // necessary.
        unreachable!("The previous loop should always cause the function to return.");
    }
}

/// Attempt to find one of the strings provided, returning the first value.
pub(crate) fn try_consume_first_match<T: Copy>(
    s: &mut &str,
    opts: impl IntoIterator<Item = (impl AsRef<str>, T)>,
) -> Option<T> {
    opts.into_iter().find_map(|(expected, value)| {
        if s.starts_with(expected.as_ref()) {
            *s = &s[expected.as_ref().len()..];
            Some(value)
        } else {
            None
        }
    })
}

/// Attempt to consume a number of digits. Consumes the maximum amount possible
/// within the range provided.
pub(crate) fn try_consume_digits<T: FromStr>(
    s: &mut &str,
    min_digits: usize,
    max_digits: usize,
) -> Option<T> {
    // Determine how many digits the string starts with, up to the upper limit
    // of the range.
    let len = s
        .chars()
        .take(max_digits)
        .take_while(char::is_ascii_digit)
        .count();

    // We don't have enough digits.
    if len < min_digits {
        return None;
    }

    // Because we're only dealing with ASCII digits here, we know that the
    // length is equal to the number of bytes, as ASCII values are always one
    // byte in Unicode.
    let digits = &s[..len];
    *s = &s[len..];
    digits.parse::<T>().ok()
}

/// Attempt to consume an exact number of digits.
pub(crate) fn try_consume_exact_digits<T: FromStr>(
    s: &mut &str,
    num_digits: usize,
    padding: Padding,
) -> Option<T> {
    let pad_size = match padding {
        Padding::Space => consume_padding(s, padding, num_digits - 1),
        _ => 0,
    };

    if padding == Padding::None {
        try_consume_digits(s, 1, num_digits - pad_size)
    } else {
        // Ensure all the necessary characters are ASCII digits.
        if !s
            .chars()
            .take(num_digits - pad_size)
            .all(|c| c.is_ascii_digit())
        {
            return None;
        }

        // Ensure the string is long enough to perform the slicing.
        if (num_digits - pad_size) > s.len() {
            return None;
        }

        // Because we're only dealing with ASCII digits here, we know that the
        // length is equal to the number of bytes, as ASCII values are always one
        // byte in Unicode.
        let digits = &s[..(num_digits - pad_size)];
        *s = &s[(num_digits - pad_size)..];
        digits.parse::<T>().ok()
    }
}

/// Consume all leading padding up to the number of characters.
///
/// Returns the number of characters trimmed.
pub(crate) fn consume_padding(s: &mut &str, padding: Padding, max_chars: usize) -> usize {
    let pad_char = match padding {
        Padding::Space => ' ',
        Padding::Zero => '0',
        Padding::None => return 0,
    };

    let pad_width = s
        .chars()
        .take(max_chars)
        .take_while(|&c| c == pad_char)
        .count();
    *s = &s[pad_width..];
    pad_width
}

/// Attempt to parse the string with the provided format, returning a struct
/// containing all information found.
#[allow(clippy::too_many_lines)]
pub(crate) fn parse(s: &str, format: &Format) -> ParseResult<ParsedItems> {
    use super::{date, offset, time};

    // Make a copy of the provided string, letting us mutate as necessary.
    let mut s = <&str>::clone(&s);

    let mut items = ParsedItems::new();

    /// Parse the provided specifier with the given parameters.
    macro_rules! parse {
        ($module:ident :: $specifier_fn:ident $( ( $($params:expr),* ) )?) => {
            $module::$specifier_fn(&mut items, &mut s, $( $($params),* )?)?
        };
    }

    macro_rules! parse_char {
        ($c:literal) => {
            try_consume_char(&mut s, $c)?
        };
    }

    match &format {
        Format::Rfc3339 => well_known::rfc3339::parse(&mut items, &mut s)?,
        Format::Custom(format) => {
            for item in parse_fmt_string(format) {
                match item {
                    FormatItem::Literal(expected) => try_consume_str(&mut s, expected)?,
                    FormatItem::Specifier(specifier) => {
                        use Specifier::*;
                        match specifier {
                            a => parse!(date::parse_a),
                            A => parse!(date::parse_A),
                            b => parse!(date::parse_b),
                            B => parse!(date::parse_B),
                            c => {
                                parse!(date::parse_a);
                                parse_char!(' ');
                                parse!(date::parse_b);
                                parse_char!(' ');
                                parse!(date::parse_d(Padding::None));
                                parse_char!(' ');
                                parse!(time::parse_H(Padding::None));
                                parse_char!(':');
                                parse!(time::parse_M(Padding::Zero));
                                parse_char!(':');
                                parse!(time::parse_S(Padding::Zero));
                                parse_char!(' ');
                                parse!(date::parse_Y(Padding::None));
                            }
                            C { padding } => parse!(date::parse_C(padding)),
                            d { padding } => parse!(date::parse_d(padding)),
                            D => {
                                parse!(date::parse_m(Padding::None));
                                parse_char!('/');
                                parse!(date::parse_d(Padding::Zero));
                                parse_char!('/');
                                parse!(date::parse_y(Padding::Zero));
                            }
                            F => {
                                parse!(date::parse_Y(Padding::None));
                                parse_char!('-');
                                parse!(date::parse_m(Padding::Zero));
                                parse_char!('-');
                                parse!(date::parse_d(Padding::Zero));
                            }
                            g { padding } => parse!(date::parse_g(padding)),
                            G { padding } => parse!(date::parse_G(padding)),
                            H { padding } => parse!(time::parse_H(padding)),
                            I { padding } => parse!(time::parse_I(padding)),
                            j { padding } => parse!(date::parse_j(padding)),
                            M { padding } => parse!(time::parse_M(padding)),
                            m { padding } => parse!(date::parse_m(padding)),
                            N => parse!(time::parse_N),
                            p => parse!(time::parse_p),
                            P => parse!(time::parse_P),
                            r => {
                                parse!(time::parse_I(Padding::None));
                                parse_char!(':');
                                parse!(time::parse_M(Padding::Zero));
                                parse_char!(':');
                                parse!(time::parse_S(Padding::Zero));
                                parse_char!(' ');
                                parse!(time::parse_p);
                            }
                            R => {
                                parse!(time::parse_H(Padding::None));
                                parse_char!(':');
                                parse!(time::parse_M(Padding::Zero));
                            }
                            S { padding } => parse!(time::parse_S(padding)),
                            T => {
                                parse!(time::parse_H(Padding::None));
                                parse_char!(':');
                                parse!(time::parse_M(Padding::Zero));
                                parse_char!(':');
                                parse!(time::parse_S(Padding::Zero));
                            }
                            u => parse!(date::parse_u),
                            U { padding } => parse!(date::parse_U(padding)),
                            V { padding } => parse!(date::parse_V(padding)),
                            w => parse!(date::parse_w),
                            W { padding } => parse!(date::parse_W(padding)),
                            y { padding } => parse!(date::parse_y(padding)),
                            z => parse!(offset::parse_z),
                            Y { padding } => parse!(date::parse_Y(padding)),
                        }
                    }
                }
            }
        }
        #[cfg(not(__time_02_supports_non_exhaustive))]
        Format::__NonExhaustive => unreachable!(),
    }

    Ok(items)
}
