use std::any::Any;
use std::fmt;
use std::str::{FromStr, from_utf8};
use std::ops::{Deref, DerefMut};
use header::{Header, RawLike, Scheme};

/// `Proxy-Authorization` header, defined in [RFC7235](https://tools.ietf.org/html/rfc7235#section-4.4)
///
/// The `Proxy-Authorization` header field allows a user agent to authenticate
/// itself with an HTTP proxy -- usually, but not necessarily, after
/// receiving a 407 (Proxy Authentication Required) response and the
/// `Proxy-Authenticate` header. Its value consists of credentials containing
/// the authentication information of the user agent for the realm of the
/// resource being requested.
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
/// use hyperx::header::{ProxyAuthorization, TypedHeaders};
///
/// let mut headers = http::HeaderMap::new();
/// headers.encode(&ProxyAuthorization("let me in".to_owned()));
/// ```
/// ```
/// # extern crate http;
/// use hyperx::header::{ProxyAuthorization, Basic, TypedHeaders};
///
/// let mut headers = http::HeaderMap::new();
/// headers.encode(
///    &ProxyAuthorization(
///        Basic {
///            username: "Aladdin".to_owned(),
///            password: Some("open sesame".to_owned())
///        }
///    )
/// );
/// ```
///
/// ```
/// # extern crate http;
/// use hyperx::header::{ProxyAuthorization, Bearer, TypedHeaders};
///
/// let mut headers = http::HeaderMap::new();
/// headers.encode(
///    &ProxyAuthorization(
///        Bearer {
///            token: "QWxhZGRpbjpvcGVuIHNlc2FtZQ".to_owned()
///        }
///    )
/// );
/// ```
#[derive(Clone, PartialEq, Debug)]
pub struct ProxyAuthorization<S: Scheme>(pub S);

impl<S: Scheme> Deref for ProxyAuthorization<S> {
    type Target = S;

    fn deref(&self) -> &S {
        &self.0
    }
}

impl<S: Scheme> DerefMut for ProxyAuthorization<S> {
    fn deref_mut(&mut self) -> &mut S {
        &mut self.0
    }
}

impl<S: Scheme + Any> Header for ProxyAuthorization<S> where <S as FromStr>::Err: 'static {
    fn header_name() -> &'static str {
        static NAME: &'static str = "Proxy-Authorization";
        NAME
    }

    fn parse_header<'a, T>(raw: &'a T) -> ::Result<ProxyAuthorization<S>>
    where T: RawLike<'a>
    {
        if let Some(line) = raw.one() {
            let header = from_utf8(line)?;
            if let Some(scheme) = <S as Scheme>::scheme() {
                if header.starts_with(scheme) && header.len() > scheme.len() + 1 {
                    match header[scheme.len() + 1..].parse::<S>().map(ProxyAuthorization) {
                        Ok(h) => Ok(h),
                        Err(_) => Err(::Error::Header)
                    }
                } else {
                    Err(::Error::Header)
                }
            } else {
                match header.parse::<S>().map(ProxyAuthorization) {
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

impl<S: Scheme> fmt::Display for ProxyAuthorization<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(scheme) = <S as Scheme>::scheme() {
            write!(f, "{} ", scheme)?
        };
        self.0.fmt_scheme(f)
    }
}

#[cfg(test)]
mod tests {
    use super::ProxyAuthorization;
    use super::super::super::{Header, Raw, Basic, Bearer};

    #[cfg(feature = "headers")]
    use super::super::super::Headers;

    #[cfg(feature = "headers")]
    #[test]
    fn test_raw_auth() {
        let mut headers = Headers::new();
        headers.set(ProxyAuthorization("foo bar baz".to_owned()));
        assert_eq!(headers.to_string(), "Proxy-Authorization: foo bar baz\r\n".to_owned());
    }

    #[test]
    fn test_raw_auth_parse() {
        let r: Raw = b"foo bar baz".as_ref().into();
        let header: ProxyAuthorization<String> = Header::parse_header(&r).unwrap();
        assert_eq!(header.0, "foo bar baz");
    }

    #[cfg(feature = "headers")]
    #[test]
    fn test_basic_auth() {
        let mut headers = Headers::new();
        headers.set(ProxyAuthorization(
            Basic { username: "Aladdin".to_owned(), password: Some("open sesame".to_owned()) }));
        assert_eq!(
            headers.to_string(),
            "Proxy-Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==\r\n".to_owned());
    }

    #[cfg(feature = "headers")]
    #[test]
    fn test_basic_auth_no_password() {
        let mut headers = Headers::new();
        headers.set(ProxyAuthorization(Basic { username: "Aladdin".to_owned(), password: None }));
        assert_eq!(headers.to_string(), "Proxy-Authorization: Basic QWxhZGRpbjo=\r\n".to_owned());
    }

    #[test]
    fn test_basic_auth_parse() {
        let r: Raw = b"Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==".as_ref().into();
        let auth: ProxyAuthorization<Basic> = Header::parse_header(&r).unwrap();
        assert_eq!(auth.0.username, "Aladdin");
        assert_eq!(auth.0.password, Some("open sesame".to_owned()));
    }

    #[test]
    fn test_basic_auth_parse_no_password() {
        let r: Raw = b"Basic QWxhZGRpbjo=".as_ref().into();
        let auth: ProxyAuthorization<Basic> = Header::parse_header(&r).unwrap();
        assert_eq!(auth.0.username, "Aladdin");
        assert_eq!(auth.0.password, Some("".to_owned()));
    }

    #[cfg(feature = "headers")]
    #[test]
    fn test_bearer_auth() {
        let mut headers = Headers::new();
        headers.set(ProxyAuthorization(
            Bearer { token: "fpKL54jvWmEGVoRdCNjG".to_owned() }));
        assert_eq!(
            headers.to_string(),
            "Proxy-Authorization: Bearer fpKL54jvWmEGVoRdCNjG\r\n".to_owned());
    }

    #[test]
    fn test_bearer_auth_parse() {
        let r: Raw = b"Bearer fpKL54jvWmEGVoRdCNjG".as_ref().into();
        let auth: ProxyAuthorization<Bearer> = Header::parse_header(&r).unwrap();
        assert_eq!(auth.0.token, "fpKL54jvWmEGVoRdCNjG");
    }
}

#[cfg(test)]
#[cfg(feature = "nightly")]
mod benches {
    use super::ProxyAuthorization;
    use ::header::{Basic, Bearer};

    bench_header!(raw, ProxyAuthorization<String>, { vec![b"foo bar baz".to_vec()] });
    bench_header!(basic, ProxyAuthorization<Basic>, { vec![b"Basic QWxhZGRpbjpuIHNlc2FtZQ==".to_vec()] });
    bench_header!(bearer, ProxyAuthorization<Bearer>, { vec![b"Bearer fpKL54jvWmEGVoRdCNjG".to_vec()] });
}

impl<S> ::header::StandardHeader for ProxyAuthorization<S>
    where S: Scheme + Any
{
    #[inline]
    fn http_header_name() -> ::http::header::HeaderName {
        ::http::header::PROXY_AUTHORIZATION
    }
}
