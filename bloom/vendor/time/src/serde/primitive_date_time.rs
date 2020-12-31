use crate::OffsetDateTime;
use standback::convert::{TryFrom, TryInto};

// Date followed by Time
#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct PrimitiveDateTime(i32, u16, u32, u32);

impl From<crate::PrimitiveDateTime> for PrimitiveDateTime {
    fn from(original: crate::PrimitiveDateTime) -> Self {
        let date: crate::serde::Date = original.date().into();
        let time: crate::serde::Time = original.time().into();
        Self(date.0, date.1, time.0, time.1)
    }
}

impl TryFrom<PrimitiveDateTime> for crate::PrimitiveDateTime {
    type Error = &'static str;

    fn try_from(original: PrimitiveDateTime) -> Result<Self, Self::Error> {
        let date = crate::serde::Date(original.0, original.1);
        let time = crate::serde::Time(original.2, original.3);
        Ok(Self::new(date.try_into()?, time.into()))
    }
}

// TODO(0.3) Store the offset as well.
impl From<OffsetDateTime> for PrimitiveDateTime {
    fn from(original: OffsetDateTime) -> Self {
        // Simplify handling by always using UTC.
        let original = original.to_offset(crate::UtcOffset::UTC);
        let date: crate::serde::Date = original.date().into();
        let time: crate::serde::Time = original.time().into();
        Self(date.0, date.1, time.0, time.1)
    }
}

impl TryFrom<PrimitiveDateTime> for OffsetDateTime {
    type Error = &'static str;

    fn try_from(original: PrimitiveDateTime) -> Result<Self, Self::Error> {
        let date = crate::serde::Date(original.0, original.1);
        let time = crate::serde::Time(original.2, original.3);
        Ok(crate::PrimitiveDateTime::new(date.try_into()?, time.into()).assume_utc())
    }
}
