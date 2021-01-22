#[derive(Debug, Clone)]
pub struct StdxXssSanitizer {}

impl StdxXssSanitizer {
    pub fn new() -> StdxXssSanitizer {
        StdxXssSanitizer {}
    }
}

impl super::XssSanitizer for StdxXssSanitizer {
    fn sanitize(&self, input: &str) -> String {
        stdx::html::sanitize_xss(input)
    }

    fn escape(&self, input: &str) -> String {
        stdx::html::escape(input)
    }
}
