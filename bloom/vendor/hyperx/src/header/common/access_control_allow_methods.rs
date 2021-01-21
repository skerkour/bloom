use method::Method;

header! {
    /// `Access-Control-Allow-Methods` header, part of
    /// [CORS](http://www.w3.org/TR/cors/#access-control-allow-methods-response-header)
    ///
    /// The `Access-Control-Allow-Methods` header indicates, as part of the
    /// response to a preflight request, which methods can be used during the
    /// actual request.
    ///
    /// # ABNF
    ///
    /// ```text
    /// Access-Control-Allow-Methods: "Access-Control-Allow-Methods" ":" #Method
    /// ```
    ///
    /// # Example values
    /// * `PUT, DELETE, XMODIFY`
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate http;
    /// use hyperx::header::{AccessControlAllowMethods, TypedHeaders};
    /// use hyperx::Method;
    ///
    /// let mut headers = http::HeaderMap::new();
    /// headers.encode(
    ///     &AccessControlAllowMethods(vec![Method::Get])
    /// );
    /// ```
    ///
    /// ```
    /// # extern crate http;
    /// use hyperx::header::{AccessControlAllowMethods, TypedHeaders};
    /// use hyperx::Method;
    ///
    /// let mut headers = http::HeaderMap::new();
    /// headers.encode(
    ///     &AccessControlAllowMethods(vec![
    ///         Method::Get,
    ///         Method::Post,
    ///         Method::Patch,
    ///         Method::Extension("COPY".to_owned()),
    ///     ])
    /// );
    /// ```
    (AccessControlAllowMethods, "Access-Control-Allow-Methods") => (Method)*

    test_access_control_allow_methods {
        test_header!(test1, vec![b"PUT, DELETE, XMODIFY"]);
    }
}

standard_header!(AccessControlAllowMethods, ACCESS_CONTROL_ALLOW_METHODS);
