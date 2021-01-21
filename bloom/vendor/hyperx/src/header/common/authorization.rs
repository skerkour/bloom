use std::any::Any;
use std::fmt::{self, Display};
use std::str::{FromStr, from_utf8};
use std::ops::{Deref, DerefMut};
use base64::{encode, decode};
use header::{Header, RawLike};

/// `Authorization` header, defined in [RFC7235](https://tools.ietf.org/html/rfc7235#section-4.2)
///
/// The `Authorization` header field allows a user agent to authenticate
/// itself with an origin server -- usually, but not necessarily, after
/// receiving a 401 (Unauthorized) response.  Its value consists of
/// credentials containing the authentication information of the user
/// agent for the realm of the resource being requested.
///
/// # ABNF
///
/// ```text
/// Authorization = credentials
/// ```
///
/// # Example values
/// * `Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==`
/// * `Bearer fpKL54jvWmEGVoRdCNjG`
///
/// # Examples
///
/// ```
/// # extern crate http;
/// use hyperx::header::{Authorization, TypedHeaders};
///
/// let mut headers = http::HeaderMap::new();
/// headers.encode(&Authorization("let me in".to_owned()));
/// ```
/// ```
/// # extern crate http;
/// use hyperx::header::{Authorization, Basic, TypedHeaders};
///
/// let mut headers = http::HeaderMap::new();
/// headers.encode(
///    &Authorization(
///        Basic {
///            username: "Aladdin".to_owned(),
///            password: Some("open sesame".to_owned())
///        }
///    )
/// );
/// ```
///
/// ```
/// use hyperx::header::{Authorization, Bearer, TypedHeaders};
///
/// let mut headers = http::HeaderMap::new();
/// headers.encode(
///    &Authorization(
///        Bearer {
///            token: "QWxhZGRpbjpvcGVuIHNlc2FtZQ".to_owned()
///        }
///    )
/// );
/// ```
#[derive(Clone, PartialEq, Debug)]
pub struct Authorization<S: Scheme>(pub S);

impl<S: Scheme> Deref for Authorization<S> {
    type Target = S;

    fn deref(&self) -> &S {
        &self.0
    }
}

impl<S: Scheme> DerefMut for Authorization<S> {
    fn deref_mut(&mut self) -> &mut S {
        &mut self.0
    }
}

impl<S: Scheme + Any> Header for Authorization<S> where <S as FromStr>::Err: 'static {
    fn header_name() -> &'static str {
        static NAME: &'static str = "Authorization";
        NAME
    }

    fn parse_header<'a, T>(raw: &'a T) -> ::Result<Authorization<S>>
    where T: RawLike<'a>
    {
        if let Some(line) = raw.one() {
            let header = from_utf8(line)?;
            if let Some(scheme) = <S as Scheme>::scheme() {
                if header.starts_with(scheme) && header.len() > scheme.len() + 1 {
                    match header[scheme.len() + 1..].parse::<S>().map(Authorization) {
                        Ok(h) => Ok(h),
                        Err(_) => Err(::Error::Header)
                    }
                } else {
                    Err(::Error::Header)
                }
            } else {
                match header.parse::<S>().map(Authorization) {
                    Ok(h) => Ok(h),
                    Err(_) => Err(::Error::Header)
                }
            }
        } else {
            Err(::Error::Header)
        }
    }

    fn fmt_header(&self, f: &mut ::header::Formatter) -> fmt::Result {
        f.fmt_line(self)
    }
}

impl<S: Scheme> fmt::Display for Authorization<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(scheme) = <S as Scheme>::scheme() {
            write!(f, "{} ", scheme)?;
        };
        self.0.fmt_scheme(f)
    }
}

/// An Authorization scheme to be used in the header.
pub trait Scheme: FromStr + fmt::Debug + Clone + Send + Sync {
    /// An optional Scheme name.
    ///
    /// Will be replaced with an associated constant once available.
    fn scheme() -> Option<&'static str>;
    /// Format the Scheme data into a header value.
    fn fmt_scheme(&self, &mut fmt::Formatter) -> fmt::Result;
}

impl Scheme for String {
    fn scheme() -> Option<&'static str> {
        None
    }

    fn fmt_scheme(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

/// Credential holder for Basic Authentication
#[derive(Clone, PartialEq, Debug)]
pub struct Basic {
    /// The username as a possibly empty string
    pub username: String,
    /// The password. `None` if the `:` delimiter character was not
    /// part of the parsed input. Note: A compliant client MUST
    /// always send a password (which may be the empty string).
    pub password: Option<String>
}

impl Scheme for Basic {
    fn scheme() -> Option<&'static str> {
        Some("Basic")
    }

    fn fmt_scheme(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //FIXME: serialize::base64 could use some Debug implementation, so
        //that we don't have to allocate a new string here just to write it
        //to the formatter.
        let mut text = self.username.clone();
        text.push(':');
        if let Some(ref pass) = self.password {
            text.push_str(&pass[..]);
        }

        f.write_str(&encode(&text))
    }
}

