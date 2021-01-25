#![cfg_attr(not(feature = "protocol"), allow(unused))]
use chrono::{DateTime, LocalResult, TimeZone, Utc};

/// Converts a datetime object into a float timestamp.
pub fn datetime_to_timestamp(dt: &DateTime<Utc>) -> f64 {
    if dt.timestamp_subsec_nanos() == 0 {
        dt.timestamp() as f64
    } else {
        (dt.timestamp() as f64) + ((dt.timestamp_subsec_micros() as f64) / 1_000_000f64)
    }
}

pub fn timestamp_to_datetime(ts: f64) -> LocalResult<DateTime<Utc>> {
    let secs = ts as i64;
    let micros = (ts.fract() * 1_000_000f64) as u32;
    Utc.timestamp_opt(secs, micros * 1000)
}

pub mod ts_seconds_float {
    use chrono::{DateTime, LocalResult, TimeZone, Utc};
    use serde::{de, ser};
    use std::fmt;

    use super::timestamp_to_datetime;

    pub fn deserialize<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Ok(d.deserialize_any(SecondsTimestampVisitor)
            .map(|dt| dt.with_timezone(&Utc))?)
    }

    pub fn serialize<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        if dt.timestamp_subsec_nanos() == 0 {
            serializer.serialize_i64(dt.timestamp())
        } else {
            serializer.serialize_f64(
                (dt.timestamp() as f64) + ((dt.timestamp_subsec_micros() as f64) / 1_000_000f64),
            )
        }
    }

    struct SecondsTimestampVisitor;

    impl<'de> de::Visitor<'de> for SecondsTimestampVisitor {
        type Value = DateTime<Utc>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a unix timestamp")
        }

        fn visit_f64<E>(self, value: f64) -> Result<DateTime<Utc>, E>
        where
            E: de::Error,
        {
            match timestamp_to_datetime(value) {
                LocalResult::None => Err(E::custom(format!("No such local time for {}", value))),
                LocalResult::Single(date) => Ok(date),
                LocalResult::Ambiguous(t1, t2) => Err(E::custom(format!(
                    "Ambiguous local time, ranging from {:?} to {:?}",
                    t1, t2
                ))),
            }
        }

        fn visit_i64<E>(self, value: i64) -> Result<DateTime<Utc>, E>
        where
            E: de::Error,
        {
            Ok(Utc.timestamp_opt(value, 0).unwrap())
        }

        fn visit_u64<E>(self, value: u64) -> Result<DateTime<Utc>, E>
        where
            E: de::Error,
        {
            Ok(Utc.timestamp_opt(value as i64, 0).unwrap())
        }

        fn visit_str<E>(self, value: &str) -> Result<DateTime<Utc>, E>
        where
            E: de::Error,
        {
            value.parse().map_err(|e| E::custom(format!("{}", e)))
        }
    }
}
