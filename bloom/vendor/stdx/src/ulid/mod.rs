#![warn(missing_docs)]
//! # ulid-rs
//!
//! This is a Rust implementation of the [ulid][ulid] project which provides
//! Universally Unique Lexicographically Sortable Identifiers.
//!
//! [ulid]: https://github.com/alizain/ulid
//!
//!
//! ## Quickstart
//!
//! ```rust
//! # use stdx::ulid::Ulid;
//! // Generate a ulid
//! let ulid = Ulid::new();
//!
//! // Generate a string for a ulid
//! let s = ulid.to_string();
//!
//! // Create from a String
//! let res = Ulid::from_string(&s);
//! assert_eq!(ulid, res.unwrap());
//!
//! // Or using FromStr
//! let res = s.parse();
//! assert_eq!(ulid, res.unwrap());
//!
//! ```

mod base32;
pub mod serde;
mod uuid;

use chrono::prelude::{DateTime, TimeZone, Utc};
use std::fmt;
use std::str::FromStr;

pub use base32::{DecodeError, EncodeError, ULID_LEN};

macro_rules! bitmask {
    ($len:expr) => {
        ((1 << $len) - 1)
    };
}

/// A Ulid is a unique 128-bit lexicographically sortable identifier
///
/// Canonically, it is represented as a 26 character Crockford Base32 encoded
/// string.
///
/// Of the 128-bits, the first 48 are a unix timestamp in milliseconds. The
/// remaining 80 are random. The first 48 provide for lexicographic sorting and
/// the remaining 80 ensure that the identifier is unique.
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Ulid(pub u128);

impl Ulid {
    const TIME_BITS: u8 = 48;
    const RAND_BITS: u8 = 80;

    /// Creates a new Ulid with the current time (UTC)
    ///
    /// # Example
    /// ```rust
    /// use stdx::ulid::Ulid;
    ///
    /// let my_ulid = Ulid::new();
    /// ```
    pub fn new() -> Ulid {
        Ulid::from_datetime(Utc::now())
    }

    /// Creates a new Ulid using data from the given random number generator
    ///
    /// # Example
    /// ```rust
    /// use rand::FromEntropy;
    /// use rand::rngs::SmallRng;
    /// use stdx::ulid::Ulid;
    ///
    /// let mut rng = SmallRng::from_entropy();
    /// let ulid = Ulid::with_source(&mut rng);
    /// ```
    pub fn with_source<R: rand::Rng>(source: &mut R) -> Ulid {
        Ulid::from_datetime_with_source(Utc::now(), source)
    }

    /// Creates a new Ulid with the given datetime
    ///
    /// This can be useful when migrating data to use stdx::ulid identifiers
    ///
    /// # Example
    /// ```rust
    /// use stdx::chrono::offset::Utc;
    /// use stdx::ulid::Ulid;
    ///
    /// let ulid = Ulid::from_datetime(Utc::now());
    /// ```
    pub fn from_datetime<T: TimeZone>(datetime: DateTime<T>) -> Ulid {
        Ulid::from_datetime_with_source(datetime, &mut rand::thread_rng())
    }

    /// Creates a new Ulid with the given datetime and random number generator
    ///
    /// # Example
    /// ```rust
    /// use stdx::chrono::offset::Utc;
    /// use rand::FromEntropy;
    /// use rand::rngs::SmallRng;
    /// use stdx::ulid::Ulid;
    ///
    /// let mut rng = SmallRng::from_entropy();
    /// let ulid = Ulid::from_datetime_with_source(Utc::now(), &mut rng);
    /// ```
    pub fn from_datetime_with_source<T, R>(datetime: DateTime<T>, source: &mut R) -> Ulid
    where
        T: TimeZone,
        R: rand::Rng,
    {
        let timestamp = datetime.timestamp_millis();
        let timebits = (timestamp & bitmask!(Self::TIME_BITS)) as u64;

        let msb = timebits << 16 | u64::from(source.gen::<u16>());
        let lsb = source.gen::<u64>();
        Ulid::from((msb, lsb))
    }

