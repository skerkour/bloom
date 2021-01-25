use std::borrow::Cow;
use std::fmt;
use std::net::IpAddr;
use std::str;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// The Status of a Release Health Session.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    /// The session is healthy.
    ///
    /// This does not necessarily indicate that the session is still active.
    Ok,
    /// The session terminated normally.
    Exited,
    /// The session resulted in an application crash.
    Crashed,
    /// The session had an unexpected abrupt termination (not crashing).
    Abnormal,
}

impl Default for SessionStatus {
    fn default() -> Self {
        Self::Ok
    }
}

/// An error used when parsing `SessionStatus`.
#[derive(Debug, Error)]
#[error("invalid session status")]
pub struct ParseSessionStatusError;

impl str::FromStr for SessionStatus {
    type Err = ParseSessionStatusError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Ok(match string {
            "ok" => SessionStatus::Ok,
            "crashed" => SessionStatus::Crashed,
            "abnormal" => SessionStatus::Abnormal,
            "exited" => SessionStatus::Exited,
            _ => return Err(ParseSessionStatusError),
        })
    }
}

impl fmt::Display for SessionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SessionStatus::Ok => write!(f, "ok"),
            SessionStatus::Crashed => write!(f, "crashed"),
            SessionStatus::Abnormal => write!(f, "abnormal"),
            SessionStatus::Exited => write!(f, "exited"),
        }
    }
}

/// Additional attributes for Sessions.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionAttributes<'a> {
    /// The release version string.
    pub release: Cow<'a, str>,

    /// The environment identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<Cow<'a, str>>,

    /// The ip address of the user. This data is not persisted but used for filtering.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<IpAddr>,

    /// The user agent of the user. This data is not persisted but used for filtering.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_false(val: &bool) -> bool {
    !val
}

/// A Release Health Session.
///
/// Refer to the [Sessions](https://develop.sentry.dev/sdk/sessions/) documentation
/// for more details.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionUpdate<'a> {
    /// The session identifier.
    #[serde(rename = "sid", default = "Uuid::new_v4")]
    pub session_id: Uuid,

    /// The distinct identifier. Should be device or user ID.
    #[serde(rename = "did", default)]
    pub distinct_id: Option<String>,

    /// An optional logical clock.
    #[serde(rename = "seq", default, skip_serializing_if = "Option::is_none")]
    pub sequence: Option<u64>,

    /// The timestamp of when the session change event was created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,

    /// The timestamp of when the session itself started.
    #[serde(default = "Utc::now")]
    pub started: DateTime<Utc>,

    /// A flag that indicates that this is the initial transmission of the session.
    #[serde(default, skip_serializing_if = "is_false")]
    pub init: bool,

    /// An optional duration of the session so far.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,

    /// The status of the session.
    #[serde(default)]
    pub status: SessionStatus,

    /// The number of errors that ocurred.
    #[serde(default)]
    pub errors: u64,

    /// The session event attributes.
    #[serde(rename = "attrs")]
    pub attributes: SessionAttributes<'a>,
}
