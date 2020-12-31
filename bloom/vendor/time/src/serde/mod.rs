//! Differential formats for serde.

// Types with guaranteed stable serde representations.
//
// This allows for the ability to change the internal structure of a type while
// maintaining backwards compatibility.
//
// Strings are avoided where possible to allow for optimal representations in
// various binary forms.

#![allow(clippy::missing_docs_in_private_items)]

// OffsetDateTime is in the primitive_date_time module.

mod date;
mod duration;
mod primitive_date_time;
mod sign;
mod time;
pub mod timestamp;
mod utc_offset;
mod weekday;

pub(crate) use self::time::Time;
pub(crate) use date::Date;
pub(crate) use duration::Duration;
pub(crate) use primitive_date_time::PrimitiveDateTime;
#[allow(deprecated)]
pub(crate) use sign::Sign;
pub(crate) use utc_offset::UtcOffset;
pub(crate) use weekday::Weekday;