    /// Creates a Ulid from a Crockford Base32 encoded string
    ///
    /// An DecodeError will be returned when the given string is not formated
    /// properly.
    ///
    /// # Example
    /// ```rust
    /// use stdx::ulid::Ulid;
    ///
    /// let text = "01D39ZY06FGSCTVN4T2V9PKHFZ";
    /// let result = Ulid::from_string(text);
    ///
    /// assert!(result.is_ok());
    /// assert_eq!(&result.unwrap().to_string(), text);
    /// ```
    pub fn from_string(encoded: &str) -> Result<Ulid, DecodeError> {
        base32::decode(encoded).map(Ulid)
    }

    /// The 'nil Ulid'.
    ///
    /// The nil Ulid is special form of Ulid that is specified to have
    /// all 128 bits set to zero.
    ///
    /// # Example
    /// ```rust
    /// use stdx::ulid::Ulid;
    ///
    /// let ulid = Ulid::nil();
    ///
    /// assert_eq!(
    ///     ulid.to_string(),
    ///     "00000000000000000000000000"
    /// );
    /// ```
    pub fn nil() -> Ulid {
        Ulid(0)
    }

    /// Gets the datetime of when this Ulid was created accurate to 1ms
    ///
    /// # Example
    /// ```rust
    /// use stdx::chrono::Duration;
    /// use stdx::chrono::offset::Utc;
    /// use stdx::ulid::Ulid;
    ///
    /// let dt = Utc::now();
    /// let ulid = Ulid::from_datetime(dt);
    ///
    /// assert!((dt - ulid.datetime()) < Duration::milliseconds(1));
    /// ```
    pub fn datetime(&self) -> DateTime<Utc> {
        let stamp = self.timestamp_ms();
        let secs = stamp / 1000;
        let millis = stamp % 1000;
        Utc.timestamp(secs as i64, (millis * 1_000_000) as u32)
    }

    /// Gets the timestamp section of this ulid
    ///
    /// # Example
    /// ```rust
    /// use stdx::chrono::offset::Utc;
    /// use stdx::ulid::Ulid;
    ///
    /// let dt = Utc::now();
    /// let ulid = Ulid::from_datetime(dt);
    ///
    /// assert_eq!(ulid.timestamp_ms(), dt.timestamp_millis() as u64);
    /// ```
    pub fn timestamp_ms(&self) -> u64 {
        (self.0 >> Self::RAND_BITS) as u64
    }

    /// Creates a Crockford Base32 encoded string that represents this Ulid
    ///
    /// # Example
    /// ```rust
    /// use stdx::ulid::Ulid;
    ///
    /// let text = "01D39ZY06FGSCTVN4T2V9PKHFZ";
    /// let ulid = Ulid::from_string(text).unwrap();
    ///
    /// let mut buf = [0; ulid::ULID_LEN];
    /// let new_text = ulid.to_str(&mut buf).unwrap();
    ///
    /// assert_eq!(new_text, text);
    /// ```
    pub fn to_str<'buf>(&self, buf: &'buf mut [u8]) -> Result<&'buf mut str, EncodeError> {
        let len = base32::encode_to(self.0, buf)?;
        Ok(unsafe { std::str::from_utf8_unchecked_mut(&mut buf[..len]) })
    }

    /// Creates a Crockford Base32 encoded string that represents this Ulid
    ///
    /// # Example
    /// ```rust
    /// use stdx::ulid::Ulid;
    ///
    /// let text = "01D39ZY06FGSCTVN4T2V9PKHFZ";
    /// let ulid = Ulid::from_string(text).unwrap();
    ///
    /// assert_eq!(&ulid.to_string(), text);
    /// ```
    pub fn to_string(&self) -> String {
        base32::encode(self.0)
    }

    /// Test if the Ulid is nil
    ///
    /// # Example
    /// ```rust
    /// use stdx::ulid::Ulid;
    ///
    /// let ulid = Ulid::new();
    /// assert!(!ulid.is_nil());
    ///
    /// let nil = Ulid::nil();
    /// assert!(nil.is_nil());
    /// ```
    pub fn is_nil(&self) -> bool {
        self.0 == 0u128
    }

    /// Increment the random number, make sure that the ts millis stays the same
    fn increment(&self) -> Option<Ulid> {
        const MAX_RANDOM: u128 = bitmask!(Ulid::RAND_BITS);

        if (self.0 & MAX_RANDOM) == MAX_RANDOM {
            None
        } else {
            Some(Ulid(self.0 + 1))
        }
    }
}

