header! {
    /// `Access-Control-Max-Age` header, part of
    /// [CORS](http://www.w3.org/TR/cors/#access-control-max-age-response-header)
    ///
    /// The `Access-Control-Max-Age` header indicates how long the results of a
    /// preflight request can be cached in a preflight result cache.
    ///
    /// # ABNF
    ///
    /// ```text
    /// Access-Control-Max-Age = \"Access-Control-Max-Age\" \":\" delta-seconds
    /// ```
    ///
    /// # Example values
    ///
    /// * `531`
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate http;
    /// use hyperx::header::{AccessControlMaxAge, TypedHeaders};
    ///
    /// let mut headers = http::HeaderMap::new();
    /// headers.encode(&AccessControlMaxAge(1728000u32));
    /// ```
    (AccessControlMaxAge, "Access-Control-Max-Age") => [u32]

    test_access_control_max_age {
        test_header!(test1, vec![b"531"]);
    }
}

standard_header!(AccessControlMaxAge, ACCESS_CONTROL_MAX_AGE);
