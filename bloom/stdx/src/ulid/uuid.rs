//! Conversions between ULID and UUID.

use super::Ulid;
use uuid::Uuid;

impl From<Uuid> for Ulid {
    fn from(uuid: Uuid) -> Self {
        Ulid(uuid.as_u128())
    }
}

impl From<Ulid> for Uuid {
    fn from(ulid: Ulid) -> Self {
        Uuid::from_u128(ulid.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn uuid_cycle() {
        let ulid = Ulid::new();
        let uuid: Uuid = ulid.into();
        let ulid2: Ulid = uuid.into();

        assert_eq!(ulid, ulid2);
    }

    #[test]
    fn uuid_str_cycle() {
        let uuid_txt = "771a3bce-02e9-4428-a68e-b1e7e82b7f9f";
        let ulid_txt = "3Q38XWW0Q98GMAD3NHWZM2PZWZ";

        let ulid: Ulid = Uuid::parse_str(uuid_txt).unwrap().into();
        assert_eq!(ulid.to_string(), ulid_txt);

        let uuid: Uuid = ulid.into();
        assert_eq!(uuid.to_string(), uuid_txt);
    }
}