/// creates a Basic from a base-64 encoded, `:`-delimited utf-8 string
impl FromStr for Basic {
    type Err = ::Error;
    fn from_str(s: &str) -> ::Result<Basic> {
        match decode(s) {
            Ok(decoded) => match String::from_utf8(decoded) {
                Ok(text) => {
                    let parts = &mut text.split(':');
                    let user = match parts.next() {
                        Some(part) => part.to_owned(),
                        None => return Err(::Error::Header)
                    };
                    let password = match parts.next() {
                        Some(part) => Some(part.to_owned()),
                        None => None
                    };
                    Ok(Basic {
                        username: user,
                        password: password
                    })
                },
                Err(_) => {
                    Err(::Error::Header)
                }
            },
            Err(_) => {
                Err(::Error::Header)
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
///Token holder for Bearer Authentication, most often seen with oauth
pub struct Bearer {
    ///Actual bearer token as a string
    pub token: String
}

impl Scheme for Bearer {
    fn scheme() -> Option<&'static str> {
        Some("Bearer")
    }

    fn fmt_scheme(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.token)
    }
}

impl FromStr for Bearer {
    type Err = ::Error;
    fn from_str(s: &str) -> ::Result<Bearer> {
        Ok(Bearer { token: s.to_owned()})
    }
}

#[cfg(test)]
mod tests {
    use super::{Authorization, Basic, Bearer};
    use super::super::super::{Header, Raw};

    #[cfg(feature = "headers")]
    use super::super::super::Headers;

    #[cfg(feature = "headers")]
    #[test]
    fn test_raw_auth() {
        let mut headers = Headers::new();
        headers.set(Authorization("foo bar baz".to_owned()));
        assert_eq!(headers.to_string(), "Authorization: foo bar baz\r\n".to_owned());
    }

    #[test]
    fn test_raw_auth_parse() {
        let r: Raw = b"foo bar baz".as_ref().into();
        let header: Authorization<String> = Header::parse_header(&r).unwrap();
        assert_eq!(header.0, "foo bar baz");
    }

    #[cfg(feature = "headers")]
    #[test]
    fn test_basic_auth() {
        let mut headers = Headers::new();
        headers.set(Authorization(
            Basic { username: "Aladdin".to_owned(), password: Some("open sesame".to_owned()) }));
        assert_eq!(
            headers.to_string(),
            "Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==\r\n".to_owned());
    }

    #[cfg(feature = "headers")]
    #[test]
    fn test_basic_auth_no_password() {
        let mut headers = Headers::new();
        headers.set(Authorization(Basic { username: "Aladdin".to_owned(), password: None }));
        assert_eq!(headers.to_string(), "Authorization: Basic QWxhZGRpbjo=\r\n".to_owned());
    }

    #[test]
    fn test_basic_auth_parse() {
        let r: Raw = b"Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==".as_ref().into();
        let auth: Authorization<Basic> = Header::parse_header(&r).unwrap();
        assert_eq!(auth.0.username, "Aladdin");
        assert_eq!(auth.0.password, Some("open sesame".to_owned()));
    }

    #[test]
    fn test_basic_auth_parse_no_password() {
        let r: Raw = b"Basic QWxhZGRpbjo=".as_ref().into();
        let auth: Authorization<Basic> = Header::parse_header(&r).unwrap();
        assert_eq!(auth.0.username, "Aladdin");
        assert_eq!(auth.0.password, Some("".to_owned()));
    }

    #[cfg(feature = "headers")]
    #[test]
    fn test_bearer_auth() {
        let mut headers = Headers::new();
        headers.set(Authorization(
            Bearer { token: "fpKL54jvWmEGVoRdCNjG".to_owned() }));
        assert_eq!(
            headers.to_string(),
            "Authorization: Bearer fpKL54jvWmEGVoRdCNjG\r\n".to_owned());
    }

    #[test]
    fn test_bearer_auth_parse() {
        let r: Raw = b"Bearer fpKL54jvWmEGVoRdCNjG".as_ref().into();
        let auth: Authorization<Bearer> = Header::parse_header(&r).unwrap();
        assert_eq!(auth.0.token, "fpKL54jvWmEGVoRdCNjG");
    }
}

bench_header!(raw, Authorization<String>, { vec![b"foo bar baz".to_vec()] });
bench_header!(basic, Authorization<Basic>, { vec![b"Basic QWxhZGRpbjpuIHNlc2FtZQ==".to_vec()] });
bench_header!(bearer, Authorization<Bearer>, { vec![b"Bearer fpKL54jvWmEGVoRdCNjG".to_vec()] });

impl<S> ::header::StandardHeader for Authorization<S>
    where S: Scheme + Any
{
    #[inline]
    fn http_header_name() -> ::http::header::HeaderName {
        ::http::header::AUTHORIZATION
    }
}
