use standback::convert::TryFrom;

// (year, ordinal)
#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct Date(pub(crate) i32, pub(crate) u16);

impl From<crate::Date> for Date {
    fn from(original: crate::Date) -> Self {
        Self(original.year(), original.ordinal())
    }
}

impl TryFrom<Date> for crate::Date {
    type Error = &'static str;

    fn try_from(original: Date) -> Result<Self, Self::Error> {
        Self::try_from_yo(original.0, original.1).map_err(|_| "invalid value")
    }
}
