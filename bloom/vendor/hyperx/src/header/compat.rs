//! Implementation module for various compatibility features with the _http_
//! crate.

use std::fmt::Display;

use http::header::{GetAll, HeaderMap, HeaderValue, ValueIter};

use ::Result;
use super::{Header, RawLike};

#[cfg(feature = "headers")]
use std::convert::From;

#[cfg(feature = "headers")]
use http;

#[cfg(feature = "headers")]
use super::{Headers};

/// A trait for the "standard" headers that have an associated `HeaderName`
/// constant in the _http_ crate.
pub trait StandardHeader: Header + Sized {
    /// The `HeaderName` from the _http_ crate for this header.
    fn http_header_name() -> ::http::header::HeaderName;
}

/// Extension trait for `decode` (parsing) and `encode` (serialization) of
/// typed headers from/to a collection of headers such as `http::HeaderMap`.
pub trait TypedHeaders {
    /// Decode and return `Header` type H or `Error::Header`.
    ///
    /// `Error::Header` is returned on failed parse, or for a single-valued
    /// Header type, if no values or multiple values are found in the
    /// collection.  Multi-valued header types such as `ContentEncoding` will
    /// instead return an empty list value if no values are found.  To
    /// distinguish the not found case, use `try_decode` instead.
    fn decode<H>(&self) -> Result<H>
        where H: StandardHeader;

    /// Decode and return `Header` type H or `Error::Header` if found, or
    /// return `None` if not found.
    ///
    /// This variant will return `Option::None` if no header with the
    /// associated key (`HeaderName`) is found in the collection. If the
    /// collection does contain such a key, it will return the header type H or
    /// `Error::Header`.
    fn try_decode<H>(&self) -> Option<Result<H>>
        where H: StandardHeader;

    /// Encode and write the specified typed header value in the collection.
    ///
    /// Uses the `Display` format of the provided header value to write a single
    /// header. This will overwrite any preexisting values with the same
    /// key (`HeaderName`). Use `encode_append` instead to avoid this.
    fn encode<H>(&mut self, value: &H)
        where H: StandardHeader + Display;

    /// Encode and append the specified typed header value into the collection.
    ///
    /// Uses the `Display` format of the provided header value to append a
    /// single header. If the collection previously had a value for the same
    /// key, the additional value is appended to the end.
    fn encode_append<H>(&mut self, value: &H)
        where H: StandardHeader + Display;
}

/// Iterator adaptor for HeaderValue
#[derive(Debug)]
pub struct ValueMapIter<'a>(ValueIter<'a, HeaderValue>);

impl TypedHeaders for HeaderMap {
    fn decode<H>(&self) -> Result<H>
        where H: StandardHeader
    {
        let vals = self.get_all(H::http_header_name());
        H::parse_header(&vals)
    }

    fn try_decode<H>(&self) -> Option<Result<H>>
        where H: StandardHeader
    {
        let hname = H::http_header_name();
        if self.contains_key(&hname) {
            let vals = self.get_all(&hname);
            Some(H::parse_header(&vals))
        } else {
            None
        }
    }

    fn encode<H>(&mut self, val: &H)
        where H: StandardHeader + Display
    {
        self.insert(
            H::http_header_name(),
            val.to_string().parse().expect("header value"));
    }

    fn encode_append<H>(&mut self, val: &H)
        where H: StandardHeader + Display
    {
        self.append(
            H::http_header_name(),
            val.to_string().parse().expect("header value"));
    }
}

#[cfg(feature = "headers")]
impl From<http::HeaderMap> for Headers {
    fn from(header_map: http::HeaderMap) -> Headers {
        Headers::from(&header_map)
    }
}

#[cfg(feature = "headers")]
impl<'a> From<&'a http::HeaderMap> for Headers {
    fn from(header_map: &'a http::HeaderMap) -> Headers {
        let mut headers = Headers::new();
        for (name, value) in header_map.iter() {
            headers.append_raw_str(name.as_str(), value.as_bytes());
        }
        headers
    }
}

#[cfg(feature = "headers")]
impl From<Headers> for http::HeaderMap {
    #[inline]
    fn from(headers: Headers) -> http::HeaderMap {
        http::HeaderMap::from(&headers)
    }
}

#[cfg(feature = "headers")]
impl<'a> From<&'a Headers> for http::HeaderMap {
    fn from(headers: &'a Headers) -> http::HeaderMap {
        let mut hmap = http::HeaderMap::new();
        for header in headers.iter() {
            let name: http::header::HeaderName = header.name().parse()
                .expect("convert invalid header name");
            let entry = hmap.entry(name);
            let mut value_iter = header.raw().iter().map(|line| {
                http::header::HeaderValue::from_bytes(line)
                    .expect("convert invalid header value")
            });
            match entry {
                http::header::Entry::Occupied(mut  occupied) => {
                    for value in value_iter {
                        occupied.append(value);
                    }
                },
                http::header::Entry::Vacant(vacant) => {
                    if let Some(first_value) = value_iter.next() {
                        let mut occupied = vacant.insert_entry(first_value);
                        for value in value_iter {
                            occupied.append(value);
                        }
                    }
                }
            }
        }
        hmap
    }
}