impl Default for Ulid {
    fn default() -> Self {
        Ulid::nil()
    }
}

impl From<Ulid> for String {
    fn from(ulid: Ulid) -> String {
        ulid.to_string()
    }
}

impl From<(u64, u64)> for Ulid {
    fn from((msb, lsb): (u64, u64)) -> Self {
        Ulid(u128::from(msb) << 64 | u128::from(lsb))
    }
}

impl From<Ulid> for (u64, u64) {
    fn from(ulid: Ulid) -> (u64, u64) {
        ((ulid.0 >> 64) as u64, (ulid.0 & bitmask!(64)) as u64)
    }
}

impl From<u128> for Ulid {
    fn from(value: u128) -> Ulid {
        Ulid(value)
    }
}

impl From<Ulid> for u128 {
    fn from(ulid: Ulid) -> u128 {
        ulid.0
    }
}

impl FromStr for Ulid {
    type Err = DecodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ulid::from_string(s)
    }
}

impl fmt::Display for Ulid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut buffer = [0; ULID_LEN];
        write!(f, "{}", self.to_str(&mut buffer).unwrap())
    }
}

/// Error while trying to generate a monotonic increment in the same millisecond
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum MonotonicError {
    /// Would overflow into the next millisecond
    Overflow,
}

impl std::error::Error for MonotonicError {}

impl fmt::Display for MonotonicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let text = match *self {
            MonotonicError::Overflow => "Ulid random bits would overflow",
        };
        write!(f, "{}", text)
    }
}

/// A Ulid generator that provides monotonically increasing Ulids
pub struct Generator {
    previous: Ulid,
}

impl Generator {
    /// Create a new ulid generator for monotonically ordered ulids
    ///
    /// # Example
    /// ```rust
    /// use stdx::ulid::Generator;
    ///
    /// let mut gen = Generator::new();
    ///
    /// let ulid1 = gen.generate().unwrap();
    /// let ulid2 = gen.generate().unwrap();
    ///
    /// assert!(ulid1 < ulid2);
    /// ```
    pub fn new() -> Generator {
        Generator {
            previous: Ulid::nil(),
        }
    }

    /// Generate a new Ulid. Each call is guaranteed to provide a Ulid with a larger value than the
    /// last call. If the random bits would overflow, this method will panic.
    ///
    /// ```rust
    /// use stdx::ulid::Generator;
    /// let mut gen = Generator::new();
    ///
    /// let ulid1 = gen.generate();
    /// let ulid2 = gen.generate();
    ///
    /// assert!(ulid1 < ulid2);
    /// ```
    pub fn generate(&mut self) -> Ulid {
        let now = Utc::now();
        self.generate_from_datetime(now).expect("ulid: monotonic overflow")
    }

    /// Generate a new Ulid. Each call is guaranteed to provide a Ulid with a larger value than the
    /// last call. If the random bits would overflow, this method will return an error.
    ///
    /// ```rust
    /// use stdx::ulid::Generator;
    /// let mut gen = Generator::new();
    ///
    /// let ulid1 = gen.generate_check().unwrap();
    /// let ulid2 = gen.generate_check().unwrap();
    ///
    /// assert!(ulid1 < ulid2);
    /// ```
    pub fn generate_check(&mut self) -> Result<Ulid, MonotonicError> {
        let now = Utc::now();
        self.generate_from_datetime(now)
    }

