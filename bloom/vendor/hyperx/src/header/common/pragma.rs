use std::fmt;

use header::{Header, RawLike, parsing};

/// The `Pragma` header defined by HTTP/1.0.
///
/// > The "Pragma" header field allows backwards compatibility with
/// > HTTP/1.0 caches, so that clients can specify a "no-cache" request
/// > that they will understand (as Cache-Control was not defined until
/// > HTTP/1.1).  When the Cache-Control header field is also present and
/// > understood in a request, Pragma is ignored.
/// > In HTTP/1.0, Pragma was defined as an extensible field for
/// > implementation-specified directives for recipients.  This
/// > specification deprecates such extensions to improve interoperability.
///
/// Spec: [https://tools.ietf.org/html/rfc7234#section-5.4][url]
///
/// [url]: https://tools.ietf.org/html/rfc7234#section-5.4
///
/// # Examples
///
/// ```
/// # extern crate http;
/// use hyperx::header::{Pragma, TypedHeaders};
///
/// let mut headers = http::HeaderMap::new();
/// headers.encode(&Pragma::NoCache);
/// ```
///
/// ```
/// # extern crate http;
/// use hyperx::header::{Pragma, TypedHeaders};
///
/// let mut headers = http::HeaderMap::new();
/// headers.encode(&Pragma::Ext("foobar".to_owned()));
/// ```
#[derive(Clone, PartialEq, Debug)]
pub enum Pragma {
    /// Corresponds to the `no-cache` value.
    NoCache,
    /// Every value other than `no-cache`.
    Ext(String),
}

impl Header for Pragma {
    fn header_name() -> &'static str {
        static NAME: &'static str = "Pragma";
        NAME
    }

    fn parse_header<'a, T>(raw: &'a T) -> ::Result<Pragma>
    where T: RawLike<'a>
    {
        parsing::from_one_raw_str(raw).and_then(|s: String| {
            let slice = &s.to_ascii_lowercase()[..];
            match slice {
                "no-cache" => Ok(Pragma::NoCache),
                _ => Ok(Pragma::Ext(s)),
            }
        })
    }

    fn fmt_header(&self, f: &mut ::header::Formatter) -> fmt::Result {
        f.fmt_line(self)
    }
}

impl fmt::Display for Pragma {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Pragma::NoCache => "no-cache",
            Pragma::Ext(ref string) => &string[..],
        })
    }
}

#[test]
fn test_parse_header() {
    use header::{Header, Raw};

    let r: Raw = "no-cache".into();
    let a: Pragma = Header::parse_header(&r).unwrap();
    let b = Pragma::NoCache;
    assert_eq!(a, b);

    let r: Raw = "FoObar".into();
    let c: Pragma = Header::parse_header(&r).unwrap();
    let d = Pragma::Ext("FoObar".to_owned());
    assert_eq!(c, d);

    let r: Raw = "".into();
    let e: ::Result<Pragma> = Header::parse_header(&r);
    assert_eq!(e.ok(), None);
}

standard_header!(Pragma, PRAGMA);
