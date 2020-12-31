//! Serialization and deserialization.
//!
//! By default, serialization and deserialization go through ULID's 26-character
//! canonical string representation as set by the ULID standard.
//!
//! ULIDs can optionally be serialized as u128 integers using the `ulid_as_u128`
//! module. See the module's documentation for examples.

use super::{Ulid, ULID_LEN};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl Serialize for Ulid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut buffer = [0; ULID_LEN];
        let text = self.to_str(&mut buffer).unwrap();
        text.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Ulid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let deserialized_str = String::deserialize(deserializer)?;
        Self::from_string(&deserialized_str).map_err(serde::de::Error::custom)
    }
}

/// Serialization and deserialization of ULIDs through their inner u128 type.
///
/// To use it, annotate a field with
/// `#[serde(with = "ulid_as_u128")]`,
/// `#[serde(serialize_with = "ulid_as_u128")]`, or
/// `#[serde(deserialize_with = "ulid_as_u128")]`.
///
/// # Examples
/// ```
/// # use ulid::Ulid;
/// # use ulid::serde::ulid_as_u128;
/// # use serde::{Serialize, Deserialize};
/// #[derive(Serialize, Deserialize)]
/// struct U128Example {
///     #[serde(with = "ulid_as_u128")]
///     identifier: Ulid
/// }
/// ```
pub mod ulid_as_u128 {
    use super::Ulid;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    /// Serializes a ULID as a u128 type.
    pub fn serialize<S>(value: &Ulid, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        value.0.serialize(serializer)
    }

    /// Deserializes a ULID from a u128 type.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Ulid, D::Error>
    where
        D: Deserializer<'de>,
    {
        let deserialized_u128 = u128::deserialize(deserializer)?;
        Ok(Ulid(deserialized_u128))
    }
}

/// Serialization and deserialization of ULIDs through UUID strings.
///
/// To use this module, annotate a field with
/// `#[serde(with = "ulid_as_uuid")]`,
/// `#[serde(serialize_with = "ulid_as_uuid")]`, or
/// `#[serde(deserialize_with = "ulid_as_uuid")]`.
///
/// # Examples
/// ```
/// # use ulid::Ulid;
/// # use ulid::serde::ulid_as_uuid;
/// # use serde::{Serialize, Deserialize};
/// #[derive(Serialize, Deserialize)]
/// struct UuidExample {
///     #[serde(with = "ulid_as_uuid")]
///     identifier: Ulid
/// }
/// ```
#[cfg(all(feature = "uuid", feature = "serde"))]
pub mod ulid_as_uuid {
    use crate::Ulid;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use uuid::Uuid;

    /// Converts the ULID to a UUID and serializes it as a string.
    pub fn serialize<S>(value: &Ulid, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let uuid: Uuid = (*value).into();
        uuid.to_string().serialize(serializer)
    }

    /// Deserializes a ULID from a string containing a UUID.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Ulid, D::Error>
    where
        D: Deserializer<'de>,
    {
        let de_string = String::deserialize(deserializer)?;
        let de_uuid = Uuid::parse_str(&de_string).map_err(serde::de::Error::custom)?;
        Ok(Ulid::from(de_uuid))
    }
}