    /// Generate a new Ulid matching the given DateTime.
    /// Each call is guaranteed to provide a Ulid with a larger value than the last call.
    /// If the random bits would overflow, this method will return an error.
    ///
    /// # Example
    /// ```rust
    /// use stdx::ulid::Generator;
    /// use stdx::chrono::Utc;
    ///
    /// let dt = Utc::now();
    /// let mut gen = Generator::new();
    ///
    /// let ulid1 = gen.generate_from_datetime(dt).unwrap();
    /// let ulid2 = gen.generate_from_datetime(dt).unwrap();
    ///
    /// assert_eq!(ulid1.datetime(), ulid2.datetime());
    /// assert!(ulid1 < ulid2);
    /// ```
    pub fn generate_from_datetime<T: TimeZone>(&mut self, datetime: DateTime<T>) -> Result<Ulid, MonotonicError> {
        self.generate_from_datetime_with_source(datetime, &mut rand::thread_rng())
    }

    /// Generate a new monotonic increasing Ulid with the given source
    /// Each call is guaranteed to provide a Ulid with a larger value than the last call.
    /// If the random bits would overflow, this method will return an error.
    ///
    /// # Example
    /// ```rust
    /// use stdx::ulid::Generator;
    /// use stdx::ulid::Ulid;
    /// use stdx::chrono::Utc;
    /// use rand::FromEntropy;
    /// use rand::rngs::SmallRng;
    ///
    /// let mut rng = SmallRng::from_entropy();
    /// let mut gen = Generator::new();
    ///
    /// let ulid1 = gen.generate_with_source(&mut rng).unwrap();
    /// let ulid2 = gen.generate_with_source(&mut rng).unwrap();
    ///
    /// assert!(ulid1 < ulid2);
    /// ```
    pub fn generate_with_source<R>(&mut self, source: &mut R) -> Result<Ulid, MonotonicError>
    where
        R: rand::Rng,
    {
        let now = Utc::now();
        self.generate_from_datetime_with_source(now, source)
    }