impl<'a> Iterator for ValueMapIter<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(HeaderValue::as_bytes)
    }
}

impl<'a> RawLike<'a> for GetAll<'a, HeaderValue> {
    type IntoIter = ValueMapIter<'a>;

    fn len(&'a self) -> usize {
        self.iter().count()
    }

    fn one(&'a self) -> Option<&'a [u8]> {
        let mut iter = self.iter();
        if let Some(v) = iter.next() {
            if iter.next().is_none() {
                return Some(v.as_bytes());
            }
        }
        None
    }

    fn iter(&'a self) -> ValueMapIter<'a> {
        ValueMapIter(self.iter())
    }
}

impl<'a> RawLike<'a> for &'a HeaderValue {
    type IntoIter = ::std::iter::Once<&'a [u8]>;

    fn len(&'a self) -> usize {
        1
    }

    fn one(&'a self) -> Option<&'a [u8]> {
        Some(self.as_bytes())
    }

    fn iter(&'a self) -> Self::IntoIter {
        ::std::iter::once(self.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use http;
    use ::header::{
        ContentEncoding, ContentLength, Encoding, ETag,
        Header, Te, TypedHeaders};

    #[cfg(feature = "headers")]
    use ::header::{Headers, Host};

    #[cfg(feature = "nightly")]
    use test::Bencher;

    #[cfg(feature = "nightly")]
    use ::header::EntityTag;

    #[test]
    fn test_empty_decode() {
        let hmap = http::HeaderMap::new();
        let len = hmap.decode::<ContentLength>();
        assert!(len.is_err());
    }

    #[test]
    fn test_empty_decode_etag() {
        let hmap = http::HeaderMap::new();
        let len = hmap.decode::<ETag>();
        assert!(len.is_err());
    }

    #[test]
    fn test_empty_decode_te() {
        let hmap = http::HeaderMap::new();
        let te = hmap.decode::<Te>().unwrap();
        assert_eq!(te, Te(vec![]));
    }

    #[test]
    fn test_empty_decode_content_encoding() {
        let hmap = http::HeaderMap::new();
        let ce = hmap.decode::<ContentEncoding>().unwrap();
        assert_eq!(ce, ContentEncoding(vec![]));
    }

    #[test]
    fn test_empty_try_decode() {
        let hmap = http::HeaderMap::new();
        let len = hmap.try_decode::<ContentLength>();
        assert!(len.is_none());
    }

    #[test]
    fn test_empty_try_decode_te() {
        let hmap = http::HeaderMap::new();
        let te = hmap.try_decode::<Te>();
        assert!(te.is_none());
    }

    #[test]
    fn test_decode() {
        let mut hmap = http::HeaderMap::new();
        hmap.insert(http::header::CONTENT_LENGTH, "11".parse().unwrap());
        let len: ContentLength = hmap.decode().unwrap();
        assert_eq!(*len, 11);
    }

    #[test]
    fn test_encode_decode() {
        let mut hmap = http::HeaderMap::new();
        hmap.encode(&ContentLength(11));
        let len: ContentLength = hmap.decode().unwrap();
        assert_eq!(*len, 11);
    }

    #[test]
    fn test_empty_encode() {
        let mut hmap = http::HeaderMap::new();
        hmap.encode(&ContentEncoding(vec![]));
        assert_eq!(hmap.len(), 1);
        let ce: ContentEncoding = hmap.decode().unwrap();
        assert_eq!(*ce, vec![]);
    }

    #[test]
    fn test_empty_encode_2() {
        let mut hmap = http::HeaderMap::new();
        hmap.encode(&ContentEncoding(vec![]));
        hmap.encode_append(&ContentEncoding(vec![]));
        assert_eq!(hmap.len(), 2);
        let ce: ContentEncoding = hmap.decode().unwrap();
        assert_eq!(*ce, vec![]);
    }

    #[test]
    fn test_encode_append() {
        let mut hmap = http::HeaderMap::new();
        hmap.encode_append(
            &ContentEncoding(vec![Encoding::Identity]));
        hmap.encode_append(
            &ContentEncoding(vec![Encoding::Gzip, Encoding::Chunked]));
        let ce: ContentEncoding = hmap.decode().unwrap();
        assert_eq!(
            *ce,
            vec![Encoding::Identity, Encoding::Gzip, Encoding::Chunked]);
    }

    #[cfg(feature = "headers")]
    fn raw_headers_sample() -> Headers {
        let mut heads = Headers::new();

        heads.set_raw("date", b"Thu, 02 May 2019 21:10:03 GMT".as_ref());
        heads.set_raw("connection", b"Keep-Alive".as_ref());
        heads.set_raw("accept-ranges", b"bytes".as_ref());
        heads.set_raw("etag", b"\"1544639720\"".as_ref());
        heads.set_raw("transfer-encoding", b"gzip".as_ref());
        heads.append_raw("transfer-encoding", b"chunked".as_ref());
        heads.set_raw("content-length", b"7050".as_ref());
        heads.set_raw("content-type", b"text/css; charset=utf-8".as_ref());
        heads.set_raw("last-modified", b"Wed, 12 Dec 2018 18:35:20 GMT".as_ref());
        heads.set_raw("x-hello-human",
                      b"Say hello back! @getBootstrapCDN on Twitter".as_ref());
        heads.set_raw("access-control-allow-origin", b"*".as_ref());
        heads.set_raw("vary", b"Accept-Encoding".as_ref());
        heads.set_raw("x-cache", b"HIT".as_ref());
        heads.set_raw("timing-allow-origin", b"*".as_ref());
        heads.set_raw("cache-control", b"public, max-age=31536000".as_ref());

        heads
    }

    #[cfg(feature = "headers")]
    #[test]
    fn test_convert_mixed() {
        let mut headers = Headers::new();
        headers.set(ContentLength(11));
        headers.set(Host::new("foo.bar", None));
        headers.append_raw("x-foo", b"bar".to_vec());
        headers.append_raw("x-foo", b"quux".to_vec());

        let mut hmap = http::HeaderMap::new();
        hmap.insert(http::header::CONTENT_LENGTH, "11".parse().unwrap());
        hmap.insert(http::header::HOST, "foo.bar".parse().unwrap());
        hmap.append("x-foo", "bar".parse().unwrap());
        hmap.append("x-foo", "quux".parse().unwrap());

        let headers2: Headers = hmap.clone().into();
        let hmap2: http::HeaderMap = headers.clone().into();
        assert_eq!(headers, headers2);
        assert_eq!(headers2.len(), 3);
        assert_eq!(hmap, hmap2);
        assert_eq!(hmap2.len(), 4);
    }

    #[test]
    #[cfg(feature = "headers")]
    fn test_convert_sample() {
        let headers = raw_headers_sample();
        let hmap = http::HeaderMap::from(headers.clone());
        let headers2 = Headers::from(hmap.clone());
        let hmap2 = http::HeaderMap::from(headers2.clone());
        assert_eq!(headers, headers2);
        assert_eq!(headers2.len(), 14);
        assert_eq!(hmap2, hmap);
        assert_eq!(hmap2.len(), 15);
    }

    #[test]
    #[cfg(feature = "headers")]
    fn test_convert_by_ref() {
        let headers = raw_headers_sample();
        let hmap = http::HeaderMap::from(&headers);
        let headers2 = Headers::from(&hmap);
        let hmap2 = http::HeaderMap::from(&headers2);
        assert_eq!(headers, headers2);
        assert_eq!(headers2.len(), 14);
        assert_eq!(hmap2, hmap);
        assert_eq!(hmap2.len(), 15);
    }

    #[test]
    fn test_value_parse() {
        let mut hmap = http::HeaderMap::new();
        hmap.insert(http::header::CONTENT_ENCODING,
                    "chunked, gzip".parse().unwrap());
        let val = hmap.get(http::header::CONTENT_ENCODING).unwrap();
        let ce = ContentEncoding::parse_header(&val).unwrap();
        assert_eq!(ce, ContentEncoding(vec![Encoding::Chunked, Encoding::Gzip]))
    }

    #[test]
    fn test_multi_value_parse() {
        let mut hmap = http::HeaderMap::new();
        hmap.insert(http::header::CONTENT_ENCODING,
                    "chunked, gzip".parse().unwrap());
        hmap.append(http::header::CONTENT_ENCODING,
                    "br".parse().unwrap());

        let vals = hmap.get_all(http::header::CONTENT_ENCODING);
        let ce = ContentEncoding::parse_header(&vals).unwrap();
        assert_eq!(
            ce,
            ContentEncoding(vec![
                Encoding::Chunked, Encoding::Gzip, Encoding::Brotli
            ])
        )
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_0_value_parse(b: &mut Bencher) {
        let mut hmap = http::HeaderMap::new();
        hmap.insert(http::header::CONTENT_ENCODING,
                    "chunked, gzip".parse().unwrap());
        b.iter(|| {
            let val = hmap.get(http::header::CONTENT_ENCODING).unwrap();
            ContentEncoding::parse_header(&val).unwrap();
        })
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_0_value_parse_extra_str(b: &mut Bencher) {
        use header::Raw;
        let mut hmap = http::HeaderMap::new();
        hmap.insert(http::header::CONTENT_ENCODING,
                    "chunked, gzip".parse().unwrap());
        b.iter(|| {
            let val = hmap.get(http::header::CONTENT_ENCODING).unwrap();
            let r: Raw = val.to_str().unwrap().into();
            ContentEncoding::parse_header(&r).unwrap();
        })
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_0_value_parse_int(b: &mut Bencher) {
        let mut hmap = http::HeaderMap::new();
        hmap.insert(http::header::CONTENT_LENGTH, "1024".parse().unwrap());
        b.iter(|| {
            let val = hmap.get(http::header::CONTENT_LENGTH).unwrap();
            ContentLength::parse_header(&val).unwrap();
        })
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_1_get_parse_int(b: &mut Bencher) {
        let mut hmap = http::HeaderMap::new();
        hmap.insert(http::header::CONTENT_LENGTH, "11".parse().unwrap());
        b.iter(|| {
            let vals = hmap.get_all(http::header::CONTENT_LENGTH);
            let len = ContentLength::parse_header(&vals).unwrap();
            assert_eq!(*len, 11);
        })
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_1_get_parse_int_one(b: &mut Bencher) {
        let mut hmap = http::HeaderMap::new();
        hmap.insert(http::header::CONTENT_LENGTH, "11".parse().unwrap());
        b.iter(|| {
            let val = hmap.get(http::header::CONTENT_LENGTH).unwrap();
            let len = ContentLength::parse_header(&val).unwrap();
            assert_eq!(*len, 11);
        })
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_2_decode_int(b: &mut Bencher) {
        let mut hmap = http::HeaderMap::new();
        hmap.insert(http::header::CONTENT_LENGTH, "11".parse().unwrap());
        b.iter(|| {
            let len: ContentLength = hmap.decode().unwrap();
            assert_eq!(*len, 11);
        })
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_2_try_decode_int(b: &mut Bencher) {
        let mut hmap = http::HeaderMap::new();
        hmap.insert(http::header::CONTENT_LENGTH, "11".parse().unwrap());
        b.iter(|| {
            let len: ContentLength = hmap.try_decode().unwrap().unwrap();
            assert_eq!(*len, 11);
        })
    }

    #[cfg(all(feature = "nightly", feature = "headers"))]
    #[bench]
    fn bench_3_get_orig_int(b: &mut Bencher) {
        let mut hdrs = ::header::Headers::new();
        hdrs.set_raw("content-length", "11");
        b.iter(|| {
            let len: &ContentLength = hdrs.get().unwrap();
            assert_eq!(**len, 11);
        })
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_4_encode_int(b: &mut Bencher) {
        b.iter(|| {
            let mut hmap = http::HeaderMap::new();
            hmap.encode(&ContentLength(11));
            assert_eq!(hmap.len(), 1);
        })
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn bench_4_encode_multi(b: &mut Bencher) {
        b.iter(|| {
            let mut hmap = http::HeaderMap::new();
            hmap.encode(
                &ContentEncoding(vec![Encoding::Identity]));
            hmap.encode_append(
                &ContentEncoding(vec![Encoding::Gzip, Encoding::Chunked]));
            hmap.encode(&ContentLength(11));
            hmap.encode(
                &ETag(EntityTag::strong("pMMV3zmCrXr-n4ZZLR9".to_owned())));
            assert_eq!(hmap.len(), 4);
        })
    }

    #[cfg(all(feature = "nightly", feature = "headers"))]
    #[bench]
    fn bench_5_map_from_headers(b: &mut Bencher) {
        let heads = raw_headers_sample();
        b.iter(|| {
            let hmap = http::HeaderMap::from(&heads);
            assert_eq!(hmap.len(), 15);
        })
    }

    #[cfg(all(feature = "nightly", feature = "headers"))]
    #[bench]
    fn bench_5_headers_from_map(b: &mut Bencher) {
        let heads = raw_headers_sample();
        let hmap: http::HeaderMap = heads.into();
        b.iter(|| {
            let heads = Headers::from(&hmap);
            assert_eq!(heads.len(), 14);
        })
    }

    #[cfg(all(feature = "nightly", feature = "headers"))]
    #[bench]
    fn bench_5_headers_from_map_by_value(b: &mut Bencher) {
        let heads = raw_headers_sample();
        let hmap: http::HeaderMap = heads.into();
        b.iter(|| {
            let heads = Headers::from(hmap.clone());
            assert_eq!(heads.len(), 14);
        })
    }
}
