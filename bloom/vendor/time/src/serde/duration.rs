// (seconds, nanoseconds)
#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct Duration(i64, i32);

impl From<crate::Duration> for Duration {
    fn from(original: crate::Duration) -> Self {
        Self(original.whole_seconds(), original.subsec_nanoseconds())
    }
}

impl From<Duration> for crate::Duration {
    fn from(original: Duration) -> Self {
        Self::new(original.0, original.1)
    }
}