    /// Generate a new monotonic increasing Ulid with the given source matching the given DateTime
    /// Each call is guaranteed to provide a Ulid with a larger value than the last call.
    /// If the random bits would overflow, this method will return an error.
    ///
    /// # Example
    /// ```rust
    /// use stdx::ulid::Generator;
    /// use stdx::chrono::Utc;
    /// use rand::FromEntropy;
    /// use rand::rngs::SmallRng;
    ///
    /// let dt = Utc::now();
    /// let mut rng = SmallRng::from_entropy();
    /// let mut gen = Generator::new();
    ///
    /// let ulid1 = gen.generate_from_datetime_with_source(dt, &mut rng).unwrap();
    /// let ulid2 = gen.generate_from_datetime_with_source(dt, &mut rng).unwrap();
    ///
    /// assert_eq!(ulid1.datetime(), ulid2.datetime());
    /// assert!(ulid1 < ulid2);
    /// ```
    pub fn generate_from_datetime_with_source<T, R>(
        &mut self,
        datetime: DateTime<T>,
        source: &mut R,
    ) -> Result<Ulid, MonotonicError>
    where
        T: TimeZone,
        R: rand::Rng,
    {
        let last_ms = self.previous.timestamp_ms() as i64;
        // maybe time went backward, or it is the same ms.
        // increment instead of generating a new random so that it is monotonic
        if datetime.timestamp_millis() <= last_ms {
            if let Some(next) = self.previous.increment() {
                self.previous = next;
                return Ok(next);
            } else {
                return Err(MonotonicError::Overflow);
            }
        }
        let next = Ulid::from_datetime_with_source(datetime, source);
        self.previous = next;
        Ok(next)
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_dynamic() {
        let ulid = Ulid::new();
        let encoded = ulid.to_string();
        let ulid2 = Ulid::from_string(&encoded).expect("failed to deserialize");

        println!("{}", encoded);
        println!("{:?}", ulid);
        println!("{:?}", ulid2);
        assert_eq!(ulid, ulid2);
    }

    #[test]
    fn test_static() {
        let s = Ulid(0x41414141414141414141414141414141).to_string();
        let u = Ulid::from_string(&s).unwrap();
        assert_eq!(&s, "21850M2GA1850M2GA1850M2GA1");
        assert_eq!(u.0, 0x41414141414141414141414141414141);
    }

    #[test]
    fn test_source() {
        use rand::rngs::mock::StepRng;
        let mut source = StepRng::new(123, 0);

        let u1 = Ulid::with_source(&mut source);
        let dt = Utc::now() + Duration::milliseconds(1);
        let u2 = Ulid::from_datetime_with_source(dt, &mut source);
        let u3 = Ulid::from_datetime_with_source(dt, &mut source);

        assert!(u1 < u2);
        assert_eq!(u2, u3);
    }

    #[test]
    fn test_order() {
        let dt = Utc::now();
        let ulid1 = Ulid::from_datetime(dt);
        let ulid2 = Ulid::from_datetime(dt + Duration::milliseconds(1));
        assert!(ulid1 < ulid2);
    }

    #[test]
    fn test_order_monotonic() {
        let dt = Utc::now();
        let mut gen = Generator::new();
        let ulid1 = gen.generate_from_datetime(dt).unwrap();
        let ulid2 = gen.generate_from_datetime(dt).unwrap();
        let ulid3 = Ulid::from_datetime(dt + Duration::milliseconds(1));
        assert_eq!(ulid1.0 + 1, ulid2.0);
        assert!(ulid2 < ulid3);
        assert!(ulid2.timestamp_ms() < ulid3.timestamp_ms())
    }

    #[test]
    fn test_order_monotonic_with_source() {
        use rand::rngs::mock::StepRng;
        let mut source = StepRng::new(123, 0);
        let mut gen = Generator::new();

        let _has_default = Generator::default();

        let ulid1 = gen.generate_with_source(&mut source).unwrap();
        let ulid2 = gen.generate_with_source(&mut source).unwrap();
        assert!(ulid1 < ulid2);
    }

    #[test]
    fn test_increment() {
        let ulid = Ulid::from_string("01BX5ZZKBKAZZZZZZZZZZZZZZZ").unwrap();
        let ulid = ulid.increment().unwrap();
        assert_eq!("01BX5ZZKBKB000000000000000", ulid.to_string());

        let ulid = Ulid::from_string("01BX5ZZKBKZZZZZZZZZZZZZZZX").unwrap();
        let ulid = ulid.increment().unwrap();
        assert_eq!("01BX5ZZKBKZZZZZZZZZZZZZZZY", ulid.to_string());
        let ulid = ulid.increment().unwrap();
        assert_eq!("01BX5ZZKBKZZZZZZZZZZZZZZZZ", ulid.to_string());
        assert!(ulid.increment().is_none());
    }

    #[test]
    fn test_increment_overflow() {
        let ulid = Ulid(u128::max_value());
        assert!(ulid.increment().is_none());
    }

    #[test]
    fn test_datetime() {
        let dt = Utc::now();
        let ulid = Ulid::from_datetime(dt);

        println!("{:?}, {:?}", dt, ulid.datetime());
        assert!(ulid.datetime() <= dt);
        assert!(ulid.datetime() + Duration::milliseconds(1) >= dt);
    }

    #[test]
    fn test_timestamp() {
        let dt = Utc::now();
        let ulid = Ulid::from_datetime(dt);
        let ts = dt.timestamp() as u64 * 1000 + dt.timestamp_subsec_millis() as u64;

        assert_eq!(ulid.timestamp_ms(), ts);
    }

    #[test]
    fn can_into_thing() {
        let ulid = Ulid::new();
        let s: String = ulid.into();
        let u: u128 = ulid.into();
        let uu: (u64, u64) = ulid.into();

        assert_eq!(Ulid::from_str(&s).unwrap(), ulid);
        assert_eq!(Ulid::from(u), ulid);
        assert_eq!(Ulid::from(uu), ulid);
    }

    #[test]
    fn default_is_nil() {
        assert_eq!(Ulid::default(), Ulid::nil());
    }

    #[test]
    fn can_display_things() {
        println!("{}", Ulid::new());
        println!("{}", EncodeError::BufferTooSmall);
        println!("{}", DecodeError::InvalidLength);
        println!("{}", DecodeError::InvalidChar);
        println!("{}", MonotonicError::Overflow);
    }
}
