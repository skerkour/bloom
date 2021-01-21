use unicase::Ascii;

header! {
    /// `Vary` header, defined in [RFC7231](https://tools.ietf.org/html/rfc7231#section-7.1.4)
    ///
    /// The "Vary" header field in a response describes what parts of a
    /// request message, aside from the method, Host header field, and
    /// request target, might influence the origin server's process for
    /// selecting and representing this response.  The value consists of
    /// either a single asterisk ("*") or a list of header field names
    /// (case-insensitive).
    ///
    /// # ABNF
    ///
    /// ```text
    /// Vary = "*" / 1#field-name
    /// ```
    ///
    /// # Example values
    ///
    /// * `accept-encoding, accept-language`
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate http;
    /// use hyperx::header::{TypedHeaders, Vary};
    ///
    /// let mut headers = http::HeaderMap::new();
    /// headers.encode(&Vary::Any);
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate http;
    /// # extern crate hyperx;
    /// # extern crate unicase;
    /// # fn main() {
    /// // extern crate unicase;
    ///
    /// use hyperx::header::{TypedHeaders, Vary};
    /// use unicase::Ascii;
    ///
    /// let mut headers = http::HeaderMap::new();
    /// headers.encode(
    ///     &Vary::Items(vec![
    ///         Ascii::new("accept-encoding".to_owned()),
    ///         Ascii::new("accept-language".to_owned()),
    ///     ])
    /// );
    /// # }
    /// ```
    (Vary, "Vary") => {Any / (Ascii<String>)+}

    test_vary {
        test_header!(test1, vec![b"accept-encoding, accept-language"]);

        #[test]
        fn test2() {
            let mut vary: ::Result<Vary>;
            let r: Raw = "*".into();
            vary = Header::parse_header(&r);
            assert_eq!(vary.ok(), Some(Vary::Any));

            let r: Raw = "etag,cookie,allow".into();
            vary = Header::parse_header(&r);
            assert_eq!(
                vary.ok(),
                Some(Vary::Items(vec![
                    "eTag".parse().unwrap(),
                    "cookIE".parse().unwrap(),
                    "AlLOw".parse().unwrap(),
                ])));
        }
    }
}

standard_header!(Vary, VARY);
