#![allow(deprecated)]

use standback::convert::TryFrom;

// positive => 1
// negative => -1
// zero => 0
#[deprecated(
    since = "0.2.7",
    note = "The only use for this struct has been replaced. See the main struct for details."
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct Sign(i8);

impl From<crate::Sign> for Sign {
    fn from(original: crate::Sign) -> Self {
        match original {
            crate::Sign::Positive => Self(1),
            crate::Sign::Negative => Self(-1),
            crate::Sign::Zero => Self(0),
        }
    }
}

impl TryFrom<Sign> for crate::Sign {
    type Error = &'static str;

    fn try_from(original: Sign) -> Result<Self, Self::Error> {
        match original {
            Sign(1) => Ok(crate::Sign::Positive),
            Sign(-1) => Ok(crate::Sign::Negative),
            Sign(0) => Ok(crate::Sign::Zero),
            _ => Err("invalid value"),
        }
    }
}
