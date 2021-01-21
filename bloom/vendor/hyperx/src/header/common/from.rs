header! {
    /// `From` header, defined in [RFC7231](http://tools.ietf.org/html/rfc7231#section-5.5.1)
    ///
    /// The `From` header field contains an Internet email address for a
    /// human user who controls the requesting user agent.  The address ought
    /// to be machine-usable.
    ///
    /// # ABNF
    ///
    /// ```text
    /// From    = mailbox
    /// mailbox = <mailbox, see [RFC5322], Section 3.4>
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate http;
    /// use hyperx::header::{From, TypedHeaders};
    ///
    /// let mut headers = http::HeaderMap::new();
    /// headers.encode(&From("webmaster@example.org".to_owned()));
    /// ```
    // FIXME: Maybe use mailbox?
    (From, "From") => [String]

    test_from {
        test_header!(test1, vec![b"webmaster@example.org"]);
    }
}

standard_header!(From, FROM);
