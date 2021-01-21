use std::fmt::{self, Display};
use std::str::FromStr;
use std::time::SystemTime;

use httpdate::HttpDate as InnerDate;

/// A timestamp with HTTP formatting and parsing
//   Prior to 1995, there were three different formats commonly used by
//   servers to communicate timestamps.  For compatibility with old
//   implementations, all three are defined here.  The preferred format is
//   a fixed-length and single-zone subset of the date and time
//   specification used by the Internet Message Format [RFC5322].
//
//     HTTP-date    = IMF-fixdate / obs-date
//
//   An example of the preferred format is
//
//     Sun, 06 Nov 1994 08:49:37 GMT    ; IMF-fixdate
//
//   Examples of the two obsolete formats are
//
//     Sunday, 06-Nov-94 08:49:37 GMT   ; obsolete RFC 850 format
//     Sun Nov  6 08:49:37 1994         ; ANSI C's asctime() format
//
//   A recipient that parses a timestamp value in an HTTP header field
//   MUST accept all three HTTP-date formats.  When a sender generates a
//   header field that contains one or more timestamps defined as
//   HTTP-date, the sender MUST generate those timestamps in the
//   IMF-fixdate format.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HttpDate(InnerDate);

impl FromStr for HttpDate {
    type Err = ::Error;
    fn from_str(s: &str) -> ::Result<HttpDate> {
        InnerDate::from_str(s)
            .map(HttpDate)
            .map_err(|_| ::Error::Header)
    }
}

impl Display for HttpDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl From<SystemTime> for HttpDate {
    fn from(sys: SystemTime) -> HttpDate {
        HttpDate(sys.into())
    }
}

impl From<HttpDate> for SystemTime {
    fn from(date: HttpDate) -> SystemTime {
        date.0.into()
    }
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, Duration};

    use super::HttpDate;

    macro_rules! test_parse {
        ($function: ident, $date: expr) => {
            #[test]
            fn $function() {
                let nov_07 = HttpDate((
                    SystemTime::UNIX_EPOCH + Duration::new(784198117, 0)
                ).into());

                assert_eq!($date.parse::<HttpDate>().unwrap(), nov_07);
            }
        };
    }

    test_parse!(test_imf_fixdate, "Sun, 07 Nov 1994 08:48:37 GMT");
    test_parse!(test_rfc_850, "Sunday, 07-Nov-94 08:48:37 GMT");
    test_parse!(test_asctime, "Sun Nov  7 08:48:37 1994");

    #[test]
    fn test_no_date() {
        assert!("this-is-no-date".parse::<HttpDate>().is_err());
    }
}
