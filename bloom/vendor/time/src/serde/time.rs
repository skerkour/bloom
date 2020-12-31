// (seconds since midnight, nanoseconds within second)
#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct Time(pub(crate) u32, pub(crate) u32);

impl From<crate::Time> for Time {
    fn from(original: crate::Time) -> Self {
        Self(
            original.hour() as u32 * 3_600
                + original.minute() as u32 * 60
                + original.second() as u32,
            original.nanosecond(),
        )
    }
}

impl From<Time> for crate::Time {
    fn from(original: Time) -> Self {
        Self::from_nanoseconds_since_midnight(original.0 as u64 * 1_000_000_000 + original.1 as u64)
    }
}
