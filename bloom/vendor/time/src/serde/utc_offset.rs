// seconds offset from UTC, positive is east
#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct UtcOffset(i32);

impl From<crate::UtcOffset> for UtcOffset {
    fn from(original: crate::UtcOffset) -> Self {
        Self(original.as_seconds())
    }
}

impl From<UtcOffset> for crate::UtcOffset {
    fn from(original: UtcOffset) -> Self {
        Self::seconds(original.0)
    }
}
